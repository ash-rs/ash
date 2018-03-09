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

pub trait FeatureExt {
    fn version_string(&self) -> String;
}
impl FeatureExt for vkxml::Feature {
    fn version_string(&self) -> String {
        let version = format!("{:.10}", self.version);
        let first_0 = version.find("0").expect("should have at least one 0");
        // + 1 is correct here because we always have 10 zeroes.
        let (version, _) = version.split_at(first_0 + 1);
        version.replace(".", "_")
    }
}
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
        let new_name = match self.basetype.as_str() {
            "void" => "c_void",
            "char" => "c_char",
            "float" => "c_float",
            "long" => "c_ulong",
            _ => {
                let prefix = &self.basetype[0..2];
                if prefix == "Vk" {
                    &self.basetype[2..]
                } else {
                    self.basetype.as_str()
                }
            }
        };
        Ident::from(new_name)
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
            if cmd_ref.is_device_command() {
                acc.0.push(cmd_ref);
            } else {
                acc.1.push(cmd_ref);
            };
            acc
        });
    let name = Ident::from("Test");
    fn generate_function_pointers(ident: Ident, commands: &[&vkxml::Command]) -> quote::Tokens {
        let function_pointers: Vec<_> = commands
            .iter()
            .map(|cmd| {
                let fn_name_raw = cmd.name.as_str();
                let fn_name_snake = cmd.command_ident();
                let params: Vec<_> = cmd.param
                    .iter()
                    .map(|field| {
                        let name = field.param_ident();
                        let ty = field.type_ident();
                        quote!{#name: #ty}
                    })
                    .collect();
                let return_ty = cmd.return_type.type_ident();
                quote!{
                    #fn_name_snake: extern "system" fn(#(#params,)*) -> #return_ty
                }
            })
            .collect();
        let names: Vec<_> = commands.iter().map(|cmd| cmd.command_ident()).collect();
        let names_left = &names;
        let names_right = &names;
        quote!{
            pub struct #ident {
                #(#function_pointers,)*
            }
            impl ::std::clone::Clone for #ident {
                pub fn clone(&self) -> Self {
                    #ident{
                        #(#names_left: self.#names_right,)*
                    }
                }
            }
        }
    }
    let version = feature.version_string();
    let instance = generate_function_pointers(
        Ident::from(format!("InstanceFnV{}", version).as_str()),
        &instance_commands,
    );
    let device = generate_function_pointers(
        Ident::from(format!("DeviceFnV{}", version).as_str()),
        &instance_commands,
    );
    quote! {
        #instance

        #device
    }
}
