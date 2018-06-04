#![recursion_limit = "256"]
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
extern crate heck;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;
pub extern crate vk_parse;
pub extern crate vkxml;

use heck::{CamelCase, ShoutySnakeCase, SnakeCase};
use proc_macro2::Term;
use quote::Tokens;
use syn::Ident;

pub enum Constant {
    Number(i32),
    Hex(String),
    BitPos(u32),
    CExpr(vkxml::CExpression),
    Text(String),
}
impl Constant {
    pub fn value(&self) -> Option<i64> {
        match *self {
            Constant::Number(n) => Some(n as i64),
            Constant::Hex(ref hex) => i64::from_str_radix(&hex, 16).ok(),
            Constant::BitPos(pos) => Some((1 << pos) as i64),
            _ => None,
        }
    }
    pub fn to_tokens(&self) -> Tokens {
        match *self {
            Constant::Number(n) => {
                let term = Term::intern(&n.to_string());
                quote!{#term}
            }
            Constant::Hex(ref s) => {
                let term = Term::intern(&format!("0x{}", s));
                quote!{#term}
            }
            Constant::Text(ref text) => {
                quote!{#text}
            }
            Constant::CExpr(ref cexpr) => {
                let rexpr = cexpr.replace("~", "!").replace("U", "u32");
                let term = Term::intern(&rexpr);
                quote!{#term}
            }
            Constant::BitPos(pos) => {
                let value = 1 << pos;
                let term = Term::intern(&format!("0b{:b}", value));
                quote!{#term}
            }
        }
    }

    pub fn from_constant(constant: &vkxml::Constant) -> Self {
        let number = constant.number.map(|n| Constant::Number(n));
        let hex = constant.hex.as_ref().map(|hex| Constant::Hex(hex.clone()));
        let bitpos = constant.bitpos.map(|bit| Constant::BitPos(bit));
        let expr = constant
            .c_expression
            .as_ref()
            .map(|e| Constant::CExpr(e.clone()));
        number.or(hex).or(bitpos).or(expr).expect("")
    }
    pub fn from_extension(extension: &vkxml::ExtensionConstant) -> Self {
        let number = extension.number.map(|n| Constant::Number(n));
        let hex = extension.hex.as_ref().map(|hex| Constant::Hex(hex.clone()));
        let bitpos = extension.bitpos.map(|bit| Constant::BitPos(bit));
        let expr = extension
            .c_expression
            .as_ref()
            .map(|e| Constant::CExpr(e.clone()));
        let text = extension.text.as_ref().map(|e| Constant::Text(e.clone()));
        number.or(hex).or(bitpos).or(expr).or(text).expect("")
    }
}

pub trait ConstantExt {}
impl ConstantExt for vkxml::Constant {}
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

fn to_type_tokens(type_name: &str, reference: Option<&vkxml::ReferenceType>) -> Tokens {
    let new_name = match type_name {
        "void" => "c_void",
        "char" => "c_char",
        "float" => "c_float",
        "long" => "c_ulong",
        _ => {
            let prefix = &type_name[0..2];
            if prefix == "Vk" {
                &type_name[2..]
            } else {
                type_name
            }
        }
    };
    let ptr_name = reference
        .as_ref()
        .map(|reference| match *reference {
            &vkxml::ReferenceType::Pointer => "*mut",
            &vkxml::ReferenceType::PointerToPointer => "*mut",
            &vkxml::ReferenceType::PointerToConstPointer => "*const",
        })
        .unwrap_or("");
    let ty: syn::Type = syn::parse_str(&format!("{} {}", ptr_name, new_name)).expect("parse field");
    quote!{#ty}
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
        to_type_tokens(&self.basetype, self.reference.as_ref())
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
pub fn generate_extension(extension: &vkxml::Extension, commands: &CommandMap) -> quote::Tokens {
    let extension_commands: Vec<&vkxml::Command> = extension
        .elements
        .iter()
        .flat_map(|extension| {
            if let &vkxml::ExtensionElement::Require(ref spec) = extension {
                spec.elements
                    .iter()
                    .filter_map(|extension_spec| {
                        if let &vkxml::ExtensionSpecificationElement::CommandReference(
                            ref cmd_ref,
                        ) = extension_spec
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
        .map(|&cmd| cmd)
        .collect();
    if extension_commands.is_empty() {
        return quote!{};
    }
    let name = format!("{}Fn", extension.name.to_camel_case());
    let ident = Ident::from(&name[2..]);
    generate_function_pointers(ident, &extension_commands)
}
pub fn generate_typedef(typedef: &vkxml::Typedef) -> Tokens {
    let typedef_name = to_type_tokens(&typedef.name, None);
    let typedef_ty = to_type_tokens(&typedef.basetype, None);
    quote!{
        pub type #typedef_name = #typedef_ty;
    }
}
pub fn generate_bitmask(bitmask: &vkxml::Bitmask) -> Option<Tokens> {
    // Workaround for empty bitmask
    if bitmask.name.len() == 0 {
        return None;
    }
    let name = &bitmask.name[2..];
    let name_without_flags = name.replace("Flags", "");
    let ident = Ident::from(name);
    let ident_without_flags = Ident::from(name_without_flags.as_str());
    let type_token = to_type_tokens(&bitmask.basetype, None);
    Some(quote!{
        pub type #ident = BitFlags<flags::#ident_without_flags>;
    })
}
pub fn to_variant_ident(enum_name: &str, variant_name: &str) -> Ident {
    let tag = ["AMD", "NN", "KHR", "NV", "EXT", "NVX", "KHX"]
        .iter()
        .filter_map(|tag| {
            if enum_name.ends_with(tag) {
                Some(tag)
            } else {
                None
            }
        })
        .nth(0);

    let name_without_tag = tag.map(|t| enum_name.replace(t, ""))
        .unwrap_or(enum_name.into());
    let variant_without_tag = tag.map(|t| variant_name.replace(t, ""))
        .unwrap_or(variant_name.into());
    let camel_case_name_enum = &name_without_tag.to_camel_case();
    let name = variant_without_tag.to_camel_case()[2..].replace(camel_case_name_enum, "");
    let is_digit = name.chars().nth(0).map(|c| c.is_digit(10)).unwrap_or(false);
    if is_digit {
        Ident::from(format!("Type{}", name).as_str())
    } else {
        Ident::from(name)
    }
}

pub enum EnumType {
    Bitflags(Tokens),
    Enum(Tokens),
}

pub fn generate_enum(_enum: &vkxml::Enumeration) -> EnumType {
    let name = &_enum.name[2..];
    let _name = name.replace("FlagBits", "");
    if name.contains("Bit") {
        let variants = _enum.elements.iter().filter_map(|elem| {
            let (variant_name, value) = match *elem {
                vkxml::EnumerationElement::Enum(ref constant) => {
                    let c = Constant::from_constant(constant);
                    if c.value().map(|v| v == 0).unwrap_or(false) {
                        return None;
                    }
                    (constant.name.as_str(), c.to_tokens())
                }
                _ => {
                    return None;
                }
            };

            let variant_ident = to_variant_ident(&_name, variant_name);
            Some(quote!{
                #variant_ident = #value
            })
        });
        let ident = Ident::from(_name.as_str());
        let q = quote!{
            #[derive(Copy, Clone, Debug, EnumFlags)]
            pub enum #ident {
                #(#variants,)*
            }
        };
        EnumType::Bitflags(q)
    } else {
        let variants = _enum.elements.iter().filter_map(|elem| {
            let (variant_name, value) = match *elem {
                vkxml::EnumerationElement::Enum(ref constant) => {
                    let c = Constant::from_constant(constant);
                    //println!("value {:?}", c.value());
                    (constant.name.as_str(), c.to_tokens())
                }
                _ => {
                    return None;
                }
            };

            let variant_ident = to_variant_ident(&_name, variant_name);
            Some(quote!{
                #variant_ident = #value
            })
        });
        let ident = Ident::from(_name.as_str());
        let q = quote!{
            #[derive(Copy, Clone, Debug)]
            #[repr(C)]
            pub enum #ident {
                #(#variants,)*
            }
        };
        EnumType::Enum(q)
    }
}
pub fn generate_struct(_struct: &vkxml::Struct) -> Tokens {
    let name = Ident::from(&_struct.name[2..]);
    let params = _struct
        .elements
        .iter()
        .filter_map(|elem| match *elem {
            vkxml::StructElement::Member(ref field) => Some(field),
            _ => None,
        })
        .map(|field| {
            let param_ident = field.param_ident();
            let param_ty_tokens = field.type_tokens();
            quote!{pub #param_ident: #param_ty_tokens}
        });
    quote!{
        #[derive(Clone, Debug)]
        #[repr(C)]
        pub struct #name {
            #(#params,)*
        }
    }
}
pub fn generate_definition(definition: &vkxml::DefinitionsElement) -> Option<Tokens> {
    match *definition {
        vkxml::DefinitionsElement::Typedef(ref typedef) => Some(generate_typedef(typedef)),
        vkxml::DefinitionsElement::Struct(ref _struct) => Some(generate_struct(_struct)),
        vkxml::DefinitionsElement::Bitmask(ref mask) => generate_bitmask(mask),
        _ => None,
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

pub fn write_source_code(spec: &vkxml::Registry) {
    use std::fs::File;
    use std::io::Write;
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

    let extensions: Vec<&vkxml::Extension> = spec.elements
        .iter()
        .filter_map(|elem| match elem {
            &vkxml::RegistryElement::Extensions(ref extensions) => Some(extensions),
            _ => None,
        })
        .flat_map(|extensions| extensions.elements.iter().map(|extension| extension))
        .collect();

    let definitions: Vec<&vkxml::DefinitionsElement> = spec.elements
        .iter()
        .filter_map(|elem| match elem {
            &vkxml::RegistryElement::Definitions(ref definitions) => Some(definitions),
            _ => None,
        })
        .flat_map(|definitions| definitions.elements.iter().map(|definition| definition))
        .collect();

    let enums: Vec<&vkxml::Enumeration> = spec.elements
        .iter()
        .filter_map(|elem| match elem {
            &vkxml::RegistryElement::Enums(ref enums) => Some(enums),
            _ => None,
        })
        .flat_map(|enums| {
            enums.elements.iter().filter_map(|_enum| match *_enum {
                vkxml::EnumsElement::Enumeration(ref e) => Some(e),
                _ => None,
            })
        })
        .collect();

    let constants: Vec<&vkxml::Constant> = spec.elements
        .iter()
        .filter_map(|elem| match elem {
            &vkxml::RegistryElement::Constants(ref constants) => Some(constants),
            _ => None,
        })
        .flat_map(|constants| constants.elements.iter())
        .collect();

    //println!("{:#?}", constants);

    let (enum_code, bitflags_code) = enums.into_iter().map(generate_enum).fold(
        (Vec::new(), Vec::new()),
        |mut acc, elem| {
            match elem {
                EnumType::Enum(token) => acc.0.push(token),
                EnumType::Bitflags(token) => acc.1.push(token),
            };
            acc
        },
    );

    let definition_code: Vec<_> = definitions.into_iter().filter_map(generate_definition).collect();

    let feature_code: Vec<_> = features
        .iter()
        .map(|feature| generate_core_spec(feature, &commands))
        .collect();

    let extension_code: Vec<_> = extensions
        .iter()
        .map(|ext| generate_extension(ext, &commands))
        .collect();
    let mut file = File::create("../ash/src/vk_test.rs").expect("vk");
    let source_code = quote!{
        //#(#feature_code)*
        //#(#extension_code)*
        //#(#definition_code)*
        //#(#enum_code)*
        pub mod flags {
            #(#bitflags_code)*
        }
    };
    write!(&mut file, "{}", source_code);
}
