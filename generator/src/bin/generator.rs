#[macro_use]
extern crate generator;
use generator::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    //let file = File::open("New-Vulkan-XML-Format/vk_new.xml").expect("vknew");
    let spec = vk_parse::parse_file_as_vkxml(Path::new("vk.xml"));
    write_source_code(&spec);
}
