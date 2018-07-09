#![recursion_limit = "256"]
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
#[macro_use]
extern crate nom;
extern crate heck;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate itertools;
extern crate syn;
pub extern crate vk_parse;
pub extern crate vkxml;

use heck::{CamelCase, ShoutySnakeCase, SnakeCase};
use itertools::Itertools;
use proc_macro2::{Term, TokenTree};
use quote::Tokens;
use std::collections::HashMap;
use syn::Ident;

pub struct Foo;
impl Foo {
    const FOO: u32 = 1;
}
#[derive(Copy, Clone, Debug)]
pub enum CType {
    USize,
    U32,
    U64,
    Float,
}
impl CType {
    fn to_tokens(&self) -> Tokens {
        let term = match self {
            CType::USize => Term::intern("usize"),
            CType::U32 => Term::intern("u32"),
            CType::U64 => Term::intern("u64"),
            CType::Float => Term::intern("f32"),
        };
        quote!{#term}
    }
}

named!(ctype<&str, CType>, 
    alt!(
        tag!("ULL") => { |_| CType::U64 } |
        tag!("U") => { |_| CType::U32  } 
    )
);
named!(cexpr<&str, (CType, String)>,
       alt!(
           map!(cfloat, |f| (CType::Float, format!("{:.2}", f))) |
           inverse_number
       )
);

named!(inverse_number<&str, (CType, String)>,
    do_parse!(
        tag!("(")>>
        tag!("~") >>
        s: take_while1!(|c: char| c.is_digit(10)) >> 
        ctype: ctype >>
        minus_num: opt!(
            do_parse!(
                tag!("-") >>
                n: take_while1!(|c: char| c.is_digit(10)) >> 
                (n)
            )
        ) >>
        tag!(")") >>
        (
            {
                let expr = if let Some(minus) = minus_num {
                    format!("!{}-{}", s, minus)
                }
                else{
                    format!("!{}", s)
                };
                (ctype, expr)
            }
        )
    )
);

named!(cfloat<&str, f32>,
    terminated!(nom::float_s, char!('f'))
);

pub fn define_handle_macro() -> Tokens {
    quote! {
        macro_rules! define_handle{
            ($name: ident) => {
                #[derive(Clone, Copy, Debug)]
                #[repr(C)]
                pub struct $name{
                    ptr: *mut u8
                }

                unsafe impl Send for $name {}
                unsafe impl Sync for $name {}

                impl $name{
                    pub unsafe fn null() -> Self{
                        $name{
                            ptr: ::std::ptr::null_mut()
                        }
                    }
                }
            }
        }
    }
}

pub fn handle_nondispatchable_macro() -> Tokens {
    quote!{
        macro_rules! handle_nondispatchable {
            ($name: ident) => {
                #[repr(C)]
                #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash)]
                pub struct $name (uint64_t);

                impl $name{
                    pub fn null() -> $name{
                        $name(0)
                    }
                }
                impl ::std::fmt::Pointer for $name {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
                        write!(f, "0x{:x}", self.0)
                    }
                }

                impl ::std::fmt::Debug for $name {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
                        write!(f, "0x{:x}", self.0)
                    }
                }
            }
        }
    }
}
pub fn vk_version_macros() -> Tokens {
    quote!{
        #[macro_export]
        macro_rules! vk_make_version {
            ($major:expr, $minor:expr, $patch:expr) => {
                (($major as u32) << 22) | (($minor as u32) << 12) | $patch as u32
            };
        }

        #[macro_export]
        macro_rules! vk_version_major {
            ($major:expr) => {
                ($major as uint32_t) >> 22
            };
        }

        #[macro_export]
        macro_rules! vk_version_minor {
            ($minor:expr) => {
                (($minor as uint32_t) >> 12) & 0x3ff
            };
        }

        #[macro_export]
        macro_rules! vk_version_patch {
            ($minor:expr) => {
                ($minor as uint32_t) & 0xfff
            };
        }
    }
}
pub fn vk_bitflags_wrapped_macro() -> Tokens {
    quote!{
        macro_rules! vk_bitflags_wrapped {
            ($name: ident, $all: expr, $flag_type: ty) => {
                #[repr(C)]
                #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
                pub struct $name {flags: $flag_type}

                impl Default for $name{
                    fn default() -> $name {
                        $name {flags: 0}
                    }
                }
                impl ::std::fmt::Debug for $name {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
                        write!(f, "{}({:b})", stringify!($name), self.flags)
                    }
                }

                impl $name {
                    #[inline]
                    pub fn empty() -> $name {
                        $name {flags: 0}
                    }

                    #[inline]
                    pub fn all() -> $name {
                        $name {flags: $all}
                    }

                    #[inline]
                    pub fn flags(self) -> $flag_type {
                        self.flags
                    }

                    #[inline]
                    pub fn from_flags(flags: $flag_type) -> Option<$name> {
                        if flags & !$all == 0 {
                            Some($name {flags: flags})
                        } else {
                            None
                        }
                    }

                    #[inline]
                    pub fn from_flags_truncate(flags: $flag_type) -> $name {
                        $name {flags: flags & $all}
                    }

                    #[inline]
                    pub fn is_empty(self) -> bool {
                        self == $name::empty()
                    }

                    #[inline]
                    pub fn is_all(self) -> bool {
                        self & $name::all() == $name::all()
                    }

                    #[inline]
                    pub fn intersects(self, other: $name) -> bool {
                        self & other != $name::empty()
                    }

                    /// Returns true of `other` is a subset of `self`
                    #[inline]
                    pub fn subset(self, other: $name) -> bool {
                        self & other == other
                    }
                }

                impl ::std::ops::BitOr for $name {
                    type Output = $name;

                    #[inline]
                    fn bitor(self, rhs: $name) -> $name {
                        $name {flags: self.flags | rhs.flags }
                    }
                }

                impl ::std::ops::BitOrAssign for $name {
                    #[inline]
                    fn bitor_assign(&mut self, rhs: $name) {
                        *self = *self | rhs
                    }
                }

                impl ::std::ops::BitAnd for $name {
                    type Output = $name;

                    #[inline]
                    fn bitand(self, rhs: $name) -> $name {
                        $name {flags: self.flags & rhs.flags}
                    }
                }

                impl ::std::ops::BitAndAssign for $name {
                    #[inline]
                    fn bitand_assign(&mut self, rhs: $name) {
                        *self = *self & rhs
                    }
                }

                impl ::std::ops::BitXor for $name {
                    type Output = $name;

                    #[inline]
                    fn bitxor(self, rhs: $name) -> $name {
                        $name {flags: self.flags ^ rhs.flags}
                    }
                }

                impl ::std::ops::BitXorAssign for $name {
                    #[inline]
                    fn bitxor_assign(&mut self, rhs: $name) {
                        *self = *self ^ rhs
                    }
                }

                impl ::std::ops::Sub for $name {
                    type Output = $name;

                    #[inline]
                    fn sub(self, rhs: $name) -> $name {
                        self & !rhs
                    }
                }

                impl ::std::ops::SubAssign for $name {
                    #[inline]
                    fn sub_assign(&mut self, rhs: $name) {
                        *self = *self - rhs
                    }
                }

                impl ::std::ops::Not for $name {
                    type Output = $name;

                    #[inline]
                    fn not(self) -> $name {
                        self ^ $name::all()
                    }
                }
            }
        }
    }
}

pub fn platform_specific_types() -> Tokens {
    quote! {
        pub type RROutput = c_ulong;
        pub type VisualID = c_uint;
        pub type Display = *const c_void;
        pub type Window = c_ulong;
        #[allow(non_camel_case_types)]
        pub type xcb_connection_t = *const c_void;
        #[allow(non_camel_case_types)]
        pub type xcb_window_t = u32;
        #[allow(non_camel_case_types)]
        pub type xcb_visualid_t = *const c_void;
        pub type MirConnection = *const c_void;
        pub type MirSurface = *const c_void;
        pub type HINSTANCE = *const c_void;
        pub type HWND = *const c_void;
        #[allow(non_camel_case_types)]
        pub type wl_display = *const c_void;
        #[allow(non_camel_case_types)]
        pub type wl_surface = *const c_void;
        pub type HANDLE = *mut c_void;
        pub type DWORD = c_ulong;
        pub type WCHAR = wchar_t;
        pub type LPCWSTR = *const WCHAR;

        // FIXME: Platform specific types that should come from a library
        // typedefs are only here so that the code compiles for now
        #[allow(non_camel_case_types)]
        pub type SECURITY_ATTRIBUTES = ();
        // Opage types
        pub type ANativeWindow = c_void;
        pub type AHardwareBuffer = c_void;
    }
}
#[derive(Debug, Copy, Clone)]
pub enum ConstVal {
    U32(u32),
    U64(u64),
    Float(f32),
}
impl ConstVal {
    pub fn bits(&self) -> u64 {
        match self {
            ConstVal::U64(n) => *n,
            _ => panic!("Constval not supported"),
        }
    }
}
pub enum Constant {
    Number(i32),
    Hex(String),
    BitPos(u32),
    CExpr(vkxml::CExpression),
    Text(String),
}
impl quote::ToTokens for ConstVal {
    fn to_tokens(&self, tokens: &mut Tokens) {
        match self {
            ConstVal::U32(n) => n.to_tokens(tokens),
            ConstVal::U64(n) => n.to_tokens(tokens),
            ConstVal::Float(f) => f.to_tokens(tokens),
        }
    }
}
impl Constant {
    // pub fn type(&self) -> Type {

    // }
    pub fn value(&self) -> Option<ConstVal> {
        match *self {
            Constant::Number(n) => Some(ConstVal::U64(n as u64)),
            Constant::Hex(ref hex) => u64::from_str_radix(&hex, 16).ok().map(ConstVal::U64),
            Constant::BitPos(pos) => Some(ConstVal::U64((1 << pos) as u64)),
            _ => None,
        }
    }

    pub fn ty(&self) -> CType {
        match self {
            Constant::Number(_) | Constant::Hex(_) => CType::USize,
            Constant::CExpr(expr) => {
                let (_, (ty, _)) = cexpr(expr).expect("Unable to parse cexpr");
                ty
            }
            _ => unimplemented!(),
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
            Constant::CExpr(ref expr) => {
                let (_, (_, rexpr)) = cexpr(expr).expect("Unable to parse cexpr");
                let term = Term::intern(rexpr.as_str());
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
        let mut version = format!("{}", self.version);
        if version.len() == 1 {
            version = format!("{}_0", version)
        }
        version.replace(".", "_")
    }
}
#[derive(Debug, Copy, Clone)]
pub enum FunctionType {
    Static,
    Entry,
    Instance,
    Device,
}
pub trait CommandExt {
    /// Returns the ident in snake_case and without the 'vk' prefix.
    fn function_type(&self) -> FunctionType;
    ///
    /// Returns true if the command is a device level command. This is indicated by
    /// the type of the first parameter.
    fn command_ident(&self) -> Ident;
}

impl CommandExt for vkxml::Command {
    fn command_ident(&self) -> Ident {
        Ident::from(self.name[2..].to_snake_case().as_str())
    }

    fn function_type(&self) -> FunctionType {
        let is_first_param_device = self
            .param
            .iter()
            .nth(0)
            .map(|field| match field.basetype.as_str() {
                "VkDevice" | "VkCommandBuffer" | "VkQueue" => true,
                _ => false,
            })
            .unwrap_or(false);
        match self.name.as_str() {
            "vkGetInstanceProcAddr" => FunctionType::Static,
            "vkCreateInstance"
            | "vkEnumerateInstanceLayerProperties"
            | "vkEnumerateInstanceExtensionProperties" => FunctionType::Entry,
            // This is actually not a device level function
            "vkGetDeviceProcAddr" => FunctionType::Instance,
            _ => {
                if is_first_param_device {
                    FunctionType::Device
                } else {
                    FunctionType::Instance
                }
            }
        }
    }
}

pub trait FieldExt {
    /// Returns the name of the paramter that doesn't clash with Rusts resevered
    /// keywords
    fn param_ident(&self) -> Ident;

    /// Returns the basetype ident and removes the 'Vk' prefix
    fn type_tokens(&self) -> Tokens;
    fn is_clone(&self) -> bool;
}

pub trait ToTokens {
    fn to_tokens(&self) -> Tokens;
}
impl ToTokens for vkxml::ReferenceType {
    fn to_tokens(&self) -> Tokens {
        let ptr_name = match self {
            vkxml::ReferenceType::Pointer => "*const",
            vkxml::ReferenceType::PointerToPointer => "*mut *mut",
            vkxml::ReferenceType::PointerToConstPointer => "*const",
        };
        let ident = Term::intern(ptr_name);
        quote!{
            #ident
        }
    }
}
fn name_to_tokens(type_name: &str) -> Tokens {
    let new_name = match type_name {
        "int" => "c_int",
        "void" => "c_void",
        "char" => "c_char",
        "float" => "c_float",
        "long" => "c_ulong",
        _ => {
            if type_name.starts_with("Vk") {
                &type_name[2..]
            } else {
                type_name
            }
        }
    };
    let new_name = new_name.replace("FlagBits", "Flags");
    let name = Term::intern(new_name.as_str());
    quote! {
        #name
    }
}
fn to_type_tokens(type_name: &str, reference: Option<&vkxml::ReferenceType>) -> Tokens {
    let new_name = name_to_tokens(type_name);
    let ptr_name = reference.map(|r| r.to_tokens()).unwrap_or(quote!{});
    quote!{#ptr_name #new_name}
}

impl FieldExt for vkxml::Field {
    fn is_clone(&self) -> bool {
        true
    }
    fn param_ident(&self) -> Ident {
        let name = self.name.as_ref().map(|s| s.as_str()).unwrap_or("field");
        let name_corrected = match name {
            "type" => "ty",
            _ => name,
        };
        Ident::from(name_corrected.to_snake_case().as_str())
    }

    fn type_tokens(&self) -> Tokens {
        let ty = name_to_tokens(&self.basetype);
        let pointer = self
            .reference
            .as_ref()
            .map(|r| r.to_tokens())
            .unwrap_or(quote!{});
        let pointer_ty = quote!{
            #pointer #ty
        };
        let array = self.array.as_ref().and_then(|arraytype| match arraytype {
            vkxml::ArrayType::Static => {
                let size = self
                    .size
                    .as_ref()
                    .or(self.size_enumref.as_ref())
                    .expect("Should have size");
                let size = Term::intern(size);
                Some(quote!{
                    [#ty; #size]
                })
            }
            _ => None,
        });
        array.unwrap_or(pointer_ty)
    }
}

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
            let params: Vec<_> = cmd
                .param
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
            fn clone(&self) -> Self {
                #ident{
                    #(#names_left: self.#names_right,)*
                }
            }
        }
        impl #ident {
            pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
                where F: FnMut(&::std::ffi::CStr) -> *const c_void
            {
                let mut _err_str = Vec::new();
                let s = #ident {
                    #(
                        #names_ref: unsafe {
                            let raw_name = stringify!(#raw_names_ref);
                            let cname = ::std::ffi::CString::new(raw_name).unwrap();
                            let val = _f(&cname);
                            if val.is_null(){
                                _err_str.push(raw_name);
                            }
                            ::std::mem::transmute(val)
                        },
                    )*
                };

                if _err_str.is_empty() {
                    Ok(s)
                }
                else{
                    Err(_err_str)
                }

            }
            #(
                pub unsafe fn #names_ref(&self, #expanded_params_ref) -> #return_types_ref {
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
                    .filter_map(|extension_spec| match extension_spec {
                        vkxml::ExtensionSpecificationElement::CommandReference(ref cmd_ref) => {
                            Some(cmd_ref)
                        }
                        vkxml::ExtensionSpecificationElement::Constant(ref field) => {
                            //println!("EXT {:?}", field);
                            None
                        }
                        vkxml::ExtensionSpecificationElement::Enum(ref field) => {
                            //println!("Enum {:?}", field);
                            None
                        }
                        _ => None,
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
    // If this enum has constants, then it will generated later in generate_enums.
    if bitmask.enumref.is_some() {
        return None;
    }

    let name = &bitmask.name[2..];
    let ident = Ident::from(name);
    Some(quote!{
        vk_bitflags_wrapped!(#ident, 0b0, Flags);
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

    let name_without_tag = tag
        .map(|t| enum_name.replace(t, ""))
        .unwrap_or(enum_name.into());
    let variant_without_tag = tag
        .map(|t| variant_name.replace(&format!("_{}", t), ""))
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

pub fn generate_enum(
    _enum: &vkxml::Enumeration,
    create_info_constants: &[&vkxml::Constant],
) -> EnumType {
    let name = &_enum.name[2..];
    let _name = name.replace("FlagBits", "Flags");
    let ident = Ident::from(_name.as_str());
    let variants = _enum.elements.iter().filter_map(|elem| {
        let (variant_name, value) = match *elem {
            vkxml::EnumerationElement::Enum(ref constant) => {
                let c = Constant::from_constant(constant);
                (constant.name.as_str(), c.to_tokens())
            }
            _ => {
                return None;
            }
        };

        let _name = _name.split("Flags").nth(0).expect("split");
        let struct_name = _name.to_shouty_snake_case();
        println!("{}", struct_name);
        let new_variant_name = variant_name.replace(&struct_name, "").replace("VK", "");
        let new_variant_name = new_variant_name.trim_matches('_');
        let is_digit = new_variant_name
            .chars()
            .nth(0)
            .map(|c| c.is_digit(10))
            .unwrap_or(false);
        let variant_ident = if is_digit {
            Ident::from(format!("TYPE_{}", new_variant_name).as_str())
        } else {
            Ident::from(new_variant_name)
        };
        Some((variant_ident, value))
    }).collect_vec();
    if name.contains("Bit") {
        let ident = Ident::from(_name.as_str());
        let all_bits = _enum
            .elements
            .iter()
            .filter_map(|elem| match elem {
                vkxml::EnumerationElement::Enum(ref constant) => {
                    let c = Constant::from_constant(constant);
                    c.value()
                }
                _ => None,
            })
            .fold(0, |acc, next| acc | next.bits());
        let all_bits_term = Term::intern(&format!("0b{:b}", all_bits));

        let variants = variants.iter().map(|(variant_ident, value)|{
            quote!{
                pub const #variant_ident: Self = #ident { flags: #value }
            }
        });
        let q = quote!{
            vk_bitflags_wrapped!(#ident, #all_bits_term, Flags);
            impl #ident {
                #(#variants;)*
            }
        };
        EnumType::Bitflags(q)
    } else {
        let variants = variants.iter().map(|(variant_ident, value)|{
            quote!{
                pub const #variant_ident: Self = #ident(#value)
            }
        });
        let q = match _name.as_str() {
            //"StructureType" => generate_structure_type(&_name, _enum, create_info_constants),
            //"Result" => generate_result(&_name, _enum),
            _ => {
                quote!{
                    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
                    #[repr(C)]
                    pub struct #ident(pub i32);
                    impl #ident {
                        #(
                            #variants;
                        )*
                    }
                }
            }
        };
        EnumType::Enum(q)
    }
}

pub fn generate_structure_type(
    name: &str,
    _enum: &vkxml::Enumeration,
    create_info_constants: &[&vkxml::Constant],
) -> Tokens {
    let ident = Ident::from(name);
    let constants: Vec<_> = _enum
        .elements
        .iter()
        .filter_map(|elem| match elem {
            vkxml::EnumerationElement::Enum(constant) => Some(constant),
            _ => None,
        })
        .collect();
    let variants = constants
        .iter()
        .chain(create_info_constants.into_iter())
        .map(|constant| {
            let value_tokens = Constant::from_constant(constant).to_tokens();
            let variant_ident = to_variant_ident(&name, &constant.name);
            quote!{
                #variant_ident = #value_tokens
            }
        });
    quote!{
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(C)]
        pub enum #ident {
            #(#variants,)*
        }
    }
}
pub fn generate_result(name: &str, _enum: &vkxml::Enumeration) -> Tokens {
    let ident = Ident::from(name);
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
        let variant_ident = to_variant_ident(&name, variant_name);
        Some(quote!{
            #variant_ident = #value
        })
    });
    let notation = _enum.elements.iter().filter_map(|elem| {
        let (variant_name, notation) = match *elem {
            vkxml::EnumerationElement::Enum(ref constant) => (
                constant.name.as_str(),
                constant.notation.as_ref().map(|s| s.as_str()).unwrap_or(""),
            ),
            _ => {
                return None;
            }
        };

        let variant_ident = to_variant_ident(&name, variant_name);
        Some(quote!{
            #ident::#variant_ident => write!(fmt, #notation)
        })
    });

    quote!{
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(C)]
        pub enum #ident {
            #(#variants,)*
        }
        impl ::std::error::Error for #ident {
            fn description(&self) -> &str {
                "vk::Result"
            }
        }
        impl ::std::fmt::Display for #ident {
            fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                writeln!(fmt, "vk::Result::{:?}", self)?;
                match self {
                    #(#notation),*
                }
            }
        }
    }
}
pub trait StructExt {}
pub fn generate_struct(_struct: &vkxml::Struct) -> Tokens {
    let name = to_type_tokens(&_struct.name, None);
    let members = _struct.elements.iter().filter_map(|elem| match *elem {
        vkxml::StructElement::Member(ref field) => Some(field),
        _ => None,
    });

    let params = members.clone().map(|field| {
        let param_ident = field.param_ident();
        let param_ty_tokens = field.type_tokens();
        quote!{pub #param_ident: #param_ty_tokens}
    });

    let contains_pfn = members.clone().any(|field| {
        field
            .name
            .as_ref()
            .map(|n| n.contains("pfn"))
            .unwrap_or(false)
    });

    let derive = if contains_pfn {
        quote!{
            #[derive(Copy, Clone)]
        }
    } else {
        // FIXME: Properly derive Debug
        quote!{
            #[derive(Copy, Clone)]
        }
    };
    quote!{
        #[repr(C)]
        #derive
        pub struct #name {
            #(#params,)*
        }
    }
}

pub fn generate_handle(handle: &vkxml::Handle) -> Option<Tokens> {
    if handle.name == "" {
        return None;
    }
    let tokens = match handle.ty {
        vkxml::HandleType::Dispatch => {
            let name = &handle.name[2..];
            let name = Ident::from(name);
            quote! {
                define_handle!(#name);
            }
        }
        vkxml::HandleType::NoDispatch => {
            let name = &handle.name[2..];
            let name = Ident::from(name);
            quote! {
                handle_nondispatchable!(#name);
            }
        }
    };
    Some(tokens)
}
fn generate_funcptr(fnptr: &vkxml::FunctionPointer) -> Tokens {
    let name = Ident::from(fnptr.name.as_str());
    let ret_ty_tokens = fnptr.return_type.type_tokens();
    quote!{
        #[allow(non_camel_case_types)]
        pub type #name = unsafe extern "system" fn() -> #ret_ty_tokens;
    }
}

fn generate_union(union: &vkxml::Union) -> Tokens {
    let name = to_type_tokens(&union.name, None);
    let fields = union.elements.iter().map(|field| {
        let name = field.param_ident();
        let ty = field.type_tokens();
        quote!{
            pub #name: #ty
        }
    });
    quote!{
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub union #name {
            #(#fields),*
        }
    }
}
pub fn generate_definition(definition: &vkxml::DefinitionsElement) -> Option<Tokens> {
    match *definition {
        vkxml::DefinitionsElement::Typedef(ref typedef) => Some(generate_typedef(typedef)),
        vkxml::DefinitionsElement::Struct(ref _struct) => Some(generate_struct(_struct)),
        vkxml::DefinitionsElement::Bitmask(ref mask) => generate_bitmask(mask),
        vkxml::DefinitionsElement::Handle(ref handle) => generate_handle(handle),
        vkxml::DefinitionsElement::FuncPtr(ref fp) => Some(generate_funcptr(fp)),
        vkxml::DefinitionsElement::Union(ref union) => Some(generate_union(union)),
        _ => None,
    }
}
pub fn generate_feature(feature: &vkxml::Feature, commands: &CommandMap) -> quote::Tokens {
    let (static_commands, entry_commands, device_commands, instance_commands) = feature
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
        .fold(
            (Vec::new(), Vec::new(), Vec::new(), Vec::new()),
            |mut acc, &cmd_ref| {
                match cmd_ref.function_type() {
                    FunctionType::Static => {
                        acc.0.push(cmd_ref);
                    }
                    FunctionType::Entry => {
                        acc.1.push(cmd_ref);
                    }
                    FunctionType::Device => {
                        acc.2.push(cmd_ref);
                    }
                    FunctionType::Instance => {
                        acc.3.push(cmd_ref);
                    }
                }
                acc
            },
        );
    let version = feature.version_string();
    let static_fn = if feature.version == 1.0 {
        generate_function_pointers(Ident::from("StaticFn"), &static_commands)
    } else {
        quote!{}
    };
    let entry = generate_function_pointers(
        Ident::from(format!("EntryFnV{}", version).as_str()),
        &entry_commands,
    );
    let instance = generate_function_pointers(
        Ident::from(format!("InstanceFnV{}", version).as_str()),
        &instance_commands,
    );
    let device = generate_function_pointers(
        Ident::from(format!("DeviceFnV{}", version).as_str()),
        &device_commands,
    );
    quote! {
        #static_fn
        #entry
        #instance
        #device
    }
}
pub fn generate_constant(constant: &vkxml::Constant) -> Tokens {
    let c = Constant::from_constant(constant);
    let ident = Ident::from(constant.name.as_str());
    let value = c.to_tokens();
    let ty = c.ty().to_tokens();
    quote!{
        pub const #ident: #ty = #value;
    }
}

pub fn write_source_code(spec: &vkxml::Registry) {
    use std::fs::File;
    use std::io::Write;
    println!("{:#?}", spec);
    let commands: HashMap<vkxml::Identifier, &vkxml::Command> = spec
        .elements
        .iter()
        .filter_map(|elem| match elem {
            &vkxml::RegistryElement::Commands(ref cmds) => Some(cmds),
            _ => None,
        })
        .flat_map(|cmds| cmds.elements.iter().map(|cmd| (cmd.name.clone(), cmd)))
        .collect();

    let features: Vec<&vkxml::Feature> = spec
        .elements
        .iter()
        .filter_map(|elem| match elem {
            &vkxml::RegistryElement::Features(ref features) => Some(features),
            _ => None,
        })
        .flat_map(|features| features.elements.iter().map(|feature| feature))
        .collect();

    let extensions: Vec<&vkxml::Extension> = spec
        .elements
        .iter()
        .filter_map(|elem| match elem {
            &vkxml::RegistryElement::Extensions(ref extensions) => Some(extensions),
            _ => None,
        })
        .flat_map(|extensions| extensions.elements.iter().map(|extension| extension))
        .collect();

    let definitions: Vec<&vkxml::DefinitionsElement> = spec
        .elements
        .iter()
        .filter_map(|elem| match elem {
            &vkxml::RegistryElement::Definitions(ref definitions) => Some(definitions),
            _ => None,
        })
        .flat_map(|definitions| definitions.elements.iter().map(|definition| definition))
        .collect();

    let enums: Vec<&vkxml::Enumeration> = spec
        .elements
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

    let constants: Vec<&vkxml::Constant> = spec
        .elements
        .iter()
        .filter_map(|elem| match elem {
            &vkxml::RegistryElement::Constants(ref constants) => Some(constants),
            _ => None,
        })
        .flat_map(|constants| constants.elements.iter())
        .collect();

    let (enum_code, bitflags_code) = enums.into_iter().map(|e| generate_enum(e, &[])).fold(
        (Vec::new(), Vec::new()),
        |mut acc, elem| {
            match elem {
                EnumType::Enum(token) => acc.0.push(token),
                EnumType::Bitflags(token) => acc.1.push(token),
            };
            acc
        },
    );
    let constants_code: Vec<_> = constants
        .iter()
        .map(|constant| generate_constant(constant))
        .collect();

    let definition_code: Vec<_> = definitions
        .into_iter()
        .filter_map(generate_definition)
        .collect();

    let feature_code: Vec<_> = features
        .iter()
        .map(|feature| generate_feature(feature, &commands))
        .collect();

    let extension_code: Vec<_> = extensions
        .iter()
        .map(|ext| generate_extension(ext, &commands))
        .collect();
    let mut file = File::create("../ash/src/vk.rs").expect("vk");
    let bitflags_macro = vk_bitflags_wrapped_macro();
    let handle_nondispatchable_macro = handle_nondispatchable_macro();
    let define_handle_macro = define_handle_macro();
    let version_macros = vk_version_macros();
    let platform_specific_types = platform_specific_types();
    let source_code = quote!{
        pub use libc::*;
        // #version_macros
        // #platform_specific_types
        #bitflags_macro
        // #handle_nondispatchable_macro
        // #define_handle_macro
        // #(#feature_code)*
        // #(#definition_code)*
        #(#enum_code)*
        #(#bitflags_code)*
        // #(#constants_code)*
        // #(#extension_code)*
    };
    write!(&mut file, "{}", source_code);
}
