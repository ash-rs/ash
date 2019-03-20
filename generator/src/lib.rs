#![recursion_limit = "256"]
extern crate heck;
extern crate itertools;
#[macro_use]
extern crate nom;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;
pub extern crate vk_parse;
pub extern crate vkxml;

use heck::{CamelCase, ShoutySnakeCase, SnakeCase};
use itertools::Itertools;
use proc_macro2::Term;
use quote::Tokens;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::Path;
use syn::Ident;
pub trait ExtensionExt {}
#[derive(Copy, Clone, Debug)]
pub enum CType {
    USize,
    U32,
    U64,
    Float,
    Bool32,
}

impl CType {
    fn to_tokens(self) -> Tokens {
        let term = match self {
            CType::USize => Term::intern("usize"),
            CType::U32 => Term::intern("u32"),
            CType::U64 => Term::intern("u64"),
            CType::Float => Term::intern("f32"),
            CType::Bool32 => Term::intern("Bool32"),
        };
        quote! {#term}
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
        #[macro_export]
        macro_rules! define_handle{
            ($name: ident, $ty: ident) => {
                #[repr(transparent)]
                #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash)]
                pub struct $name(*mut u8);
                impl Default for $name {
                    fn default() -> $name {
                        $name::null()
                    }
                }

                impl Handle for $name {
                    const TYPE: ObjectType = ObjectType::$ty;
                    fn as_raw(self) -> u64 { self.0 as u64 }
                    fn from_raw(x: u64) -> Self { $name(x as _) }
                }

                unsafe impl Send for $name {}
                unsafe impl Sync for $name {}

                impl $name{
                    pub fn null() -> Self{
                        $name(::std::ptr::null_mut())
                    }
                }

                impl fmt::Pointer for $name {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        fmt::Pointer::fmt(&self.0, f)
                    }
                }

                impl fmt::Debug for $name {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        fmt::Debug::fmt(&self.0, f)
                    }
                }
            }
        }
    }
}

pub fn handle_nondispatchable_macro() -> Tokens {
    quote! {
        #[macro_export]
        macro_rules! handle_nondispatchable {
            ($name: ident, $ty: ident) => {
                #[repr(transparent)]
                #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Default)]
                pub struct $name(u64);

                impl Handle for $name {
                    const TYPE: ObjectType = ObjectType::$ty;
                    fn as_raw(self) -> u64 { self.0 as u64 }
                    fn from_raw(x: u64) -> Self { $name(x as _) }
                }

                impl $name{
                    pub fn null() -> $name{
                        $name(0)
                    }
                }

                impl fmt::Pointer for $name {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "0x{:x}", self.0)
                    }
                }

                impl fmt::Debug for $name {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "0x{:x}", self.0)
                    }
                }
            }
        }
    }
}
pub fn vk_version_macros() -> Tokens {
    quote! {
        #[macro_export]
        macro_rules! vk_make_version {
            ($major:expr, $minor:expr, $patch:expr) => {
                (($major as u32) << 22) | (($minor as u32) << 12) | $patch as u32
            };
        }

        #[macro_export]
        macro_rules! vk_version_major {
            ($major:expr) => {
                ($major as u32) >> 22
            };
        }

        #[macro_export]
        macro_rules! vk_version_minor {
            ($minor:expr) => {
                (($minor as u32) >> 12) & 0x3ff
            };
        }

        #[macro_export]
        macro_rules! vk_version_patch {
            ($minor:expr) => {
                ($minor as u32) & 0xfff
            };
        }
    }
}
pub fn vk_bitflags_wrapped_macro() -> Tokens {
    quote! {
        #[macro_export]
        macro_rules! vk_bitflags_wrapped {
            ($name: ident, $all: expr, $flag_type: ty) => {

                impl Default for $name{
                    fn default() -> $name {
                        $name(0)
                    }
                }
                impl fmt::Debug for $name {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}({:b})", stringify!($name), self.0)
                    }
                }

                impl $name {
                    #[inline]
                    pub fn empty() -> $name {
                        $name(0)
                    }

                    #[inline]
                    pub fn all() -> $name {
                        $name($all)
                    }

                    #[inline]
                    pub fn from_raw(x: $flag_type) -> Self { $name(x) }

                    #[inline]
                    pub fn as_raw(self) -> $flag_type { self.0 }

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

                    /// Returns whether `other` is a subset of `self`
                    #[inline]
                    pub fn contains(self, other: $name) -> bool {
                        self & other == other
                    }
                }

                impl ::std::ops::BitOr for $name {
                    type Output = $name;

                    #[inline]
                    fn bitor(self, rhs: $name) -> $name {
                        $name (self.0 | rhs.0 )
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
                        $name (self.0 & rhs.0)
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
                        $name (self.0 ^ rhs.0 )
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
        pub type wl_display = c_void;
        #[allow(non_camel_case_types)]
        pub type wl_surface = c_void;
        pub type HANDLE = *mut c_void;
        pub type DWORD = c_ulong;
        pub type LPCWSTR = *const u16;
        #[allow(non_camel_case_types)]
        pub type zx_handle_t = u32;

        // FIXME: Platform specific types that should come from a library id:0
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
pub trait ConstantExt {
    fn variant_ident(&self, enum_name: &str) -> Ident;
    fn to_tokens(&self) -> Tokens;
    fn notation(&self) -> Option<&str>;
}

impl ConstantExt for vkxml::ExtensionEnum {
    fn variant_ident(&self, enum_name: &str) -> Ident {
        variant_ident(enum_name, &self.name)
    }
    fn to_tokens(&self) -> Tokens {
        Constant::from_extension_enum(self).expect("").to_tokens()
    }
    fn notation(&self) -> Option<&str> {
        self.notation.as_ref().map(|s| s.as_str())
    }
}

impl ConstantExt for vkxml::Constant {
    fn variant_ident(&self, enum_name: &str) -> Ident {
        variant_ident(enum_name, &self.name)
    }
    fn to_tokens(&self) -> Tokens {
        Constant::from_constant(self).to_tokens()
    }
    fn notation(&self) -> Option<&str> {
        self.notation.as_ref().map(|s| s.as_str())
    }
}

#[derive(Debug)]
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
                quote! {#term}
            }
            Constant::Hex(ref s) => {
                let term = Term::intern(&format!("0x{}", s));
                quote! {#term}
            }
            Constant::Text(ref text) => {
                quote! {#text}
            }
            Constant::CExpr(ref expr) => {
                let (_, (_, rexpr)) = cexpr(expr).expect("Unable to parse cexpr");
                let term = Term::intern(rexpr.as_str());
                quote! {#term}
            }
            Constant::BitPos(pos) => {
                let value = 1 << pos;
                let term = Term::intern(&format!("0b{:b}", value));
                quote! {#term}
            }
        }
    }

    pub fn from_extension_enum(constant: &vkxml::ExtensionEnum) -> Option<Self> {
        let number = constant.number.map(Constant::Number);
        let hex = constant.hex.as_ref().map(|hex| Constant::Hex(hex.clone()));
        let bitpos = constant.bitpos.map(Constant::BitPos);
        let expr = constant
            .c_expression
            .as_ref()
            .map(|e| Constant::CExpr(e.clone()));
        number.or(hex).or(bitpos).or(expr)
    }

    pub fn from_constant(constant: &vkxml::Constant) -> Self {
        let number = constant.number.map(Constant::Number);
        let hex = constant.hex.as_ref().map(|hex| Constant::Hex(hex.clone()));
        let bitpos = constant.bitpos.map(Constant::BitPos);
        let expr = constant
            .c_expression
            .as_ref()
            .map(|e| Constant::CExpr(e.clone()));
        number.or(hex).or(bitpos).or(expr).expect("")
    }
}

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
            .get(0)
            .map(|field| match field.basetype.as_str() {
                "VkDevice" | "VkCommandBuffer" | "VkQueue" => true,
                _ => false,
            })
            .unwrap_or(false);
        match self.name.as_str() {
            "vkGetInstanceProcAddr" => FunctionType::Static,
            "vkCreateInstance"
            | "vkEnumerateInstanceLayerProperties"
            | "vkEnumerateInstanceExtensionProperties"
            | "vkEnumerateInstanceVersion" => FunctionType::Entry,
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

    /// Returns the basetype ident and removes the 'Vk' prefix. When `is_ffi_param` is `true`
    /// array types (e.g. `[f32; 3]`) will be converted to pointer types (e.g. `&[f32; 3]`),
    /// which is needed for `C` function parameters. Set to `false` for struct definitions.
    fn type_tokens(&self, is_ffi_param: bool) -> Tokens;
    fn is_clone(&self) -> bool;
}

pub trait ToTokens {
    fn to_tokens(&self, is_const: bool) -> Tokens;
}
impl ToTokens for vkxml::ReferenceType {
    fn to_tokens(&self, is_const: bool) -> Tokens {
        let ptr_name = match self {
            vkxml::ReferenceType::Pointer => {
                if is_const {
                    "*const"
                } else {
                    "*mut"
                }
            }
            vkxml::ReferenceType::PointerToPointer => "*mut *mut",
            vkxml::ReferenceType::PointerToConstPointer => {
                if is_const {
                    "*const *const"
                } else {
                    "*mut *const"
                }
            }
        };
        let ident = Term::intern(ptr_name);
        quote! {
            #ident
        }
    }
}
fn name_to_tokens(type_name: &str) -> Ident {
    let new_name = match type_name {
        "uint8_t" => "u8",
        "uint16_t" => "u16",
        "uint32_t" => "u32",
        "uint64_t" => "u64",
        "int8_t" => "i8",
        "int16_t" => "i16",
        "int32_t" => "i32",
        "int64_t" => "i64",
        "size_t" => "usize",
        "int" => "c_int",
        "void" => "c_void",
        "char" => "c_char",
        "float" => "f32",
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
    Ident::from(new_name.as_str())
}
fn to_type_tokens(type_name: &str, reference: Option<&vkxml::ReferenceType>) -> Tokens {
    let new_name = name_to_tokens(type_name);
    let ptr_name = reference.map(|r| r.to_tokens(false)).unwrap_or(quote! {});
    quote! {#ptr_name #new_name}
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

    fn type_tokens(&self, is_ffi_param: bool) -> Tokens {
        let ty = name_to_tokens(&self.basetype);
        let pointer = self
            .reference
            .as_ref()
            .map(|r| r.to_tokens(self.is_const))
            .unwrap_or(quote! {});
        let pointer_ty = quote! {
            #pointer #ty
        };
        let array = self.array.as_ref().and_then(|arraytype| match arraytype {
            vkxml::ArrayType::Static => {
                let size = self
                    .size
                    .as_ref()
                    .or_else(|| self.size_enumref.as_ref())
                    .expect("Should have size");
                // Make sure we also rename the constant, that is
                // used inside the static array
                let size = constant_name(size);
                let size = Term::intern(&size);
                // arrays in c are always passed as a pointer
                if is_ffi_param {
                    Some(quote! {
                        &[#ty; #size]
                    })
                } else {
                    Some(quote! {
                        [#ty; #size]
                    })
                }
            }
            _ => None,
        });
        array.unwrap_or(pointer_ty)
    }
}

pub type CommandMap<'a> = HashMap<vkxml::Identifier, &'a vkxml::Command>;

fn generate_function_pointers<'a>(
    ident: Ident,
    commands: &[&'a vkxml::Command],
    fn_cache: &mut HashSet<&'a str>,
) -> quote::Tokens {
    let names: Vec<_> = commands.iter().map(|cmd| cmd.command_ident()).collect();
    let names_ref = &names;
    let names_ref1 = &names;
    let names_ref2 = &names;
    let names_ref3 = &names;
    let raw_names: Vec<_> = commands
        .iter()
        .map(|cmd| Ident::from(cmd.name.as_str()))
        .collect();
    let raw_names_ref = &raw_names;
    let names_left = &names;
    let names_right = &names;

    let pfn_commands: Vec<_> = commands
        .iter()
        .filter(|cmd| {
            let ident = cmd.name.as_str();
            if !fn_cache.contains(ident) {
                fn_cache.insert(ident);
                return true;
            } else {
                return false;
            }
        })
        .collect();

    let params: Vec<Vec<(Ident, Tokens)>> = commands
        .iter()
        .map(|cmd| {
            let params: Vec<_> = cmd
                .param
                .iter()
                .map(|field| {
                    let name = field.param_ident();
                    let ty = field.type_tokens(true);
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
                quote! {#param_name: #param_ty}
            });
            quote! {
                #(#inner_params_iter,)*
            }
        })
        .collect();
    let expanded_params_unused: Vec<_> = params
        .iter()
        .map(|inner_params| {
            let inner_params_iter = inner_params.iter().map(|&(ref param_name, ref param_ty)| {
                let unused_name = Ident::from(format!("_{}", param_name).as_str());
                quote! {#unused_name: #param_ty}
            });
            quote! {
                #(#inner_params_iter,)*
            }
        })
        .collect();
    let expanded_params_ref = &expanded_params;

    let return_types: Vec<_> = commands
        .iter()
        .map(|cmd| cmd.return_type.type_tokens(true))
        .collect();
    let return_types_ref = &return_types;

    let pfn_names: Vec<_> = pfn_commands
        .iter()
        .map(|cmd| Ident::from(format!("PFN_{}", cmd.name.as_str())))
        .collect();
    let pfn_names_ref = &pfn_names;

    let signature_params: Vec<Vec<_>> = pfn_commands
        .iter()
        .map(|cmd| {
            let params: Vec<_> = cmd
                .param
                .iter()
                .map(|field| {
                    let name = field.param_ident();
                    let ty = field.type_tokens(true);
                    quote! { #name: #ty }
                })
                .collect();
            params
        })
        .collect();
    let signature_params_ref = &signature_params;

    let pfn_return_types: Vec<_> = pfn_commands
        .iter()
        .map(|cmd| cmd.return_type.type_tokens(true))
        .collect();
    let pfn_return_types_ref = &pfn_return_types;

    quote! {
        #(
            #[allow(non_camel_case_types)]
            pub type #pfn_names_ref = extern "system" fn(#(#signature_params_ref),*) -> #pfn_return_types_ref;
        )*

        pub struct #ident {
            #(
                pub #names_ref: extern "system" fn(#expanded_params_ref) -> #return_types_ref,
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
            pub fn load<F>(mut _f: F) -> Self
                where F: FnMut(&::std::ffi::CStr) -> *const c_void
            {
                #ident {
                    #(
                        #names_ref: unsafe {

                            extern "system" fn #names_ref1 (#expanded_params_unused) -> #return_types_ref {
                                panic!(concat!("Unable to load ", stringify!(#names_ref2)))
                            }
                            let raw_name = stringify!(#raw_names_ref);
                            let cname = ::std::ffi::CString::new(raw_name).unwrap();
                            let val = _f(&cname);
                            if val.is_null(){
                                #names_ref3
                            }
                            else{
                                ::std::mem::transmute(val)
                            }
                        },
                    )*
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
pub struct ExtensionConstant<'a> {
    pub name: &'a str,
    pub constant: Constant,
}
impl<'a> ConstantExt for ExtensionConstant<'a> {
    fn variant_ident(&self, enum_name: &str) -> Ident {
        variant_ident(enum_name, self.name)
    }
    fn to_tokens(&self) -> Tokens {
        self.constant.to_tokens()
    }
    fn notation(&self) -> Option<&str> {
        None
    }
}

pub fn generate_extension_constants<'a>(
    extension_name: &str,
    extension_number: i64,
    extension_items: &'a [vk_parse::ExtensionChild],
    const_cache: &mut HashSet<&'a str>,
    const_values: &mut BTreeMap<Ident, Vec<Ident>>,
) -> quote::Tokens {
    use vk_parse::EnumSpec;
    let items = extension_items
        .iter()
        .filter_map(|item| match item {
            vk_parse::ExtensionChild::Require { items, .. } => Some(items.iter()),
            _ => None,
        })
        .flat_map(|iter| iter);
    let enum_tokens = items.filter_map(|item| match item {
        vk_parse::InterfaceItem::Enum(_enum) => {
            if const_cache.contains(_enum.name.as_str()) {
                return None;
            }
            let (constant, extends) = match &_enum.spec {
                EnumSpec::Bitpos { bitpos, extends } => {
                    Some((Constant::BitPos(*bitpos as u32), extends.clone()))
                }
                EnumSpec::Offset {
                    offset,
                    extends,
                    extnumber,
                    dir: positive,
                } => {
                    let ext_base = 1_000_000_000;
                    let ext_block_size = 1000;
                    let extnumber = extnumber.unwrap_or_else(|| extension_number);
                    let value = ext_base + (extnumber - 1) * ext_block_size + offset;
                    let value = if *positive { value } else { -value };
                    Some((Constant::Number(value as i32), Some(extends.clone())))
                }
                _ => None,
            }?;
            let extends = extends?;
            let ext_constant = ExtensionConstant {
                name: &_enum.name,
                constant,
            };
            let ident = name_to_tokens(&extends);
            const_values
                .entry(ident.clone())
                .or_insert_with(Vec::new)
                .push(ext_constant.variant_ident(&extends));
            let impl_block = bitflags_impl_block(ident, &extends, &[&ext_constant]);
            let doc_string = format!("Generated from '{}'", extension_name);
            let q = quote! {
                #[doc = #doc_string]
                #impl_block
            };

            const_cache.insert(_enum.name.as_str());
            Some(q)
        }
        _ => None,
    });
    quote! {
        #(#enum_tokens)*
    }
}
pub fn generate_extension_commands<'a>(
    extension_name: &str,
    items: &[vk_parse::ExtensionChild],
    cmd_map: &CommandMap<'a>,
    fn_cache: &mut HashSet<&'a str>,
) -> Tokens {
    let commands = items
        .iter()
        .filter_map(|ext_item| match ext_item {
            vk_parse::ExtensionChild::Require { items, .. } => {
                Some(items.iter().filter_map(|item| match item {
                    vk_parse::InterfaceItem::Command { ref name, .. } => {
                        cmd_map.get(name).map(|c| *c)
                    }
                    _ => None,
                }))
            }
            _ => None,
        })
        .flat_map(|iter| iter)
        .collect_vec();
    let name = format!("{}Fn", extension_name.to_camel_case());
    let ident = Ident::from(&name[2..]);
    let fp = generate_function_pointers(ident, &commands, fn_cache);
    let byte_name = format!("{}\0", extension_name);

    let byte_name_ident =
        syn::LitByteStr::new(byte_name.as_bytes(), proc_macro2::Span::call_site());
    let extension_cstr = quote! {
        impl #ident {
            pub fn name() -> &'static ::std::ffi::CStr {
                ::std::ffi::CStr::from_bytes_with_nul(#byte_name_ident).expect("Wrong extension string")
            }
        }
    };
    quote! {
        #extension_cstr
        #fp
    }
}
pub fn generate_extension<'a>(
    extension: &'a vk_parse::Extension,
    cmd_map: &CommandMap<'a>,
    const_cache: &mut HashSet<&'a str>,
    const_values: &mut BTreeMap<Ident, Vec<Ident>>,
    fn_cache: &mut HashSet<&'a str>,
) -> Option<quote::Tokens> {
    // Okay this is a little bit odd. We need to generate all extensions, even disabled ones,
    // because otherwise some StructureTypes won't get generated. But we don't generate extensions
    // that are reserved
    if extension.name.contains("RESERVED") {
        return None;
    }
    let extension_tokens = generate_extension_constants(
        &extension.name,
        extension.number.unwrap_or(0),
        &extension.children,
        const_cache,
        const_values,
    );
    let fp = generate_extension_commands(&extension.name, &extension.children, cmd_map, fn_cache);
    let q = quote! {
        #fp
        #extension_tokens
    };
    Some(q)
}
pub fn generate_typedef(typedef: &vkxml::Typedef) -> Tokens {
    let typedef_name = to_type_tokens(&typedef.name, None);
    let typedef_ty = to_type_tokens(&typedef.basetype, None);
    quote! {
        pub type #typedef_name = #typedef_ty;
    }
}
pub fn generate_bitmask(
    bitmask: &vkxml::Bitmask,
    bitflags_cache: &mut HashSet<Ident>,
) -> Option<Tokens> {
    // Workaround for empty bitmask
    if bitmask.name.is_empty() {
        return None;
    }
    // If this enum has constants, then it will generated later in generate_enums.
    if bitmask.enumref.is_some() {
        return None;
    }

    let name = &bitmask.name[2..];
    let ident = Ident::from(name);
    if bitflags_cache.contains(&ident) {
        return None;
    };
    bitflags_cache.insert(ident.clone());
    Some(quote! {
        #[repr(transparent)]
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct #ident(Flags);
        vk_bitflags_wrapped!(#ident, 0b0, Flags);
    })
}

pub enum EnumType {
    Bitflags(Tokens),
    Enum(Tokens),
}

pub fn variant_ident(enum_name: &str, variant_name: &str) -> Ident {
    let _name = enum_name.replace("FlagBits", "");
    // TODO: Should be read from vk.xml id:2
    // TODO: Also needs to be more robust, vendor names can be substrings from itself, id:4
    // like NVX and NV
    let vendors = ["_NVX", "_KHR", "_EXT", "_NV", "_AMD", "_ANDROID", "_GOOGLE"];
    let mut struct_name = _name.to_shouty_snake_case();
    let vendor = vendors
        .into_iter()
        .find(|&vendor| struct_name.contains(vendor))
        .cloned()
        .unwrap_or("");
    struct_name = struct_name.replace(vendor, "");
    let new_variant_name = variant_name.replace(&struct_name, "").replace("VK", "");
    let new_variant_name = new_variant_name
        .trim_matches('_')
        .to_shouty_snake_case()
        .replace("_BIT", "")
        .replace(vendor, "");
    let is_digit = new_variant_name
        .chars()
        .nth(0)
        .map(|c| c.is_digit(10))
        .unwrap_or(false);
    if is_digit {
        Ident::from(format!("TYPE_{}", new_variant_name).as_str())
    } else {
        Ident::from(new_variant_name)
    }
}

pub fn bitflags_impl_block(
    ident: Ident,
    enum_name: &str,
    constants: &[&impl ConstantExt],
) -> Tokens {
    let variants = constants
        .iter()
        .map(|constant| {
            let variant_ident = constant.variant_ident(enum_name);
            let tokens = constant.to_tokens();
            (variant_ident, tokens)
        })
        .collect_vec();

    let notations = constants.iter().map(|constant| {
        constant.notation().map(|n| {
            quote! {
                #[doc = #n]
            }
        })
    });

    let variants =
        variants
            .iter()
            .zip(notations.clone())
            .map(|((variant_ident, value), ref notation)| {
                quote! {
                    #notation
                    pub const #variant_ident: Self = #ident(#value);
                }
            });
    quote! {
        impl #ident {
            #(#variants)*
        }
    }
}

pub fn generate_enum<'a>(
    _enum: &'a vkxml::Enumeration,
    const_cache: &mut HashSet<&'a str>,
    const_values: &mut BTreeMap<Ident, Vec<Ident>>,
    bitflags_cache: &mut HashSet<Ident>,
) -> EnumType {
    let name = &_enum.name[2..];
    let _name = name.replace("FlagBits", "Flags");
    let ident = Ident::from(_name.as_str());
    let constants: Vec<_> = _enum
        .elements
        .iter()
        .filter_map(|elem| match *elem {
            vkxml::EnumerationElement::Enum(ref constant) => Some(constant),
            _ => None,
        })
        .collect_vec();
    let values = const_values.entry(ident.clone()).or_insert_with(Vec::new);
    for constant in &constants {
        const_cache.insert(constant.name.as_str());
        values.push(constant.variant_ident(&_enum.name));
    }

    if name.contains("Bit") {
        let ident = Ident::from(_name.as_str());
        let all_bits = constants
            .iter()
            .filter_map(|constant| Constant::from_constant(constant).value())
            .fold(0, |acc, next| acc | next.bits());
        let all_bits_term = Term::intern(&format!("0b{:b}", all_bits));

        let impl_bitflags = bitflags_impl_block(ident, &_enum.name, &constants);
        if bitflags_cache.contains(&ident) {
            EnumType::Bitflags(quote! {})
        } else {
            bitflags_cache.insert(ident.clone());
            let q = quote! {
                #[repr(transparent)]
                #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
                pub struct #ident(pub(crate) Flags);
                vk_bitflags_wrapped!(#ident, #all_bits_term, Flags);
                #impl_bitflags
            };
            EnumType::Bitflags(q)
        }
    } else {
        let impl_block = bitflags_impl_block(ident, &_enum.name, &constants);
        let enum_quote = quote! {
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
            #[repr(transparent)]
            pub struct #ident(pub(crate) i32);
            impl #ident {
                pub fn from_raw(x: i32) -> Self { #ident(x) }
                pub fn as_raw(self) -> i32 { self.0 }
            }
            #impl_block
        };
        let special_quote = match _name.as_str() {
            //"StructureType" => generate_structure_type(&_name, _enum, create_info_constants),
            "Result" => generate_result(ident, _enum),
            _ => {
                quote! {}
            }
        };
        let q = quote! {
            #enum_quote
            #special_quote

        };
        EnumType::Enum(q)
    }
}

pub fn generate_result(ident: Ident, _enum: &vkxml::Enumeration) -> Tokens {
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

        let variant_ident = variant_ident(&_enum.name, variant_name);
        Some(quote! {
            #ident::#variant_ident => Some(#notation)
        })
    });

    let notation2 = notation.clone();
    quote! {
        impl ::std::error::Error for #ident {
            fn description(&self) -> &str {
                let name = match *self {
                    #(#notation),*,
                    _ => None,
                };
                name.unwrap_or("unknown error")
            }
        }
        impl fmt::Display for #ident {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                let name = match *self {
                    #(#notation2),*,
                    _ => None,
                };
                if let Some(x) = name {
                    fmt.write_str(x)
                } else {
                    write!(fmt, "{}", self.0)
                }
            }
        }
    }
}

fn is_static_array(field: &vkxml::Field) -> bool {
    field
        .array
        .as_ref()
        .map(|ty| match ty {
            vkxml::ArrayType::Static => true,
            _ => false,
        })
        .unwrap_or(false)
}
pub fn derive_default(_struct: &vkxml::Struct) -> Option<Tokens> {
    let name = name_to_tokens(&_struct.name);
    let members = _struct.elements.iter().filter_map(|elem| match *elem {
        vkxml::StructElement::Member(ref field) => Some(field),
        _ => None,
    });
    let is_structure_type = |field: &vkxml::Field| field.basetype == "VkStructureType";

    // This are also pointers, and therefor also don't implement Default. The spec
    // also doesn't mark them as pointers
    let handles = ["LPCWSTR", "HANDLE", "HINSTANCE", "HWND"];
    let contains_ptr = members.clone().any(|field| field.reference.is_some());
    let contains_strucutre_type = members.clone().any(is_structure_type);
    let contains_static_array = members.clone().any(is_static_array);
    if !(contains_ptr || contains_strucutre_type || contains_static_array) {
        return None;
    };
    let default_fields = members.clone().map(|field| {
        let param_ident = field.param_ident();
        if is_structure_type(field) {
            let ty = field
                .type_enums
                .as_ref()
                .and_then(|ty| ty.split(',').nth(0));
            if let Some(variant) = ty {
                let variant_ident = variant_ident("VkStructureType", variant);

                quote! {
                    #param_ident: StructureType::#variant_ident
                }
            } else {
                quote! {
                    #param_ident: unsafe { ::std::mem::zeroed() }
                }
            }
        } else if let Some(ref reference) = field.reference {
            match reference {
                vkxml::ReferenceType::Pointer => {
                    if field.is_const {
                        quote! {
                            #param_ident: ::std::ptr::null()
                        }
                    } else {
                        quote! {
                            #param_ident: ::std::ptr::null_mut()
                        }
                    }
                }
                vkxml::ReferenceType::PointerToPointer => {
                    quote! {
                        #param_ident: ::std::ptr::null_mut()
                    }
                }
                vkxml::ReferenceType::PointerToConstPointer => {
                    if field.is_const {
                        quote! {
                            #param_ident: ::std::ptr::null()
                        }
                    } else {
                        quote! {
                            #param_ident: ::std::ptr::null_mut()
                        }
                    }
                }
            }
        } else if is_static_array(field) || handles.contains(&field.basetype.as_str()) {
            quote! {
                #param_ident: unsafe { ::std::mem::zeroed() }
            }
        } else {
            let ty = field.type_tokens(false);
            quote! {
                #param_ident: #ty::default()
            }
        }
    });
    let q = quote! {
        impl ::std::default::Default for #name {
            fn default() -> #name {
                #name {
                    #(
                        #default_fields
                    ),*
                }
            }
        }
    };
    Some(q)
}
pub fn derive_debug(_struct: &vkxml::Struct, union_types: &HashSet<&str>) -> Option<Tokens> {
    let name = name_to_tokens(&_struct.name);
    let members = _struct.elements.iter().filter_map(|elem| match *elem {
        vkxml::StructElement::Member(ref field) => Some(field),
        _ => None,
    });
    let contains_pfn = members.clone().any(|field| {
        field
            .name
            .as_ref()
            .map(|n| n.contains("pfn"))
            .unwrap_or(false)
    });
    let contains_static_array = members
        .clone()
        .any(|x| is_static_array(x) && x.basetype == "char");
    let contains_union = members
        .clone()
        .any(|field| union_types.contains(field.basetype.as_str()));
    if !(contains_union || contains_static_array || contains_pfn) {
        return None;
    }
    let debug_fields = members.clone().map(|field| {
        let param_ident = field.param_ident();
        let param_str = param_ident.as_ref();
        let debug_value = if is_static_array(field) && field.basetype == "char" {
            quote! {
                &unsafe {
                    ::std::ffi::CStr::from_ptr(self.#param_ident.as_ptr() as *const c_char)
                }
            }
        } else if param_ident.as_ref().contains("pfn") {
            quote! {
                &(self.#param_ident.map(|x| x as *const ()))
            }
        } else if union_types.contains(field.basetype.as_str()) {
            quote!(&"union")
        } else {
            quote! {
                &self.#param_ident
            }
        };
        quote! {
            .field(#param_str, #debug_value)
        }
    });
    let name_str = name.as_ref();
    let q = quote! {
        impl fmt::Debug for #name {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                fmt.debug_struct(#name_str)
                #(#debug_fields)*
                .finish()
            }
        }
    };
    Some(q)
}

pub fn derive_setters(
    _struct: &vkxml::Struct,
    root_struct_names: &HashSet<String>,
) -> Option<Tokens> {
    if &_struct.name == "VkBaseInStructure" || &_struct.name == "VkBaseOutStructure" {
        return None;
    }

    let name = name_to_tokens(&_struct.name);
    let name_builder = name_to_tokens(&(_struct.name.clone() + "Builder"));

    let members = _struct.elements.iter().filter_map(|elem| match *elem {
        vkxml::StructElement::Member(ref field) => Some(field),
        _ => None,
    });

    let (has_next, is_next_const) = match members
        .clone()
        .find(|field| field.param_ident().to_string() == "p_next")
    {
        Some(p_next) => {
            if p_next.type_tokens(false).to_string().starts_with("*const") {
                (true, true)
            } else {
                (true, false)
            }
        }
        None => (false, false),
    };

    let nofilter_count_members = [
        "VkPipelineViewportStateCreateInfo.pViewports",
        "VkPipelineViewportStateCreateInfo.pScissors",
        "VkDescriptorSetLayoutBinding.pImmutableSamplers",
    ];
    let filter_members: Vec<String> = members
        .clone()
        .filter_map(|field| {
            let field_name = field.name.as_ref().unwrap();

            // Associated _count members
            if field.array.is_some() {
                if let Some(ref array_size) = field.size {
                    if !array_size.starts_with("latexmath")
                        && !nofilter_count_members
                            .iter()
                            .any(|n| *n == &(_struct.name.clone() + "." + field_name))
                    {
                        return Some((*array_size).clone());
                    }
                }
            }

            // VkShaderModuleCreateInfo requiers a custom setter
            if field_name == "codeSize" {
                return Some(field_name.clone());
            }

            None
        })
        .collect();

    let setters = members.clone().filter_map(|field| {
        let param_ident = field.param_ident();
        let param_ty_tokens = field.type_tokens(false);
        let param_ty_string = param_ty_tokens.to_string();

        let param_ident_string = param_ident.to_string();
        if param_ident_string == "s_type" || param_ident_string == "p_next" {
            return None;
        }

        let mut param_ident_short = param_ident_string.as_str();
        if param_ident_string.starts_with("p_") {
            param_ident_short = &param_ident_string[2..];
        };
        if param_ident_string.starts_with("pp_") {
            param_ident_short = &param_ident_string[3..];
        };
        let param_ident_short = Term::intern(&param_ident_short);

        if let Some(name) = field.name.as_ref() {
            // Fiter
            if filter_members.iter().any(|n| *n == *name) {
                return None;
            }

            // Unique cases
            if name == "pCode" {
                return Some(quote!{
                        pub fn code(mut self, code: &'a [u32]) -> #name_builder<'a> {
                            self.inner.code_size = code.len() * 4;
                            self.inner.p_code = code.as_ptr() as *const u32;
                            self
                        }
                });
            }

            if name == "pSampleMask" {
                return Some(quote!{
                        pub fn sample_mask(mut self, sample_mask: &'a [SampleMask]) -> #name_builder<'a> {
                            self.inner.p_sample_mask = sample_mask.as_ptr() as *const SampleMask;
                            self
                        }
                });
            }
        }

        // TODO: Improve in future when https://github.com/rust-lang/rust/issues/53667 is merged id:6
        if param_ident_string.starts_with("p_") || param_ident_string.starts_with("pp_") {
            if param_ty_string == "*const c_char" {
                return Some(quote!{
                        pub fn #param_ident_short(mut self, #param_ident_short: &'a ::std::ffi::CStr) -> #name_builder<'a> {
                            self.inner.#param_ident = #param_ident_short.as_ptr();
                            self
                        }
                });
            }

            if let Some(ref array_type) = field.array {
                if let Some(ref array_size) = field.size {
                    if !array_size.starts_with("latexmath") {
                        let array_size_ident = Ident::from(array_size.to_snake_case().as_str());
                        if param_ty_string == "*const *const c_char" {
                            return Some(quote!{
                                    pub fn #param_ident_short(mut self, #param_ident_short: &'a [*const c_char]) -> #name_builder<'a> {
                                        self.inner.#param_ident = #param_ident_short.as_ptr();
                                        self.inner.#array_size_ident = #param_ident_short.len() as _;
                                        self
                                    }
                            });
                        }

                        let slice_param_ty_tokens;
                        let ptr;
                        if param_ty_string.starts_with("*const ") {
                            let slice_type = &param_ty_string[7..];
                            if slice_type == "c_void" {
                                slice_param_ty_tokens = "&'a [u8]".to_string();
                                ptr = ".as_ptr() as *const c_void";
                            } else {
                                slice_param_ty_tokens = "&'a [".to_string() + slice_type + "]";
                                ptr = ".as_ptr()";
                            }
                        } else {
                            // *mut
                            let slice_type = &param_ty_string[5..];
                            if slice_type == "c_void" {
                                slice_param_ty_tokens = "&mut 'a [u8]".to_string();
                                ptr = ".as_mut_ptr() as *mut c_void";
                            } else {
                                slice_param_ty_tokens = "&'a mut [".to_string() + slice_type + "]";
                                ptr = ".as_mut_ptr()";
                            }
                        }
                        let slice_param_ty_tokens = Term::intern(&slice_param_ty_tokens);
                        let ptr = Term::intern(ptr);

                        match array_type {
                            vkxml::ArrayType::Dynamic => {
                                return Some(quote!{
                                    pub fn #param_ident_short(mut self, #param_ident_short: #slice_param_ty_tokens) -> #name_builder<'a> {
                                        self.inner.#array_size_ident = #param_ident_short.len() as _;
                                        self.inner.#param_ident = #param_ident_short#ptr;
                                        self
                                    }
                                });
                            }
                            _ => {}
                        }
                    }
                }
            }

            if param_ty_string.starts_with("*const ") {
                let slice_param_ty_tokens = "&'a ".to_string() + &param_ty_string[7..];
                let slice_param_ty_tokens = Term::intern(&slice_param_ty_tokens);
                return Some(quote!{
                        pub fn #param_ident_short(mut self, #param_ident_short: #slice_param_ty_tokens) -> #name_builder<'a> {
                            self.inner.#param_ident = #param_ident_short;
                            self
                        }
                });
            }
        }

        if param_ty_string == "Bool32" {
            return Some(quote!{
                pub fn #param_ident_short(mut self, #param_ident_short: bool) -> #name_builder<'a> {
                    self.inner.#param_ident = #param_ident_short.into();
                    self
                }
            });
        }

        Some(quote!{
            pub fn #param_ident_short(mut self, #param_ident_short: #param_ty_tokens) -> #name_builder<'a> {
                self.inner.#param_ident = #param_ident_short;
                self
            }
        })
    });

    let extends_name = name_to_tokens(&format!("Extends{}", name));

    let root_structs: Vec<Ident> = _struct
        .extends
        .as_ref()
        .map(|extends| {
            extends
                .split(',')
                .filter(|extend| root_struct_names.contains(&extend.to_string()))
                .map(|extends| name_to_tokens(&format!("Extends{}", name_to_tokens(&extends))))
                .collect()
        })
        .unwrap_or(vec![]);

    // We only implement a next methods for root structs with a `pnext` field.
    let next_function = if has_next && root_structs.is_empty() {
        quote! {
            /// Prepends the given extension struct between the root and the first pointer. This
            /// method only exists on structs that can be passed to a function directly. Only
            /// valid extension structs can be pushed into the chain.
            /// If the chain looks like `A -> B -> C`, and you call `builder.push_next(&mut D)`, then the
            /// chain will look like `A -> D -> B -> C`.
            pub fn push_next<T: #extends_name>(mut self, next: &'a mut T) -> #name_builder<'a> {
                unsafe{
                    let next_ptr = next as *mut T as *mut BaseOutStructure;
                    // `next` here can contain a pointer chain. This means that we must correctly
                    // attach he head to the root and the tail to the rest of the chain
                    // For example:
                    //
                    // next = A -> B
                    // Before: `Root -> C -> D -> E`
                    // After: `Root -> A -> B -> C -> D -> E`
                    //                 ^^^^^^
                    //                 next chain
                    let last_next = ptr_chain_iter(next).last().unwrap();
                    (*last_next).p_next = self.inner.p_next as _;
                    self.inner.p_next = next_ptr as _;
                }
                self
            }
        }
    } else {
        quote! {}
    };

    // Root structs come with their own trait that structs that extends this struct will
    // implement
    let next_trait = if has_next && _struct.extends.is_none() {
        quote! {
            pub unsafe trait #extends_name {
            }
        }
    } else {
        quote! {}
    };

    // If the struct extends something we need to implement the trait.
    let impl_extend_trait = root_structs.iter().map(|extends| {
        quote! {
            unsafe impl #extends for #name_builder<'_> {
            }
            unsafe impl #extends for #name {
            }
        }
    });

    let q = quote! {
        impl #name {
            pub fn builder<'a>() -> #name_builder<'a> {
                #name_builder {
                    inner: #name::default(),
                    marker: ::std::marker::PhantomData,
                }
            }
        }

        #[repr(transparent)]
        pub struct #name_builder<'a> {
            inner: #name,
            marker: ::std::marker::PhantomData<&'a ()>,
        }

        #(#impl_extend_trait)*
        #next_trait


        impl<'a> ::std::ops::Deref for #name_builder<'a> {
            type Target = #name;

            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        impl<'a> ::std::ops::DerefMut for #name_builder<'a> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.inner
            }
        }

        impl<'a> #name_builder<'a> {
            #(#setters)*

            #next_function

            /// Calling build will **discard** all the lifetime information. Only call this if
            /// necessary! Builders implement `Deref` targeting their corresponding Vulkan struct,
            /// so references to builders can be passed directly to Vulkan functions.
            pub fn build(self) -> #name {
                self.inner
            }
        }
    };

    Some(q)
}

/// At the moment `Ash` doesn't properly derive all the necessary drives
/// like Eq, Hash etc.
/// To Address some cases, you can add the name of the struct that you
/// require and add the missing derives yourself.
pub fn manual_derives(_struct: &vkxml::Struct) -> Tokens {
    match _struct.name.as_str() {
        "VkExtent3D" | "VKExtent2D" => quote! {PartialEq, Eq, Hash,},
        _ => quote! {},
    }
}
pub fn generate_struct(
    _struct: &vkxml::Struct,
    root_struct_names: &HashSet<String>,
    union_types: &HashSet<&str>,
) -> Tokens {
    let name = name_to_tokens(&_struct.name);
    let members = _struct.elements.iter().filter_map(|elem| match *elem {
        vkxml::StructElement::Member(ref field) => Some(field),
        _ => None,
    });

    let params = members.clone().map(|field| {
        let param_ident = field.param_ident();
        let param_ty_tokens = field.type_tokens(false);
        quote! {pub #param_ident: #param_ty_tokens}
    });

    let debug_tokens = derive_debug(_struct, union_types);
    let default_tokens = derive_default(_struct);
    let setter_tokens = derive_setters(_struct, root_struct_names);
    let manual_derive_tokens = manual_derives(_struct);
    let dbg_str = if debug_tokens.is_none() {
        quote!(Debug,)
    } else {
        quote!()
    };
    let default_str = if default_tokens.is_none() {
        quote!(Default,)
    } else {
        quote!()
    };
    quote! {
        #[repr(C)]
        #[derive(Copy, Clone, #default_str #dbg_str #manual_derive_tokens)]
        pub struct #name {
            #(#params,)*
        }
        #debug_tokens
        #default_tokens
        #setter_tokens
    }
}

pub fn generate_handle(handle: &vkxml::Handle) -> Option<Tokens> {
    if handle.name == "" {
        return None;
    }
    let tokens = match handle.ty {
        vkxml::HandleType::Dispatch => {
            let name = &handle.name[2..];
            let ty = Ident::from(name.to_shouty_snake_case());
            let name = Ident::from(name);
            quote! {
                define_handle!(#name, #ty);
            }
        }
        vkxml::HandleType::NoDispatch => {
            let name = &handle.name[2..];
            let ty = Ident::from(name.to_shouty_snake_case());
            let name = Ident::from(name);
            quote! {
                handle_nondispatchable!(#name, #ty);
            }
        }
    };
    Some(tokens)
}
fn generate_funcptr(fnptr: &vkxml::FunctionPointer) -> Tokens {
    let name = Ident::from(fnptr.name.as_str());
    let ret_ty_tokens = fnptr.return_type.type_tokens(true);
    let params = fnptr.param.iter().map(|field| {
        let ident = field.param_ident();
        let type_tokens = field.type_tokens(true);
        quote! {
            #ident: #type_tokens
        }
    });
    quote! {
        #[allow(non_camel_case_types)]
        pub type #name = Option<unsafe extern "system" fn(#(#params),*) -> #ret_ty_tokens>;
    }
}

fn generate_union(union: &vkxml::Union) -> Tokens {
    let name = to_type_tokens(&union.name, None);
    let fields = union.elements.iter().map(|field| {
        let name = field.param_ident();
        let ty = field.type_tokens(false);
        quote! {
            pub #name: #ty
        }
    });
    quote! {
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub union #name {
            #(#fields),*
        }
        impl ::std::default::Default for #name {
            fn default() -> #name {
                unsafe { ::std::mem::zeroed() }
            }
        }
    }
}
pub fn root_struct_names(definitions: &[&vkxml::DefinitionsElement]) -> HashSet<String> {
    definitions
        .iter()
        .filter_map(|definition| match *definition {
            vkxml::DefinitionsElement::Struct(ref _struct) => {
                let is_root_struct = _struct.extends.is_none();
                if is_root_struct {
                    Some(_struct.name.clone())
                } else {
                    None
                }
            }
            _ => None,
        })
        .collect()
}
pub fn generate_definition(
    definition: &vkxml::DefinitionsElement,
    union_types: &HashSet<&str>,
    root_structs: &HashSet<String>,
    bitflags_cache: &mut HashSet<Ident>,
) -> Option<Tokens> {
    match *definition {
        vkxml::DefinitionsElement::Typedef(ref typedef) => Some(generate_typedef(typedef)),
        vkxml::DefinitionsElement::Struct(ref _struct) => {
            Some(generate_struct(_struct, root_structs, union_types))
        }
        vkxml::DefinitionsElement::Bitmask(ref mask) => generate_bitmask(mask, bitflags_cache),
        vkxml::DefinitionsElement::Handle(ref handle) => generate_handle(handle),
        vkxml::DefinitionsElement::FuncPtr(ref fp) => Some(generate_funcptr(fp)),
        vkxml::DefinitionsElement::Union(ref union) => Some(generate_union(union)),
        _ => None,
    }
}
pub fn generate_feature<'a>(
    feature: &vkxml::Feature,
    commands: &CommandMap<'a>,
    fn_cache: &mut HashSet<&'a str>,
) -> quote::Tokens {
    let (static_commands, entry_commands, device_commands, instance_commands) = feature
        .elements
        .iter()
        .flat_map(|feature| {
            if let vkxml::FeatureElement::Require(ref spec) = feature {
                spec.elements
                    .iter()
                    .filter_map(|feature_spec| {
                        if let vkxml::FeatureReference::CommandReference(ref cmd_ref) = feature_spec
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
        generate_function_pointers(Ident::from("StaticFn"), &static_commands, fn_cache)
    } else {
        quote! {}
    };
    let entry = generate_function_pointers(
        Ident::from(format!("EntryFnV{}", version).as_str()),
        &entry_commands,
        fn_cache,
    );
    let instance = generate_function_pointers(
        Ident::from(format!("InstanceFnV{}", version).as_str()),
        &instance_commands,
        fn_cache,
    );
    let device = generate_function_pointers(
        Ident::from(format!("DeviceFnV{}", version).as_str()),
        &device_commands,
        fn_cache,
    );
    quote! {
        #static_fn
        #entry
        #instance
        #device
    }
}
pub fn constant_name(name: &str) -> String {
    name.replace("VK_", "")
}

pub fn generate_constant<'a>(
    constant: &'a vkxml::Constant,
    cache: &mut HashSet<&'a str>,
) -> Tokens {
    cache.insert(constant.name.as_str());
    let c = Constant::from_constant(constant);
    let name = constant_name(&constant.name);
    let ident = Ident::from(name.as_str());
    let value = c.to_tokens();
    let ty = if name == "TRUE" || name == "FALSE" {
        CType::Bool32
    } else {
        c.ty()
    };
    let ty = ty.to_tokens();
    quote! {
        pub const #ident: #ty = #value;
    }
}

pub fn generate_feature_extension<'a>(
    registry: &'a vk_parse::Registry,
    const_cache: &mut HashSet<&'a str>,
    const_values: &mut BTreeMap<Ident, Vec<Ident>>,
) -> Tokens {
    let constants = registry.0.iter().filter_map(|item| match item {
        vk_parse::RegistryChild::Feature(feature) => Some(generate_extension_constants(
            &feature.name,
            0,
            &feature.children,
            const_cache,
            const_values,
        )),
        _ => None,
    });
    quote! {
        #(#constants)*
    }
}

pub fn generate_const_displays<'a>(const_values: &BTreeMap<Ident, Vec<Ident>>) -> Tokens {
    let impls = const_values
        .iter()
        .filter(|(ty, _)| *ty != "Result")
        .map(|(ty, values)| {
            if ty.to_string().contains("Flags") {
                let cases = values.iter().map(|value| {
                    let name = value.to_string();
                    quote! { (#ty::#value.0, #name) }
                });
                quote! {
                    impl fmt::Display for #ty {
                        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                            const KNOWN: &[(Flags, &str)] = &[#(#cases),*];
                            display_flags(f, KNOWN, self.0)
                        }
                    }
                }
            } else {
                let cases = values.iter().map(|value| {
                    let name = value.to_string();
                    quote! { Self::#value => Some(#name), }
                });
                quote! {
                    impl fmt::Display for #ty {
                        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                            let name = match *self {
                                #(#cases)*
                                _ => None,
                            };
                            if let Some(x) = name {
                                f.write_str(x)
                            } else {
                                write!(f, "{}", self.0)
                            }
                        }
                    }
                }
            }
        });
    quote! {
        fn display_flags(f: &mut fmt::Formatter, known: &[(Flags, &'static str)], value: Flags) -> fmt::Result {
            let mut first = true;
            let mut accum = value;
            for (bit, name) in known {
                if *bit != 0 && accum & *bit == *bit {
                    if !first { f.write_str(" | ")?; }
                    f.write_str(name)?;
                    first = false;
                    accum &= !bit;
                }
            }
            if accum != 0 {
                if !first { f.write_str(" | ")?; }
                write!(f, "{:b}", accum)?;
            }
            Ok(())
        }

        #(#impls)*
    }
}
pub fn generate_aliases_of_types<'a>(
    types: &'a vk_parse::Types,
    ty_cache: &mut HashSet<Ident>,
) -> Tokens {
    let aliases = types
        .children
        .iter()
        .filter_map(|child| match child {
            vk_parse::TypesChild::Type(ty) => Some((ty.name.as_ref()?, ty.alias.as_ref()?)),
            _ => None,
        })
        .filter_map(|(name, alias)| {
            let name_ident = name_to_tokens(name);
            if ty_cache.contains(&name_ident) {
                return None;
            };
            ty_cache.insert(name_ident.clone());
            let alias_ident = name_to_tokens(alias);
            let tokens = quote! {
                pub type #name_ident = #alias_ident;
            };
            Some(tokens)
        });
    quote! {
        #(#aliases)*
    }
}
pub fn write_source_code(path: &Path) {
    use std::fs::File;
    use std::io::Write;
    let spec2 = vk_parse::parse_file(path);
    let extensions: &Vec<vk_parse::Extension> = spec2
        .0
        .iter()
        .filter_map(|item| match item {
            vk_parse::RegistryChild::Extensions(ref ext) => Some(&ext.children),
            _ => None,
        })
        .nth(0)
        .expect("extension");
    let mut ty_cache = HashSet::new();
    let aliases: Vec<_> = spec2
        .0
        .iter()
        .filter_map(|item| match item {
            vk_parse::RegistryChild::Types(ref ty) => {
                Some(generate_aliases_of_types(ty, &mut ty_cache))
            }
            _ => None,
        })
        .collect();

    let spec = vk_parse::parse_file_as_vkxml(path);
    let commands: HashMap<vkxml::Identifier, &vkxml::Command> = spec
        .elements
        .iter()
        .filter_map(|elem| match elem {
            vkxml::RegistryElement::Commands(ref cmds) => Some(cmds),
            _ => None,
        })
        .flat_map(|cmds| cmds.elements.iter().map(|cmd| (cmd.name.clone(), cmd)))
        .collect();

    let features: Vec<&vkxml::Feature> = spec
        .elements
        .iter()
        .filter_map(|elem| match elem {
            vkxml::RegistryElement::Features(ref features) => Some(features),
            _ => None,
        })
        .flat_map(|features| features.elements.iter())
        .collect();

    let definitions: Vec<&vkxml::DefinitionsElement> = spec
        .elements
        .iter()
        .filter_map(|elem| match elem {
            vkxml::RegistryElement::Definitions(ref definitions) => Some(definitions),
            _ => None,
        })
        .flat_map(|definitions| definitions.elements.iter())
        .collect();

    let enums: Vec<&vkxml::Enumeration> = spec
        .elements
        .iter()
        .filter_map(|elem| match elem {
            vkxml::RegistryElement::Enums(ref enums) => Some(enums),
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
            vkxml::RegistryElement::Constants(ref constants) => Some(constants),
            _ => None,
        })
        .flat_map(|constants| constants.elements.iter())
        .collect();

    let mut fn_cache = HashSet::new();
    let mut bitflags_cache = HashSet::new();
    let mut const_cache = HashSet::new();

    let mut const_values: BTreeMap<Ident, Vec<Ident>> = BTreeMap::new();

    let (enum_code, bitflags_code) = enums
        .into_iter()
        .map(|e| generate_enum(e, &mut const_cache, &mut const_values, &mut bitflags_cache))
        .fold((Vec::new(), Vec::new()), |mut acc, elem| {
            match elem {
                EnumType::Enum(token) => acc.0.push(token),
                EnumType::Bitflags(token) => acc.1.push(token),
            };
            acc
        });

    let constants_code: Vec<_> = constants
        .iter()
        .map(|constant| generate_constant(constant, &mut const_cache))
        .collect();
    let extension_code = extensions
        .iter()
        .filter_map(|ext| {
            generate_extension(
                ext,
                &commands,
                &mut const_cache,
                &mut const_values,
                &mut fn_cache,
            )
        })
        .collect_vec();

    let union_types = definitions
        .iter()
        .filter_map(|def| match def {
            vkxml::DefinitionsElement::Union(ref union) => Some(union.name.as_str()),
            _ => None,
        })
        .collect::<HashSet<&str>>();

    let root_names = root_struct_names(&definitions);
    let definition_code: Vec<_> = definitions
        .into_iter()
        .filter_map(|def| generate_definition(def, &union_types, &root_names, &mut bitflags_cache))
        .collect();

    let feature_code: Vec<_> = features
        .iter()
        .map(|feature| generate_feature(feature, &commands, &mut fn_cache))
        .collect();
    let feature_extensions_code =
        generate_feature_extension(&spec2, &mut const_cache, &mut const_values);

    let const_displays = generate_const_displays(&const_values);

    let mut file = File::create("../ash/src/vk.rs").expect("vk");
    let bitflags_macro = vk_bitflags_wrapped_macro();
    let handle_nondispatchable_macro = handle_nondispatchable_macro();
    let define_handle_macro = define_handle_macro();
    let version_macros = vk_version_macros();
    let platform_specific_types = platform_specific_types();
    let source_code = quote! {
        use std::fmt;
        use std::os::raw::*;
        /// Iterates through the pointer chain. Includes the item that is passed into the function.
        /// Stops at the last `BaseOutStructure` that has a null `p_next` field.
        pub(crate) unsafe fn ptr_chain_iter<T>(
            ptr: &mut T,
        ) -> impl Iterator<Item = *mut BaseOutStructure> {
            use std::ptr::null_mut;
            let ptr: *mut BaseOutStructure = ptr as *mut T as _;
            (0..).scan(ptr, |p_ptr, _| {
                if *p_ptr == null_mut() {
                    return None;
                }
                let n_ptr = (**p_ptr).p_next as *mut BaseOutStructure;
                let old = *p_ptr;
                *p_ptr = n_ptr;
                Some(old)
            })
        }

        pub trait Handle {
            const TYPE: ObjectType;
            fn as_raw(self) -> u64;
            fn from_raw(u64) -> Self;
        }

        #version_macros
        #platform_specific_types
        #bitflags_macro
        #handle_nondispatchable_macro
        #define_handle_macro
        #(#feature_code)*
        #(#definition_code)*
        #(#enum_code)*
        #(#bitflags_code)*
        #(#constants_code)*
        #(#extension_code)*
        #feature_extensions_code
        #const_displays
        #(#aliases)*
    };
    write!(&mut file, "{}", source_code).expect("Unable to write to file");
}
