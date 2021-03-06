use generator2::generate;
use generator2::template::*;
use std::collections::HashMap;

use std::fmt::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let name = "Foo";
    let fields = vec!["x: 32,", "y: 32", "z: 32"];
    generator2::source!(
        "
        pub struct #ident# {
        }
        impl #ident# {
            pub fn new() -> Self {
                Self {
                    #fields#
                }
            }
        }
        ",
        ident = name,
        fields = fields,
    );

    generate("", "../generator/Vulkan-Headers/registry/vk.xml")
}
