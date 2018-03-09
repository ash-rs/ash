// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
extern crate heck;
#[macro_use]
extern crate quote;
extern crate syn;
pub extern crate vkxml;

use heck::SnakeCase;

use syn::Ident;

pub trait CommandExt {
    /// Returns the ident in snake_case and without the 'vk' prefix.
    fn is_device_command(&self) -> bool;
    ///
    /// Returns true if the command is a device level command. This is indicated by
    /// the type of the first parameter.
    fn command_ident(&self) -> Ident;
}

impl CommandExt for vkxml::Command {
    fn command_ident(&self) -> Ident {
        Ident::from(self.name[2..].to_snake_case().as_str())
    }

    fn is_device_command(&self) -> bool {
        self.param
            .iter()
            .nth(0)
            .map(|field| match field.basetype.as_str() {
                "VkDevice" | "VkCommandBuffer" | "VkQueue" => true,
                _ => false,
            })
            .unwrap_or(false)
    }
}

pub trait FieldExt {
    /// Returns the name of the paramter that doesn't clash with Rusts resevered
    /// keywords
    fn param_ident(&self) -> Ident;

    /// Returns the basetype ident and removes the 'Vk' prefix
    fn type_ident(&self) -> Ident;
}

impl FieldExt for vkxml::Field {
    fn param_ident(&self) -> Ident {
        let name = self.name.as_ref().map(|s| s.as_str()).unwrap_or("field");
        let name_corrected = match name {
            "type" => "ty",
            _ => name,
        };
        Ident::from(name_corrected.to_snake_case().as_str())
    }

    fn type_ident(&self) -> Ident {
        let prefix = &self.basetype[0..2];
        if prefix == "Vk" {
            Ident::from(&self.basetype[2..])
        } else {
            Ident::from(&self.basetype[..])
        }
    }
}
use std::collections::HashMap;
pub type CommandMap<'a> = HashMap<vkxml::Identifier, &'a vkxml::Command>;
pub fn gen_load(feature: &vkxml::Feature, commands: &CommandMap) -> quote::Tokens {
    let (device_commands, instance_commands) = feature
        .elements
        .iter()
        .flat_map(|feature| {
            if let &vkxml::FeatureElement::Require(ref spec) = feature {
                spec.elements
                    .iter()
                    .filter_map(|feature_spec| {
                        if let &vkxml::FeatureReference::CommandReference(ref cmd_ref) =
                            feature_spec
                        {
                            Some(cmd_ref)
                        } else {
                            None
                        }
                    })
                    .collect()
            } else {
                vec![]
            }
        })
        .filter_map(|cmd_ref| commands.get(&cmd_ref.name))
        .fold((Vec::new(), Vec::new()), |mut acc, &cmd_ref| {
            let basetype = cmd_ref.param[0].basetype.as_str();
            match basetype {
                "VkDevice" | "VkCommandBuffer" | "VkQueue" => acc.0.push(cmd_ref),
                _ => acc.1.push(cmd_ref),
            };
            acc
        });
    let name = Ident::from("Test");
    let function_pointers: Vec<_> = instance_commands
        .iter()
        .map(|cmd| {
            let fn_name_raw = cmd.name.as_str();
            let fn_name_snake = cmd.command_ident();
            let params: Vec<_> = cmd.param
                .iter()
                .map(|field| {
                    let name = field.param_ident();
                    let ty = field.type_ident();
                    quote!{#name: vk::#ty}
                })
                .collect();
            let return_ty = cmd.return_type.type_ident();
            quote!{
                #fn_name_snake: extern "system" fn(#(#params,)*) -> vk::#return_ty
            }
        })
        .collect();
    quote!{
        pub struct #name {
            #(#function_pointers,)*
        }
    }
}
