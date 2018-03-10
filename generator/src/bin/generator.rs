#[macro_use]
extern crate generator;
use std::fs::File;
use generator::*;
use std::collections::HashMap;
use std::io::Write;

fn main() {
    let file = File::open("New-Vulkan-XML-Format/vk_new.xml").expect("vknew");
    let spec = vkxml::Registry::from_file(file).expect("");
    write_source_code(&spec);

}
