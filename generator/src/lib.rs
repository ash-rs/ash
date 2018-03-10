#![recursion_limit = "256"]
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
extern crate heck;
#[macro_use]
extern crate quote;
extern crate syn;
pub extern crate vkxml;

use quote::Tokens;
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
    fn type_tokens(&self) -> Tokens;
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

    fn type_tokens(&self) -> Tokens {
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
        let ptr_name = self.reference
            .as_ref()
            .map(|reference| match *reference {
                vkxml::ReferenceType::Pointer => "*mut",
                vkxml::ReferenceType::PointerToPointer => "*mut",
                vkxml::ReferenceType::PointerToConstPointer => "*const",
            })
            .unwrap_or("");
        let ty: syn::Type =
            syn::parse_str(&format!("{} {}", ptr_name, new_name)).expect("parse field");
        quote!{#ty}
    }
}
use std::collections::HashMap;
pub type CommandMap<'a> = HashMap<vkxml::Identifier, &'a vkxml::Command>;

fn generate_function_pointers(ident: Ident, commands: &[&vkxml::Command]) -> quote::Tokens {
    let names: Vec<_> = commands.iter().map(|cmd| cmd.command_ident()).collect();
    let names_ref = &names;
    let raw_names: Vec<_> = commands
        .iter()
        .map(|cmd| Ident::from(cmd.name.as_str()))
        .collect();
    let raw_names_ref = &raw_names;
    let names_left = &names;
    let names_right = &names;

    let params: Vec<Vec<(Ident, Tokens)>> = commands
        .iter()
        .map(|cmd| {
            let fn_name_raw = cmd.name.as_str();
            let fn_name_snake = cmd.command_ident();
            let params: Vec<_> = cmd.param
                .iter()
                .map(|field| {
                    let name = field.param_ident();
                    let ty = field.type_tokens();
                    (name, ty)
                })
                .collect();
            params
        })
        .collect();

    let params_names: Vec<Vec<_>> = params
        .iter()
        .map(|inner_params| {
            inner_params
                .iter()
                .map(|&(param_name, _)| param_name)
                .collect()
        })
        .collect();
    let param_names_ref = &params_names;
    let expanded_params: Vec<_> = params
        .iter()
        .map(|inner_params| {
            let inner_params_iter = inner_params.iter().map(|&(ref param_name, ref param_ty)| {
                quote!{#param_name: #param_ty}
            });
            quote!{
                #(#inner_params_iter,)*
            }
        })
        .collect();
    let expanded_params_ref = &expanded_params;

    let params_ref = &params;
    let return_types: Vec<_> = commands
        .iter()
        .map(|cmd| cmd.return_type.type_tokens())
        .collect();
    let return_types_ref = &return_types;
    quote!{
        pub struct #ident {
            #(
                #names_ref: extern "system" fn(#expanded_params_ref) -> #return_types_ref,
            )*
        }

        unsafe impl Send for #ident {}
        unsafe impl Sync for #ident {}

        impl ::std::clone::Clone for #ident {
            pub fn clone(&self) -> Self {
                #ident{
                    #(#names_left: self.#names_right,)*
                }
            }
        }
        impl #ident {
            pub fn load<F>(mut f: F) -> ::std::result::Result<Self, Vec<&'static str>>
                where F: FnMut(&::std::ffi::CStr) -> *const c_void
            {
                use std::ffi::CString;
                use std::mem;
                let mut err_str = Vec::new();
                let s = #ident {
                    #(
                        #names_ref: unsafe {
                            let raw_name = stringify!(#raw_names_ref);
                            let cname = CString::new(raw_name).unwrap();
                            let val = f(&cname);
                            if val.is_null(){
                                err_str.push(raw_name);
                            }
                            mem::transmute(val)
                        },
                    )*
                };

                if err_str.is_empty() {
                    Ok(s)
                }
                else{
                    Err(err_str)
                }

            }
            #(
                pub fn #names_ref(#expanded_params_ref) -> #return_types_ref {
                    (self.#names_left)(#(#param_names_ref,)*)
                }
            )*
        }
    }
}
pub fn generate_core_spec(feature: &vkxml::Feature, commands: &CommandMap) -> quote::Tokens {
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
    let version = feature.version_string();
    let instance = generate_function_pointers(
        Ident::from(format!("InstanceFnV{}", version).as_str()),
        &instance_commands,
    );
    let device = generate_function_pointers(
        Ident::from(format!("DeviceFnV{}", version).as_str()),
        &device_commands,
    );
    quote! {
        #instance
        #device
    }
}
