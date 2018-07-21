#[macro_use]
extern crate generator;
use generator::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    //let file = File::open("New-Vulkan-XML-Format/vk_new.xml").expect("vknew");
    write_source_code(Path::new("vk.xml"));
}
