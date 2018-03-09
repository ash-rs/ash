extern crate generator;
use std::fs::File;
use generator::*;
use std::collections::HashMap;
use std::io::Write;

fn main() {
    let file = File::open("vk_new.xml").expect("vk");
    let spec = vkxml::Registry::from_file(file).expect("");

    let commands: HashMap<vkxml::Identifier, &vkxml::Command> = spec.elements
        .iter()
        .filter_map(|elem| match elem {
            &vkxml::RegistryElement::Commands(ref cmds) => Some(cmds),
            _ => None,
        })
        .flat_map(|cmds| cmds.elements.iter().map(|cmd| (cmd.name.clone(), cmd)))
        .collect();

    let features: Vec<&vkxml::Feature> = spec.elements
        .iter()
        .filter_map(|elem| match elem {
            &vkxml::RegistryElement::Features(ref features) => Some(features),
            _ => None,
        })
        .flat_map(|features| features.elements.iter().map(|feature| feature))
        .collect();

    let source_code: Vec<_> = features.iter().map(|feature| gen_load(feature, &commands)).collect();
    let mut file = File::create("vk_test.rs").expect("vk");
    for source_code in &source_code {
        write!(&mut file, "{}", source_code);
    }
    

}
