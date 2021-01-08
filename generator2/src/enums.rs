use crate::{constants, template, Context, EnumExt, EnumKind, Error};
use heck::ShoutySnakeCase;
use std::io::Write;

pub fn write_enum(ctx: &Context<'_>, _enum: &vk::Enum, w: &mut dyn Write) -> Result<(), Error> {
    // let vk_name = &_enum.name;

    // let kind = ctx
    //     .enums_by_name
    //     .get(vk_name.as_str())
    //     .expect("enum kind")
    //     .enum_kind();

    // let rust_name = match kind {
    //     EnumKind::Bitflag => ctx.rust_bitflag_type_name(vk_name),
    //     EnumKind::Enum => ctx.rust_type_name(vk_name).to_string(),
    //     _ => unimplemented!(),
    // };

    // write_enum_header(&rust_name, w)?;
    // writeln!(w, "impl {} {{", rust_name);
    // for child in &enums.children {
    //     if let vk::EnumsChild::Enum(e) = &child {
    //         write_enum_variant(ctx, vk_name, e, w);
    //     }
    // }

    Ok(())
}
pub fn write_enums(ctx: &Context<'_>, enums: &vk::Enums, w: &mut dyn Write) -> Result<(), Error> {
    if let Some(name) = enums.name.as_ref() {
        let vk_name = name.as_str();
        let kind = enums.enum_kind();
        let rust_name = match kind {
            EnumKind::Bitflag => ctx.rust_bitflag_type_name(vk_name),
            EnumKind::Enum => ctx.rust_type_name(vk_name).to_string(),
            _ => unimplemented!(),
        };

        let header = enum_template(&rust_name, kind);
        writeln!(w, "{}", header);
        writeln!(w, "impl {} {{", rust_name);
        for child in &enums.children {
            if let vk::EnumsChild::Enum(e) = &child {
                write_enum_variant(ctx, vk_name, e, w);
            }
        }

        // if let Some(extend_enums) = ctx.extension_enums.get(vk_name) {
        //     writeln!(w, "    // {}", extend_enums.extension);
        //     for &e in &extend_enums.enums {
        //         write_enum_variant(ctx, vk_name, e, w);
        //     }
        // }
        writeln!(w, "}}");
    }
    Ok(())
}

fn write_enum_variant(ctx: &Context<'_>, enum_name: &str, e: &vk::Enum, w: &mut dyn Write) {
    let name = ctx.rust_enum_variant_name(enum_name, &e.name);
    let value = enum_variant_value(ctx, e);
    match value {
        VariantValue::Alias(alias) => {
            let rust_alias = ctx.rust_enum_variant_name(enum_name, &alias);
            writeln!(
                w,
                "    pub const {name}: Self = Self::{alias};",
                name = name,
                alias = rust_alias
            );
        }
        VariantValue::Value(value) => {
            writeln!(
                w,
                "    pub const {name}: Self = Self({value});",
                name = name,
                value = value
            );
        }
    };
}

pub fn enum_variant_value<'spec>(ctx: &Context<'spec>, e: &'spec vk::Enum) -> VariantValue<'spec> {
    match &e.spec {
        vk::EnumSpec::Value { value, .. } => {
            VariantValue::Value(crate::parse::c_parse_int(value) as _)
        }
        vk::EnumSpec::Bitpos { bitpos, .. } => VariantValue::Value(1 << bitpos),
        vk::EnumSpec::Alias { alias, .. } => VariantValue::Alias(alias),
        vk::EnumSpec::Offset {
            offset,
            extends,
            extnumber,
            dir: positive,
        } => {
            let ext_base = 1_000_000_000;
            let ext_block_size = 1000;
            // TODO let extnumber = extnumber.unwrap_or_else(|| extension_number);
            let extnumber = extnumber.unwrap_or(1);
            let value = ext_base + (extnumber - 1) * ext_block_size + offset;
            let value = if *positive { value } else { -value };
            VariantValue::Value(value)
        }
        _ => unimplemented!(),
    }
}

pub enum VariantValue<'spec> {
    Value(i64),
    Alias(&'spec str),
}

fn enum_template(name: &str, kind: EnumKind) -> String {
    let ty = match kind {
        EnumKind::Bitflag => "u32",
        EnumKind::Enum => "i32",
        EnumKind::Constant => "i32",
    };

    let mut code = crate::source_code!(
        "
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
        #[repr(transparent)]
        pub struct {name}(pub(crate) {ty});
        impl {name} {{
            pub const fn from_raw(x: {ty}) -> Self {{
                {name}(x)
            }}
            pub const fn as_raw(self) -> {ty} {{
                self.0
            }}
        }}
        ",
        name = name,
        ty = ty,
    );

    if let EnumKind::Bitflag = kind {
        let bitflags_impl = format!("vk_bitflags_wrapped!({name}, 0b1111, Flags)", name = name);
        code.push_str(&bitflags_impl);
    }

    code


}
