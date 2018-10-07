extern crate generator;

use generator::write_source_code;
use std::{env, path::Path};

fn main() {
    let xml = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).parent().unwrap().join("generator").join("Vulkan-Headers").join("registry").join("vk.xml");
    println!("cargo:rerun-if-changed={}", xml.display());
    write_source_code(&xml,
                      &Path::new(&env::var("OUT_DIR").unwrap()).join("vk.rs"));
}
