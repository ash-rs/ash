use crate::{Context, Error};
use heck::SnakeCase;
use std::{
    collections::HashMap,
    fs::{create_dir_all, File},
    io::Write,
    path::Path,
};

pub fn generate_extensions(ctx: &Context) -> Result<(), Error> {
    create_dir_all(ctx.output_path)?;
    let extesions_dir = ctx.output_path.join("extensions");
    create_dir_all(&extesions_dir)?;
    for (name, extension) in &ctx.extension_by_name {
        generate_extension(ctx, &extesions_dir, extension)?;
    }
    Ok(())
}

pub fn generate_extension(
    ctx: &Context,
    dir: &Path,
    extension: &vk::Extension,
) -> Result<(), Error> {
    let extension_name_path = extension.name.to_snake_case();
    let extension_path = dir.join(extension_name_path).with_extension("rs");
    let mut w = File::create(extension_path)?;

    for enums in extensions_types_iter(extension).filter_map(|name| ctx.enums_by_name.get(name)) {
        crate::enums::write_enum_definitions(ctx, enums, &mut w)?;
    }

    writeln!(w, "// Extension enums")?;

    let mut map: HashMap<&str, Vec<&vk::Enum>> = HashMap::new();
    for e in extensions_enum_iter(extension) {
        if let Some(extends) = crate::get_extends_from_enum(e) {
            map.entry(extends).or_default().push(e);
        }
    }

    for (extends, enums) in map {
        let name = ctx.rust_type_name(extends);
        writeln!(w, "impl {} {{", name)?;
        for e in enums {
            crate::enums::write_enum_variant(ctx, extends, e, &mut w)?;
        }
        writeln!(w, "}}",)?;
    }

    for ty in extensions_types_iter(extension).filter_map(|name| ctx.type_by_name.get(name)) {
        if ty.contains_bitfields() {
            let code = match ty.name {
                "VkAccelerationStructureInstanceKHR" => {
                    include_str!("../src/templates/vk_acceleration_structure_instance_khr.rs")
                }
                name => {
                    unimplemented!(
                        "{name} requires a manual implementation, please file an issue",
                        name = name
                    )
                }
            };
            writeln!(w, "{}", code)?;
        } else {
            crate::structs::write_type(ctx, ty, &mut w)?;
            crate::structs::derive_debug2(ctx, ty, &mut w)?;
            crate::structs::derive_default2(ctx, ty, &mut w)?;
        }
    }

    Ok(())
}

// Helper function to access all the types in the extension
fn extensions_types_iter<'spec>(
    extension: &'spec vk::Extension,
) -> impl Iterator<Item = &'spec str> + 'spec {
    extension
        .children
        .iter()
        .filter_map(|child| {
            if let vk::ExtensionChild::Require { items, .. } = child {
                let iter = items.iter().filter_map(|item| {
                    if let vk::InterfaceItem::Type { name, .. } = item {
                        Some(name.as_str())
                    } else {
                        None
                    }
                });
                Some(iter)
            } else {
                None
            }
        })
        .flatten()
}

// Helper function to access all the enums in the extension
fn extensions_enum_iter<'spec>(
    extension: &'spec vk::Extension,
) -> impl Iterator<Item = &'spec vk::Enum> + 'spec {
    extension
        .children
        .iter()
        .filter_map(|child| {
            if let vk::ExtensionChild::Require { items, .. } = child {
                let iter = items.iter().filter_map(|item| {
                    if let vk::InterfaceItem::Enum(e) = item {
                        Some(e)
                    } else {
                        None
                    }
                });
                Some(iter)
            } else {
                None
            }
        })
        .flatten()
}

// Helper function to access all the enums in the extension
fn extensions_command_iter<'spec>(
    extension: &'spec vk::Extension,
) -> impl Iterator<Item = (&'spec str, Option<&'spec str>)> + 'spec {
    extension
        .children
        .iter()
        .filter_map(|child| {
            if let vk::ExtensionChild::Require { items, .. } = child {
                let iter = items.iter().filter_map(|item| {
                    if let vk::InterfaceItem::Command { name, comment } = item {
                        Some((name.as_str(), comment.as_ref().map(String::as_str)))
                    } else {
                        None
                    }
                });
                Some(iter)
            } else {
                None
            }
        })
        .flatten()
}
