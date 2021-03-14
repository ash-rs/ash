use crate::{
    parse::{c_parse_variable_decl, CDecoration, CType, CVariableDecl},
    Context, Error,
};
use std::io::Write;
pub fn type_member_iter<'spec>(
    ty: &'spec vk::Type,
) -> impl Iterator<Item = &'spec vk::TypeMemberDefinition> + 'spec {
    std::iter::once(&ty.spec)
        .filter_map(|spec| {
            if let vk::TypeSpec::Members(members) = spec {
                Some(members)
            } else {
                None
            }
        })
        .flatten()
        .filter_map(|member| {
            if let vk::TypeMember::Definition(def) = member {
                Some(def)
            } else {
                None
            }
        })
}

pub fn to_ty<'spec>(ty: &'spec vk::Type) -> Option<TypeDefinition<'spec>> {
    let category = ty.category.as_ref().map(String::as_str).unwrap_or("");
    match category {
        "struct" | "union" => {
            let category = Category::from_str(category).unwrap();
            let name = ty.name.as_ref().expect("name");
            let fields = type_member_iter(ty)
                .map(|member| Field {
                    default_value: member.values.as_ref().map(String::as_str),
                    variable: c_parse_variable_decl(&member.code),
                })
                .collect();
            let ty = Type {
                name: name.as_str(),
                fields,
                category,
            };
            Some(TypeDefinition::Type(ty))
        }
        "alias" => {
            let name = ty.name.as_ref().expect("name");
            let alias = ty.alias.as_ref().expect("alias");
            Some(TypeDefinition::Alias(alias.as_str(), name.as_str()))
        }
        "define" => Some(TypeDefinition::Define),
        _ => None,
    }
}

pub enum TypeDefinition<'spec> {
    Type(Type<'spec>),
    Alias(&'spec str, &'spec str),
    Define,
}

#[derive(Debug, Copy, Clone)]
pub enum Category {
    Union,
    Struct,
}

impl Category {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "struct" => Some(Self::Struct),
            "union" => Some(Self::Union),
            _ => None,
        }
    }
}
#[derive(Debug)]
pub struct Type<'spec> {
    pub name: &'spec str,
    pub fields: Vec<Field<'spec>>,
    pub category: Category,
}

impl Type<'_> {
    pub fn is_union(&self) -> bool {
        matches!(self.category, Category::Union)
    }
    /// Returns true if this type contains bitfields. We need this because Rust doesn't support
    /// bitfields in ffi and we need to generate those types by hand
    pub fn contains_bitfields(&self) -> bool {
        self.fields
            .iter()
            .any(|field| field.variable.ty.bitfield.is_some())
    }
}

#[derive(Debug)]
pub struct Field<'spec> {
    pub default_value: Option<&'spec str>,
    pub variable: CVariableDecl<'spec>,
}

impl<'spec> Field<'spec> {
    pub fn default_value(&self, ctx: &Context) -> String {
        self.default_value.map_or_else(
            || {
                match self.variable.ty.decoration {
                    CDecoration::None => "Default::default()",
                    CDecoration::PointerToConst => "::std::ptr::null()",
                    _ => "::std::ptr::null_mut()",
                }
                .into()
            },
            |default_value_str| match self.variable.ty.name {
                "VkStructureType" => {
                    format!(
                        "StructureType::{}",
                        ctx.rust_enum_variant_name("VkStructureType", default_value_str)
                    )
                }
                value => unimplemented!("Please add support for the following value {}", value),
            },
        )
    }
}

pub fn write_type<'spec>(
    ctx: &Context,
    ty: &'spec Type<'spec>,
    w: &mut dyn Write,
) -> Result<(), Error> {
    let keyword = match ty.category {
        Category::Struct => "struct",
        Category::Union => "union",
    };

    let fields: Vec<_> = ty
        .fields
        .iter()
        .map(|field| {
            let name = ctx.rust_field_name(field.variable.name);

            let pointer = match field.variable.ty.decoration {
                CDecoration::Pointer => "*mut ",
                CDecoration::PointerToConst => "*const ",
                CDecoration::PointerToPointer => "*mut *mut ",
                CDecoration::PointerToConstPointerToConst => "*const *const ",
                CDecoration::None => "",
            };

            let rust_ty = ctx.rust_type_name(field.variable.ty.name);

            let rust_ty = match (field.variable.ty.array_size, field.variable.ty.array_size2) {
                (Some(n), None) => format!("[{}; {}]", rust_ty, n),
                (Some(n), Some(m)) => format!("[[{}; {}]; {}]", rust_ty, n, m),
                _ => rust_ty.to_owned(),
            };

            format!(
                "pub {name}: {pointer}{rust_ty}, ",
                name = name,
                pointer = pointer,
                rust_ty = rust_ty,
            )
        })
        .collect();

    let name = ctx.rust_type_name(ty.name);

    let source = crate::source!(
        "
        pub #keyword# #name# {
            #fields#
        }
        ",
        keyword = keyword,
        name = name,
        fields = fields,
    );

    write!(w, "{}", source);

    Ok(())
}

pub fn derive_debug(ctx: &Context, ty: &Type<'_>, w: &mut dyn Write) -> Result<(), Error> {
    if ty.is_union() {
        return Ok(());
    }

    let name = ctx.rust_type_name(ty.name);
    let fields: Vec<String> = ty
        .fields
        .iter()
        .map(|field| {
            let field_name = ctx.rust_field_name(field.variable.name);
            format!(".field(\"{name}\", &self.{name})", name = field_name,)
        })
        .collect();

    let code = crate::source! {
        "
        impl std::debug::Debug for #name# {{
            fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result
                fmt.debug_struct(\"#name#\")
                    #fields#
                }
            }
        }
        ",
        name = name,
        fields = fields,
    };

    writeln!(w, "{}", code)?;
    Ok(())
}

pub fn derive_default(ctx: &Context, ty: &Type<'_>, w: &mut dyn Write) -> Result<(), Error> {
    // Can't derive default for unions
    if ty.is_union() {
        return Ok(());
    }

    let name = ctx.rust_type_name(ty.name);
    let fields: Vec<String> = ty
        .fields
        .iter()
        .map(|field| {
            let field_name = ctx.rust_field_name(field.variable.name);
            format!(
                "{name}: {default},",
                name = field_name,
                default = field.default_value(ctx),
            )
        })
        .collect();

    let code = crate::source! {
        "
        impl std::default::Default for #name# {
            fn default() -> #name# {
                Self {
                    #fields#
                }
            }
        }
        ",
        name = name,
        fields = fields,
    };

    writeln!(w, "{}", code)?;
    Ok(())
}
