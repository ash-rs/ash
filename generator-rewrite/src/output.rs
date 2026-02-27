mod vfs;

use crate::{output::vfs::VirtualRustFs, util};
use analysis::item::RequiredBy;
use indexmap::IndexMap;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::{
    collections::{HashMap, HashSet},
    io, iter,
    path::{Path, PathBuf},
};
use syn::Ident;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Destination(pub RequiredBy);

struct DestinationPathComponent {
    module_name: Ident,
    doc_comment: String,
}

impl Destination {
    fn path_components(&self) -> Vec<DestinationPathComponent> {
        match self.0 {
            RequiredBy::Feature { major, minor } => vec![DestinationPathComponent {
                module_name: format_ident!("vk{major}_{minor}"),
                doc_comment: format!("Vulkan version {major}.{minor}"),
            }],
            RequiredBy::Extension { name } => {
                if let Some(vulkan_ext) = name.strip_prefix("VK_") {
                    let (ext_tag, ext_name) = vulkan_ext.split_once('_').unwrap();
                    vec![
                        DestinationPathComponent {
                            module_name: format_ident!("{}", ext_tag.to_ascii_lowercase()),
                            doc_comment: format!("Extensions tagged {ext_tag}"),
                        },
                        DestinationPathComponent {
                            module_name: util::snake_case_escape_ident(ext_name),
                            doc_comment: util::refpage_doc(name, "Vulkan extension"),
                        },
                    ]
                } else if let Some(video_ext) = name.strip_prefix("vulkan_video_") {
                    vec![
                        DestinationPathComponent {
                            module_name: format_ident!("video"),
                            doc_comment: "Vulkan Video".into(),
                        },
                        DestinationPathComponent {
                            module_name: format_ident!("{video_ext}"),
                            doc_comment: format!("Items provided by `{}`", name),
                        },
                    ]
                } else {
                    panic!()
                }
            }
        }
    }
}

#[derive(Default)]
pub struct CodeMap(IndexMap<Destination, TokenStream>);

impl CodeMap {
    pub fn new(destination: Destination, tokens: TokenStream) -> CodeMap {
        let mut map = IndexMap::with_capacity(1);
        map.insert(destination, tokens);
        CodeMap(map)
    }

    pub fn extend(&mut self, other: CodeMap) {
        for (destination, tokens) in other.0 {
            self.0.entry(destination).or_default().extend(tokens);
        }
    }

    pub fn write(&self, output_path: impl AsRef<Path>) -> io::Result<()> {
        let mut vfs = VirtualRustFs::default();
        vfs.write(
            "mod.rs",
            quote! {
                /// A re-export of all items
                pub mod vk;
            },
        );

        // foo/bar/mod.rs -> (doc comment, child modules)
        let mut mod_files: HashMap<PathBuf, (Option<String>, HashSet<Ident>)> = Default::default();
        for (destination, content) in &self.0 {
            let components = destination.path_components();

            let doc = &components.last().unwrap().doc_comment;
            let mut path = PathBuf::from_iter(
                (components.iter())
                    .map(|component| PathBuf::from(component.module_name.to_string())),
            );
            path.add_extension("rs");

            vfs.write(
                path,
                quote! {
                    #![doc = #doc]
                    #content
                },
            );

            let component_idents = components.iter().map(|component| &component.module_name);
            vfs.write(
                "vk.rs",
                quote! { pub use super:: #( #component_idents ) :: * ::*; },
            );

            // collect mod.rs files to be created
            for (i, component) in components.iter().enumerate() {
                let parent_path = &components[0..i];
                let mod_path = PathBuf::from_iter(
                    (parent_path.iter())
                        .map(|component| PathBuf::from(&component.module_name.to_string()))
                        .chain(iter::once(PathBuf::from("mod.rs"))),
                );

                let (_, mod_child_idents) = mod_files.entry(mod_path).or_insert_with(|| {
                    let doc = (parent_path.last()).map(|component| component.doc_comment.clone());
                    (doc, HashSet::new())
                });

                mod_child_idents.insert(component.module_name.clone());
            }
        }

        for (mod_path, (doc, mod_child_idents)) in mod_files {
            let mut mod_child_idents: Vec<Ident> = mod_child_idents.into_iter().collect();
            mod_child_idents.sort_unstable();

            let doc = doc.map(|doc| quote! { #![doc = #doc] });
            vfs.write(
                mod_path,
                quote! {
                    #doc
                    #(pub mod #mod_child_idents;)*
                },
            );
        }

        vfs.sync_to(output_path)
    }
}
