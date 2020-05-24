use crate::{EnumExt, EnumKind, constants, template, Context, Error};
use heck::ShoutySnakeCase;
use std::io::Write;

pub fn write_enums(ctx: &Context<'_>, w: &mut dyn Write) -> Result<(), Error> {
    for (vk_name, enums) in &ctx.enums_by_name {

        let kind = enums.enum_kind();
        let rust_name = match kind {
            EnumKind::Bitflag => ctx.rust_bitflag_type_name(vk_name),
            EnumKind::Enum => ctx.rust_type_name(vk_name).to_string(),
            _ => continue
        };

        write_enum_header(&rust_name, w)?;
        writeln!(w, "impl {} {{", rust_name);
        for child in &enums.children {
            if let vk::EnumsChild::Enum(e) = &child {
                write_enum_variant(ctx, vk_name, e, w);
            }
        }
        writeln!(w, "}}");

        if let Some(extend_enums) = ctx.extension_enums.get(vk_name) {
            writeln!(w, "    // {}", extend_enums.extension);
            for e in &extend_enums.enums {
                write_enum_variant(ctx, vk_name, e, w);
            }
        }
    }
    Ok(())
}

fn write_enum_impl(enum_name: &str, e: &vk::Enum, w: &mut dyn Write) {}
fn write_enum_variant(ctx: &Context<'_>, enum_name: &str, e: &vk::Enum, w: &mut dyn Write) {
    let name = ctx.rust_enum_variant_name(enum_name, &e.name);
    let value = enum_variant_value(ctx, e);
    println!("{:#?}", e.spec);
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

fn write_enum_header(name: &str, w: &mut dyn Write) -> Result<(), Error> {
    let header = template::render(
        include_str!("templates/enum.rs.template"),
        &[("name", name)],
    );
    writeln!(w, "{}", header)?;
    Ok(())
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
            // let extnumber = extnumber.unwrap_or_else(|| extension_number);
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
