use crate::{
    parse::{c_parse_variable_decl, CDecoration, CType, CVariableDecl},
    Context, Error, Ident,
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
    match ty.category.as_ref().map(String::as_str).unwrap_or("") {
        "struct" => {
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
#[derive(Debug)]
pub struct Type<'spec> {
    pub name: &'spec str,
    pub fields: Vec<Field<'spec>>,
}

impl Type<'_> {
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

pub fn write_type<'spec>(
    ctx: &Context,
    ty: &'spec Type<'spec>,
    w: &mut dyn Write,
) -> Result<(), Error> {
    writeln!(w, "pub struct {} {{", ctx.rust_type_name(ty.name))?;

    for field in &ty.fields {
        let name = ctx.rust_field_name(field.variable.name);

        let pointer = match field.variable.ty.decoration {
            CDecoration::Pointer => "*mut",
            CDecoration::PointerToConst => "*const",
            CDecoration::PointerToPointer => "*mut *mut",
            CDecoration::PointerToConstPointerToConst => "*const *const",
            CDecoration::None => "",
        };

        let rust_ty = ctx.rust_type_name(field.variable.ty.name);

        let rust_ty = match (field.variable.ty.array_size, field.variable.ty.array_size2) {
            (Some(n), None) => format!("[{}; {}]", rust_ty, n),
            (Some(n), Some(m)) => format!("[[{}; {}]; {}]", rust_ty, n, m),
            _ => rust_ty.to_owned(),
        };

        writeln!(
            w,
            "    {name}: {pointer} {rust_ty}, ",
            name = name,
            pointer = pointer,
            rust_ty = rust_ty,
        )?;
    }
    writeln!(w, "}}")?;

    Ok(())
}

pub fn derive_debug(ctx: &Context, ty: &Type<'_>, w: &mut dyn Write) -> Result<(), Error> {
    let name = ctx.rust_type_name(ty.name);

    writeln!(w, "impl std::fmt::Debug for {} {{", name)?;
    writeln!(
        w,
        "{ident}fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {{",
        ident = Ident(1),
    )?;
    writeln!(w, "{ident}fmt.debug_struct(\"{}\")", name, ident = Ident(2))?;

    for field in &ty.fields {
        let field_name = ctx.rust_field_name(field.variable.name);
        writeln!(
            w,
            "{ident}.field(\"{name}\", &self.{name})",
            name = field_name,
            ident = Ident(3),
        )?;
    }
    writeln!(w, "{ident}.finish()", ident = Ident(3))?;
    writeln!(w, "{ident}}}", ident = Ident(1))?;
    writeln!(w, "}}")?;
    Ok(())
}

pub fn derive_default(ctx: &Context, ty: &Type<'_>, w: &mut dyn Write) -> Result<(), Error> {
    let name = ctx.rust_type_name(ty.name);
    writeln!(w, "impl std::default::Default for {} {{", name)?;
    writeln!(
        w,
        "{ident}fn default() -> {} {{",
        name,
        ident = Ident(1),
    )?;
    writeln!(w, "{ident}{} {{", name, ident = Ident(2))?;

    for field in &ty.fields {
        let field_name = ctx.rust_field_name(field.variable.name);
        writeln!(
            w,
            "{ident}{name}: {ty}::default(),",
            name = field_name,
            ident = Ident(3),
            ty = ctx.rust_type_name(field.variable.ty.name),
        )?;
    }
    writeln!(w, "{ident}}}", ident = Ident(2))?;
    writeln!(w, "{ident}}}", ident = Ident(1))?;
    writeln!(w, "}}")?;

    Ok(())
}
