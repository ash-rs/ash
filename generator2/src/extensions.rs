use crate::{Context, Error};
use heck::SnakeCase;
use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::Path,
};

pub fn generate_extensions(ctx: &Context) -> Result<(), Error> {
    create_dir_all(ctx.output_path)?;
    let extesions_dir = ctx.output_path.join("extensions");
    create_dir_all(&extesions_dir)?;
    for (name, extension) in &ctx.extension_by_name {
        generate_extension(ctx, &extesions_dir, extension);
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
    let mut file = File::create(extension_path)?;

    for enums in extensions_types_iter(extension).filter_map(|name| ctx.enums_by_name.get(name)) {
        crate::enums::write_enum_definitions(ctx, enums, &mut file);
    }

    // if let Some(extend_enums) = ctx.extension_enums.get(extension.name.as_str()) {
    //     for &e in &extend_enums.enums {
    //         crate::enums::write_enum(ctx, e, &mut file)?;
    //     }
    // }

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
