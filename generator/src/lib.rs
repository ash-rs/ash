#![recursion_limit = "256"]
#![warn(trivial_casts, trivial_numeric_casts)]

use heck::{ToShoutySnakeCase, ToSnakeCase, ToUpperCamelCase};
use itertools::Itertools;
use nom::sequence::pair;
use nom::{
    branch::alt,
    bytes::streaming::tag,
    character::complete::{char, digit1, hex_digit1, multispace1, none_of, one_of},
    combinator::{complete, map, opt, value},
    error::{ParseError, VerboseError},
    multi::many1,
    sequence::{delimited, preceded, terminated},
    IResult, Parser,
};
use once_cell::sync::Lazy;
use proc_macro2::{Delimiter, Group, Literal, Span, TokenStream, TokenTree};
use quote::*;
use regex::Regex;
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fmt::Display,
    path::Path,
};
use syn::Ident;

macro_rules! get_variant {
    ($variant:path) => {
        |enum_| match enum_ {
            $variant(inner) => Some(inner),
            _ => None,
        }
    };
    ($variant:path { $($member:ident),+ }) => {
        |enum_| match enum_ {
            $variant { $($member),+, .. } => Some(( $($member),+ )),
            _ => None,
        }
    };
}

const BACKWARDS_COMPATIBLE_ALIAS_COMMENT: &str = "Backwards-compatible alias containing a typo";

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
    fn to_string(self) -> &'static str {
        match self {
            Self::USize => "usize",
            Self::U32 => "u32",
            Self::U64 => "u64",
            Self::Float => "f32",
            Self::Bool32 => "Bool32",
        }
    }
}

impl quote::ToTokens for CType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        format_ident!("{}", self.to_string()).to_tokens(tokens);
    }
}

fn parse_ctype<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, CType, E> {
    (alt((
        value(CType::U64, complete(tag("ULL"))),
        value(CType::U32, complete(tag("U"))),
    )))(i)
}

fn parse_cexpr<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, (CType, String), E> {
    (alt((
        map(parse_cfloat, |f| (CType::Float, format!("{:.2}", f))),
        parse_inverse_number,
        parse_decimal_number,
        parse_hexadecimal_number,
    )))(i)
}

fn parse_cfloat<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, f32, E> {
    (terminated(nom::number::complete::float, one_of("fF")))(i)
}

fn parse_inverse_number<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, (CType, String), E> {
    (map(
        delimited(
            char('('),
            pair(
                preceded(char('~'), parse_decimal_number),
                opt(preceded(char('-'), digit1)),
            ),
            char(')'),
        ),
        |((ctyp, num), minus_num)| {
            let expr = if let Some(minus) = minus_num {
                format!("!{}-{}", num, minus)
            } else {
                format!("!{}", num)
            };
            (ctyp, expr)
        },
    ))(i)
}

// Like a C string, but does not support quote escaping and expects at least one character.
// If needed, use https://github.com/Geal/nom/blob/8e09f0c3029d32421b5b69fb798cef6855d0c8df/tests/json.rs#L61-L81
fn parse_c_include_string<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, String, E> {
    (delimited(
        char('"'),
        map(many1(none_of("\"")), |c| {
            c.iter().map(char::to_string).join("")
        }),
        char('"'),
    ))(i)
}

fn parse_c_include<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, String, E> {
    (preceded(
        tag("#include"),
        preceded(multispace1, parse_c_include_string),
    ))(i)
}

fn parse_decimal_number<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, (CType, String), E> {
    (map(
        pair(digit1.map(str::to_string), parse_ctype),
        |(dig, ctype)| (ctype, dig),
    ))(i)
}

fn parse_hexadecimal_number<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, (CType, String), E> {
    (preceded(
        alt((tag("0x"), tag("0X"))),
        map(pair(hex_digit1, parse_ctype), |(num, typ)| {
            (
                typ,
                format!("0x{}{}", num.to_ascii_lowercase(), typ.to_string()),
            )
        }),
    ))(i)
}

fn khronos_link<S: Display + ?Sized>(name: &S) -> Literal {
    Literal::string(&format!(
        "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/{name}.html>",
        name = name
    ))
}

fn is_opaque_type(ty: &str) -> bool {
    matches!(
        ty,
        "void"
            | "wl_display"
            | "wl_surface"
            | "Display"
            | "xcb_connection_t"
            | "ANativeWindow"
            | "AHardwareBuffer"
            | "CAMetalLayer"
            | "IDirectFB"
            | "IDirectFBSurface"
    )
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
    fn constant(&self, enum_name: &str) -> Constant;
    fn variant_ident(&self, enum_name: &str) -> Ident;
    fn notation(&self) -> Option<&str>;
    fn is_alias(&self) -> bool {
        false
    }
    fn doc_attribute(&self) -> TokenStream {
        assert_ne!(
            self.notation(),
            Some(BACKWARDS_COMPATIBLE_ALIAS_COMMENT),
            "Backwards-compatible constants should not be emitted"
        );
        match self.notation() {
            Some(n) if n.starts_with("Alias") => {
                quote!(#[deprecated = #n])
            }
            Some(n) => quote!(#[doc = #n]),
            None => quote!(),
        }
    }
}

impl ConstantExt for vkxml::ExtensionEnum {
    fn constant(&self, _enum_name: &str) -> Constant {
        Constant::from_extension_enum(self).unwrap()
    }
    fn variant_ident(&self, enum_name: &str) -> Ident {
        variant_ident(enum_name, &self.name)
    }
    fn notation(&self) -> Option<&str> {
        self.notation.as_deref()
    }
}

impl ConstantExt for vk_parse::Enum {
    fn constant(&self, enum_name: &str) -> Constant {
        Constant::from_vk_parse_enum(self, Some(enum_name), None)
            .unwrap()
            .0
    }
    fn variant_ident(&self, enum_name: &str) -> Ident {
        variant_ident(enum_name, &self.name)
    }
    fn notation(&self) -> Option<&str> {
        self.comment.as_deref()
    }
    fn is_alias(&self) -> bool {
        matches!(self.spec, vk_parse::EnumSpec::Alias { .. })
    }
}

impl ConstantExt for vkxml::Constant {
    fn constant(&self, _enum_name: &str) -> Constant {
        Constant::from_constant(self)
    }
    fn variant_ident(&self, enum_name: &str) -> Ident {
        variant_ident(enum_name, &self.name)
    }
    fn notation(&self) -> Option<&str> {
        self.notation.as_deref()
    }
}

#[derive(Clone, Debug)]
pub enum Constant {
    Number(i32),
    Hex(String),
    BitPos(u32),
    CExpr(vkxml::CExpression),
    Text(String),
    Alias(Ident),
}

impl quote::ToTokens for Constant {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match *self {
            Constant::Number(n) => {
                let number = interleave_number('_', 3, &n.to_string());
                syn::LitInt::new(&number, Span::call_site()).to_tokens(tokens);
            }
            Constant::Hex(ref s) => {
                let number = interleave_number('_', 4, s);
                syn::LitInt::new(&format!("0x{}", number), Span::call_site()).to_tokens(tokens);
            }
            Constant::Text(ref text) => text.to_tokens(tokens),
            Constant::CExpr(ref expr) => {
                let (_, (_, rexpr)) =
                    parse_cexpr::<VerboseError<&str>>(expr).expect("Unable to parse cexpr");
                tokens.extend(rexpr.parse::<TokenStream>());
            }
            Constant::BitPos(pos) => {
                let value = 1u64 << pos;
                let bit_string = format!("{:b}", value);
                let bit_string = interleave_number('_', 4, &bit_string);
                syn::LitInt::new(&format!("0b{}", bit_string), Span::call_site()).to_tokens(tokens);
            }
            Constant::Alias(ref value) => tokens.extend(quote!(Self::#value)),
        }
    }
}

impl quote::ToTokens for ConstVal {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            ConstVal::U32(n) => n.to_tokens(tokens),
            ConstVal::U64(n) => n.to_tokens(tokens),
            ConstVal::Float(f) => f.to_tokens(tokens),
        }
    }
}

// Interleaves a number, for example 100000 => 100_000. Mostly used to make clippy happy
fn interleave_number(symbol: char, count: usize, n: &str) -> String {
    let number: String = n
        .chars()
        .rev()
        .enumerate()
        .fold(String::new(), |mut acc, (idx, next)| {
            if idx != 0 && idx % count == 0 {
                acc.push(symbol);
            }
            acc.push(next);
            acc
        });
    number.chars().rev().collect()
}
impl Constant {
    pub fn value(&self) -> Option<ConstVal> {
        match *self {
            Constant::Number(n) => Some(ConstVal::U64(n as u64)),
            Constant::Hex(ref hex) => u64::from_str_radix(hex, 16).ok().map(ConstVal::U64),
            Constant::BitPos(pos) => Some(ConstVal::U64(1u64 << pos)),
            _ => None,
        }
    }

    pub fn ty(&self) -> CType {
        match self {
            Constant::Number(_) | Constant::Hex(_) => CType::USize,
            Constant::CExpr(expr) => {
                let (_, (ty, _)) =
                    parse_cexpr::<VerboseError<&str>>(expr).expect("Unable to parse cexpr");
                ty
            }
            _ => unimplemented!(),
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

    /// Returns (Constant, optional base type, is_alias)
    pub fn from_vk_parse_enum(
        enum_: &vk_parse::Enum,
        enum_name: Option<&str>,
        extension_number: Option<i64>,
    ) -> Option<(Self, Option<String>, bool)> {
        use vk_parse::EnumSpec;

        match &enum_.spec {
            EnumSpec::Bitpos { bitpos, extends } => {
                Some((Self::BitPos(*bitpos as u32), extends.clone(), false))
            }
            EnumSpec::Offset {
                offset,
                extends,
                extnumber,
                dir: positive,
            } => {
                let ext_base = 1_000_000_000;
                let ext_block_size = 1000;
                let extnumber = extnumber
                    .or(extension_number)
                    .expect("Need an extension number");
                let value = ext_base + (extnumber - 1) * ext_block_size + offset;
                let value = if *positive { value } else { -value };
                Some((Self::Number(value as i32), Some(extends.clone()), false))
            }
            EnumSpec::Value { value, extends } => {
                let value = value
                    .strip_prefix("0x")
                    .map(|hex| Self::Hex(hex.to_owned()))
                    .or_else(|| value.parse::<i32>().ok().map(Self::Number))?;
                Some((value, extends.clone(), false))
            }
            EnumSpec::Alias { alias, extends } => {
                let base_type = extends.as_deref().or(enum_name)?;
                let key = variant_ident(base_type, alias);
                if key == "DISPATCH_BASE" {
                    None
                } else {
                    Some((Self::Alias(key), Some(base_type.to_owned()), true))
                }
            }
            _ => None,
        }
    }
}

pub trait FeatureExt {
    fn version_string(&self) -> String;
    fn is_version(&self, major: u32, minor: u32) -> bool;
}
impl FeatureExt for vkxml::Feature {
    fn is_version(&self, major: u32, minor: u32) -> bool {
        let self_major = self.version as u32;
        let self_minor = (self.version * 10.0) as u32 - self_major * 10;
        major == self_major && self_minor == minor
    }
    fn version_string(&self) -> String {
        let mut version = format!("{}", self.version);
        if version.len() == 1 {
            version = format!("{}_0", version)
        }

        version.replace('.', "_")
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
        format_ident!("{}", self.name.strip_prefix("vk").unwrap().to_snake_case())
    }

    fn function_type(&self) -> FunctionType {
        let is_first_param_device = self
            .param
            .get(0)
            .map(|field| {
                matches!(
                    field.basetype.as_str(),
                    "VkDevice" | "VkCommandBuffer" | "VkQueue"
                )
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
    /// Returns the name of the parameter that doesn't clash with Rusts reserved
    /// keywords
    fn param_ident(&self) -> Ident;

    /// The inner type of this field, with one level of pointers removed
    fn inner_type_tokens(&self) -> TokenStream;

    /// Returns reference-types wrapped in their safe variant. (Dynamic) arrays become
    /// slices, pointers become Rust references.
    fn safe_type_tokens(&self, lifetime: TokenStream) -> TokenStream;

    /// Returns the basetype ident and removes the 'Vk' prefix. When `is_ffi_param` is `true`
    /// array types (e.g. `[f32; 3]`) will be converted to pointer types (e.g. `&[f32; 3]`),
    /// which is needed for `C` function parameters. Set to `false` for struct definitions.
    fn type_tokens(&self, is_ffi_param: bool) -> TokenStream;

    /// Whether this is C's `void` type (not to be mistaken with a void _pointer_!)
    fn is_void(&self) -> bool;

    /// Exceptions for pointers to static-sized arrays,
    /// `vk.xml` does not annotate this.
    fn is_pointer_to_static_sized_array(&self) -> bool;
}

pub trait ToTokens {
    fn to_tokens(&self, is_const: bool) -> TokenStream;
    /// Returns the topmost pointer as safe reference
    fn to_safe_tokens(&self, is_const: bool, lifetime: TokenStream) -> TokenStream;
}
impl ToTokens for vkxml::ReferenceType {
    fn to_tokens(&self, is_const: bool) -> TokenStream {
        let r = if is_const {
            quote!(*const)
        } else {
            quote!(*mut)
        };
        match self {
            vkxml::ReferenceType::Pointer => quote!(#r),
            vkxml::ReferenceType::PointerToPointer => quote!(#r *mut),
            vkxml::ReferenceType::PointerToConstPointer => quote!(#r *const),
        }
    }

    fn to_safe_tokens(&self, is_const: bool, lifetime: TokenStream) -> TokenStream {
        let r = if is_const {
            quote!(&#lifetime)
        } else {
            quote!(&#lifetime mut)
        };
        match self {
            vkxml::ReferenceType::Pointer => quote!(#r),
            vkxml::ReferenceType::PointerToPointer => quote!(#r *mut),
            vkxml::ReferenceType::PointerToConstPointer => quote!(#r *const),
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
        "double" => "f64",
        "long" => "c_ulong",
        _ => type_name.strip_prefix("Vk").unwrap_or(type_name),
    };
    let new_name = new_name.replace("FlagBits", "Flags");
    format_ident!("{}", new_name.as_str())
}

/// Parses and rewrites a C literal into Rust
///
/// If no special pattern is recognized the original literal is returned.
/// Any new conversions need to be added to the [`parse_cexpr()`] [`nom`] parser.
///
/// Examples:
/// - `0x3FFU` -> `0x3ffu32`
fn convert_c_literal(lit: Literal) -> Literal {
    if let Ok((_, (_, rexpr))) = parse_cexpr::<VerboseError<&str>>(&lit.to_string()) {
        // lit::SynInt uses the same `.parse` method to create hexadecimal
        // literals because there is no `Literal` constructor for it.
        let mut stream = rexpr.parse::<TokenStream>().unwrap().into_iter();
        // If expression rewriting succeeds this should parse into a single literal
        match (stream.next(), stream.next()) {
            (Some(TokenTree::Literal(l)), None) => l,
            x => panic!("Stream must contain a single literal, not {:?}", x),
        }
    } else {
        lit
    }
}

/// Parse and yield a C expression that is valid to write in Rust
/// Identifiers are replaced with their Rust vk equivalent.
///
/// Examples:
/// - `VK_MAKE_VERSION(1, 2, VK_HEADER_VERSION)` -> `make_version(1, 2, HEADER_VERSION)`
/// - `2*VK_UUID_SIZE` -> `2 * UUID_SIZE`
fn convert_c_expression(c_expr: &str, identifier_renames: &BTreeMap<String, Ident>) -> TokenStream {
    fn rewrite_token_stream(
        stream: TokenStream,
        identifier_renames: &BTreeMap<String, Ident>,
    ) -> TokenStream {
        stream
            .into_iter()
            .map(|tt| match tt {
                TokenTree::Group(group) => TokenTree::Group(Group::new(
                    group.delimiter(),
                    rewrite_token_stream(group.stream(), identifier_renames),
                )),
                TokenTree::Ident(term) => {
                    let name = term.to_string();
                    identifier_renames
                        .get(&name)
                        .cloned()
                        .unwrap_or_else(|| format_ident!("{}", constant_name(&name)))
                        .into()
                }
                TokenTree::Literal(lit) => TokenTree::Literal(convert_c_literal(lit)),
                tt => tt,
            })
            .collect::<TokenStream>()
    }
    let c_expr = c_expr
        .parse()
        .unwrap_or_else(|_| panic!("Failed to parse `{}` as Rust", c_expr));
    rewrite_token_stream(c_expr, identifier_renames)
}

fn discard_outmost_delimiter(stream: TokenStream) -> TokenStream {
    let stream = stream.into_iter().collect_vec();
    // Discard the delimiter if this stream consists of a single top-most group
    if let [TokenTree::Group(group)] = stream.as_slice() {
        TokenTree::Group(Group::new(Delimiter::None, group.stream())).into()
    } else {
        stream.into_iter().collect::<TokenStream>()
    }
}

impl FieldExt for vkxml::Field {
    fn param_ident(&self) -> Ident {
        let name = self.name.as_deref().unwrap_or("field");
        let name_corrected = match name {
            "type" => "ty",
            _ => name,
        };
        format_ident!("{}", name_corrected.to_snake_case().as_str())
    }

    fn inner_type_tokens(&self) -> TokenStream {
        assert!(!self.is_void());
        let ty = name_to_tokens(&self.basetype);

        match self.reference {
            Some(vkxml::ReferenceType::PointerToPointer) => quote!(*mut #ty),
            Some(vkxml::ReferenceType::PointerToConstPointer) => quote!(*const #ty),
            _ => quote!(#ty),
        }
    }

    fn safe_type_tokens(&self, lifetime: TokenStream) -> TokenStream {
        assert!(!self.is_void());
        match self.array {
            // The outer type fn type_tokens() returns is [], which fits our "safe" prescription
            Some(vkxml::ArrayType::Static) => self.type_tokens(false),
            Some(vkxml::ArrayType::Dynamic) => {
                let ty = self.inner_type_tokens();
                quote!([#ty])
            }
            None => {
                let ty = name_to_tokens(&self.basetype);
                let pointer = self
                    .reference
                    .as_ref()
                    .map(|r| r.to_safe_tokens(self.is_const, lifetime));
                quote!(#pointer #ty)
            }
        }
    }

    fn type_tokens(&self, is_ffi_param: bool) -> TokenStream {
        assert!(!self.is_void());
        let ty = name_to_tokens(&self.basetype);

        match self.array {
            Some(vkxml::ArrayType::Static) => {
                assert!(self.reference.is_none());
                let size = self
                    .size
                    .as_ref()
                    .or(self.size_enumref.as_ref())
                    .expect("Should have size");
                // Make sure we also rename the constant, that is
                // used inside the static array
                let size = convert_c_expression(size, &BTreeMap::new());
                // arrays in c are always passed as a pointer
                if is_ffi_param {
                    quote!(*const [#ty; #size])
                } else {
                    quote!([#ty; #size])
                }
            }
            _ => {
                let pointer = self.reference.as_ref().map(|r| r.to_tokens(self.is_const));
                if self.is_pointer_to_static_sized_array() {
                    let size = self.c_size.as_ref().expect("Should have c_size");
                    let size = convert_c_expression(size, &BTreeMap::new());
                    quote!(#pointer [#ty; #size])
                } else {
                    quote!(#pointer #ty)
                }
            }
        }
    }

    fn is_void(&self) -> bool {
        self.basetype == "void" && self.reference.is_none()
    }

    fn is_pointer_to_static_sized_array(&self) -> bool {
        matches!(self.array, Some(vkxml::ArrayType::Dynamic))
            && self.name.as_deref() == Some("pVersionData")
    }
}

impl FieldExt for vk_parse::TypeMemberDefinition {
    fn param_ident(&self) -> Ident {
        let name = self
            .markup
            .iter()
            .find_map(get_variant!(vk_parse::TypeMemberMarkup::Name))
            .expect("Type Member must have a name!")
            .as_str();
        let name_corrected = match name {
            "type" => "ty",
            _ => name,
        };
        format_ident!("{}", name_corrected.to_snake_case().as_str())
    }

    fn inner_type_tokens(&self) -> TokenStream {
        todo!()
    }

    fn safe_type_tokens(&self, _lifetime: TokenStream) -> TokenStream {
        todo!()
    }

    fn type_tokens(&self, _is_ffi_param: bool) -> TokenStream {
        todo!()
    }

    fn is_void(&self) -> bool {
        todo!()
    }

    fn is_pointer_to_static_sized_array(&self) -> bool {
        todo!()
    }
}

pub type CommandMap<'a> = HashMap<vkxml::Identifier, &'a vkxml::Command>;

fn generate_function_pointers<'a>(
    ident: Ident,
    commands: &[&'a vkxml::Command],
    aliases: &HashMap<String, String>,
    fn_cache: &mut HashSet<&'a str>,
) -> TokenStream {
    // Commands can have duplicates inside them because they are declared per features. But we only
    // really want to generate one function pointer.
    let commands = commands
        .iter()
        .unique_by(|cmd| cmd.name.as_str())
        .collect::<Vec<_>>();

    struct Command {
        type_needs_defining: bool,
        type_name: Ident,
        function_name_c: String,
        function_name_rust: Ident,
        parameters: TokenStream,
        parameters_unused: TokenStream,
        returns: TokenStream,
    }

    let commands = commands
        .iter()
        .map(|cmd| {
            let type_name = format_ident!("PFN_{}", cmd.name);

            let function_name_c = if let Some(alias_name) = aliases.get(&cmd.name) {
                alias_name.to_string()
            } else {
                cmd.name.to_string()
            };
            let function_name_rust = format_ident!(
                "{}",
                function_name_c
                    .strip_prefix("vk")
                    .unwrap()
                    .to_snake_case()
                    .as_str()
            );

            let params: Vec<_> = cmd
                .param
                .iter()
                .map(|field| {
                    let name = field.param_ident();
                    let ty = field.type_tokens(true);
                    (name, ty)
                })
                .collect();

            let params_iter = params
                .iter()
                .map(|(param_name, param_ty)| quote!(#param_name: #param_ty));
            let parameters = quote!(#(#params_iter,)*);

            let params_iter = params.iter().map(|(param_name, param_ty)| {
                let unused_name = format_ident!("_{}", param_name);
                quote!(#unused_name: #param_ty)
            });
            let parameters_unused = quote!(#(#params_iter,)*);

            Command {
                // PFN function pointers are global and can not have duplicates.
                // This can happen because there are aliases to commands
                type_needs_defining: fn_cache.insert(cmd.name.as_str()),
                type_name,
                function_name_c,
                function_name_rust,
                parameters,
                parameters_unused,
                returns: if cmd.return_type.is_void() {
                    quote!()
                } else {
                    let ret_ty_tokens = cmd.return_type.type_tokens(true);
                    quote!(-> #ret_ty_tokens)
                },
            }
        })
        .collect::<Vec<_>>();

    struct CommandToType<'a>(&'a Command);
    impl<'a> quote::ToTokens for CommandToType<'a> {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let type_name = &self.0.type_name;
            let parameters = &self.0.parameters;
            let returns = &self.0.returns;
            quote!(
                #[allow(non_camel_case_types)]
                pub type #type_name = unsafe extern "system" fn(#parameters) #returns;
            )
            .to_tokens(tokens)
        }
    }

    struct CommandToMember<'a>(&'a Command);
    impl<'a> quote::ToTokens for CommandToMember<'a> {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let type_name = &self.0.type_name;
            let type_name = if self.0.type_needs_defining {
                // Type is defined in local scope
                quote!(#type_name)
            } else {
                // Type is usually defined in another module
                quote!(crate::vk::#type_name)
            };
            let function_name_rust = &self.0.function_name_rust;
            quote!(pub #function_name_rust: #type_name).to_tokens(tokens)
        }
    }

    struct CommandToLoader<'a>(&'a Command);
    impl<'a> quote::ToTokens for CommandToLoader<'a> {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let function_name_rust = &self.0.function_name_rust;
            let parameters_unused = &self.0.parameters_unused;
            let returns = &self.0.returns;

            let byte_function_name =
                Literal::byte_string(format!("{}\0", self.0.function_name_c).as_bytes());

            quote!(
                #function_name_rust: unsafe {
                    unsafe extern "system" fn #function_name_rust (#parameters_unused) #returns {
                        panic!(concat!("Unable to load ", stringify!(#function_name_rust)))
                    }
                    let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(#byte_function_name);
                    let val = _f(cname);
                    if val.is_null() {
                        #function_name_rust
                    } else {
                        ::std::mem::transmute(val)
                    }
                }
            )
            .to_tokens(tokens)
        }
    }

    let pfn_typedefs = commands
        .iter()
        .filter(|pfn| pfn.type_needs_defining)
        .map(CommandToType);
    let members = commands.iter().map(CommandToMember);
    let loaders = commands.iter().map(CommandToLoader);

    quote! {
        #(#pfn_typedefs)*

        #[derive(Clone)]
        pub struct #ident {
            #(#members,)*
        }

        unsafe impl Send for #ident {}
        unsafe impl Sync for #ident {}

        impl #ident {
            pub fn load<F>(mut _f: F) -> Self
                where F: FnMut(&::std::ffi::CStr) -> *const c_void
            {
                Self {
                    #(#loaders,)*
                }
            }
        }
    }
}
pub struct ExtensionConstant<'a> {
    pub name: &'a str,
    pub constant: Constant,
    pub notation: Option<&'a str>,
}
impl<'a> ConstantExt for ExtensionConstant<'a> {
    fn constant(&self, _enum_name: &str) -> Constant {
        self.constant.clone()
    }
    fn variant_ident(&self, enum_name: &str) -> Ident {
        variant_ident(enum_name, self.name)
    }
    fn notation(&self) -> Option<&str> {
        self.notation
    }
}

pub fn generate_extension_constants<'a>(
    extension_name: &str,
    extension_number: i64,
    extension_items: &'a [vk_parse::ExtensionChild],
    const_cache: &mut HashSet<&'a str>,
    const_values: &mut BTreeMap<Ident, ConstantTypeInfo>,
) -> TokenStream {
    let items = extension_items
        .iter()
        .filter_map(get_variant!(vk_parse::ExtensionChild::Require { items }))
        .flatten();

    let mut extended_enums = BTreeMap::<String, Vec<ExtensionConstant>>::new();

    for item in items {
        if let vk_parse::InterfaceItem::Enum(enum_) = item {
            if !const_cache.insert(enum_.name.as_str()) {
                continue;
            }

            if enum_.comment.as_deref() == Some(BACKWARDS_COMPATIBLE_ALIAS_COMMENT) {
                continue;
            }

            let (constant, extends, is_alias) = if let Some(r) =
                Constant::from_vk_parse_enum(enum_, None, Some(extension_number))
            {
                r
            } else {
                continue;
            };
            let extends = if let Some(extends) = extends {
                extends
            } else {
                continue;
            };
            let ext_constant = ExtensionConstant {
                name: &enum_.name,
                constant,
                notation: enum_.comment.as_deref(),
            };
            let ident = name_to_tokens(&extends);
            const_values
                .get_mut(&ident)
                .unwrap()
                .values
                .push(ConstantMatchInfo {
                    ident: ext_constant.variant_ident(&extends),
                    is_alias,
                });

            extended_enums
                .entry(extends)
                .or_default()
                .push(ext_constant);
        }
    }

    let enum_tokens = extended_enums.iter().map(|(extends, constants)| {
        let ident = name_to_tokens(extends);
        let doc_string = format!("Generated from '{}'", extension_name);
        let impl_block = bitflags_impl_block(ident, extends, &constants.iter().collect_vec());
        quote! {
            #[doc = #doc_string]
            #impl_block
        }
    });
    quote!(#(#enum_tokens)*)
}
pub fn generate_extension_commands<'a>(
    extension_name: &str,
    items: &[vk_parse::ExtensionChild],
    cmd_map: &CommandMap<'a>,
    cmd_aliases: &HashMap<String, String>,
    fn_cache: &mut HashSet<&'a str>,
) -> TokenStream {
    let mut commands = Vec::new();
    let mut aliases = HashMap::new();
    let names = items
        .iter()
        .filter_map(get_variant!(vk_parse::ExtensionChild::Require { items }))
        .flatten()
        .filter_map(get_variant!(vk_parse::InterfaceItem::Command { name }));
    for name in names {
        if let Some(cmd) = cmd_map.get(name).copied() {
            commands.push(cmd);
        } else if let Some(cmd) = cmd_aliases
            .get(name)
            .and_then(|alias_name| cmd_map.get(alias_name).copied())
        {
            aliases.insert(cmd.name.clone(), name.to_string());
            commands.push(cmd);
        }
    }

    let ident = format_ident!(
        "{}Fn",
        extension_name
            .to_upper_camel_case()
            .strip_prefix("Vk")
            .unwrap()
    );
    let fp = generate_function_pointers(ident.clone(), &commands, &aliases, fn_cache);

    let spec_version = items
        .iter()
        .filter_map(get_variant!(vk_parse::ExtensionChild::Require { items }))
        .flatten()
        .filter_map(get_variant!(vk_parse::InterfaceItem::Enum))
        .find(|e| e.name.contains("SPEC_VERSION"))
        .and_then(|e| {
            if let vk_parse::EnumSpec::Value { value, .. } = &e.spec {
                let v: u32 = str::parse(value).unwrap();
                Some(quote!(pub const SPEC_VERSION: u32 = #v;))
            } else {
                None
            }
        });

    let byte_name_ident = Literal::byte_string(format!("{}\0", extension_name).as_bytes());
    let extension_cstr = quote! {
        impl #ident {
            #[inline]
            pub const fn name() -> &'static ::std::ffi::CStr {
                unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(#byte_name_ident) }
            }
            #spec_version
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
    const_values: &mut BTreeMap<Ident, ConstantTypeInfo>,
    cmd_aliases: &HashMap<String, String>,
    fn_cache: &mut HashSet<&'a str>,
) -> Option<TokenStream> {
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
    let fp = generate_extension_commands(
        &extension.name,
        &extension.children,
        cmd_map,
        cmd_aliases,
        fn_cache,
    );
    let q = quote! {
        #fp
        #extension_tokens
    };
    Some(q)
}
pub fn generate_define(
    define: &vkxml::Define,
    identifier_renames: &mut BTreeMap<String, Ident>,
) -> TokenStream {
    let name = constant_name(&define.name);
    let ident = format_ident!("{}", name);

    if name == "NULL_HANDLE" {
        quote!()
    } else if let Some(value) = &define.value {
        str::parse::<u32>(value).map_or(quote!(), |v| quote!(pub const #ident: u32 = #v;))
    } else if let Some(c_expr) = &define.c_expression {
        if define.name.contains(&"VERSION".to_string()) {
            let link = khronos_link(&define.name);
            let c_expr = c_expr.trim_start_matches('\\');
            let c_expr = c_expr.replace("(uint32_t)", "");
            let c_expr = convert_c_expression(&c_expr, identifier_renames);
            let c_expr = discard_outmost_delimiter(c_expr);

            let deprecated = define
                .comment
                .as_ref()
                .and_then(|c| c.strip_prefix("DEPRECATED: "))
                .map(|comment| quote!(#[deprecated = #comment]));

            let (code, ident) = if define.parameters.is_empty() {
                (quote!(pub const #ident: u32 = #c_expr;), ident)
            } else {
                let params = define
                    .parameters
                    .iter()
                    .map(|param| format_ident!("{}", param))
                    .map(|i| quote!(#i: u32));
                let ident = format_ident!("{}", name.to_lowercase());
                (
                    quote!(pub const fn #ident(#(#params),*) -> u32 { #c_expr }),
                    ident,
                )
            };

            identifier_renames.insert(define.name.clone(), ident);

            quote! {
                #deprecated
                #[doc = #link]
                #code
            }
        } else {
            quote!()
        }
    } else {
        quote!()
    }
}
pub fn generate_typedef(typedef: &vkxml::Typedef) -> TokenStream {
    if typedef.basetype.is_empty() {
        // Ignore forward declarations
        quote! {}
    } else {
        let typedef_name = name_to_tokens(&typedef.name);
        let typedef_ty = name_to_tokens(&typedef.basetype);
        let khronos_link = khronos_link(&typedef.name);
        quote! {
            #[doc = #khronos_link]
            pub type #typedef_name = #typedef_ty;
        }
    }
}
pub fn generate_bitmask(
    bitmask: &vkxml::Bitmask,
    bitflags_cache: &mut HashSet<Ident>,
    const_values: &mut BTreeMap<Ident, ConstantTypeInfo>,
) -> Option<TokenStream> {
    // Workaround for empty bitmask
    if bitmask.name.is_empty() {
        return None;
    }
    // If this enum has constants, then it will generated later in generate_enums.
    if bitmask.enumref.is_some() {
        return None;
    }

    let name = bitmask.name.strip_prefix("Vk").unwrap();
    let ident = format_ident!("{}", name);
    if !bitflags_cache.insert(ident.clone()) {
        return None;
    };
    const_values.insert(ident.clone(), Default::default());
    let khronos_link = khronos_link(&bitmask.name);
    let type_ = name_to_tokens(&bitmask.basetype);
    Some(quote! {
        #[repr(transparent)]
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[doc = #khronos_link]
        pub struct #ident(pub(crate) #type_);
        vk_bitflags_wrapped!(#ident, #type_);
    })
}

pub enum EnumType {
    Bitflags(TokenStream),
    Enum(TokenStream),
}

static TRAILING_NUMBER: Lazy<Regex> = Lazy::new(|| Regex::new("(\\d+)$").unwrap());

pub fn variant_ident(enum_name: &str, variant_name: &str) -> Ident {
    let variant_name = variant_name.to_uppercase();
    let name = enum_name.replace("FlagBits", "");
    // TODO: Should be read from vk.xml id:2
    // TODO: Also needs to be more robust, vendor names can be substrings from itself, id:4
    // like NVX and NV
    let vendors = [
        "_NVX", "_KHR", "_EXT", "_NV", "_AMD", "_ANDROID", "_GOOGLE", "_INTEL", "_FUCHSIA",
    ];
    let struct_name = name.to_shouty_snake_case();
    let vendor = vendors
        .iter()
        .find(|&vendor| struct_name.ends_with(vendor))
        .cloned()
        .unwrap_or("");
    let struct_name = struct_name.strip_suffix(vendor).unwrap();
    let struct_name = TRAILING_NUMBER.replace(struct_name, "_$1");
    let variant_name = variant_name.strip_suffix(vendor).unwrap_or(&variant_name);

    let new_variant_name = variant_name
        .strip_prefix(struct_name.as_ref())
        .unwrap_or_else(|| {
            if enum_name == "VkResult" {
                variant_name.strip_prefix("VK").unwrap()
            } else {
                panic!(
                    "Failed to strip {} prefix from enum variant {}",
                    struct_name, variant_name
                )
            }
        });

    // Both of the above strip_prefix leave a leading `_`:
    let new_variant_name = new_variant_name.strip_prefix('_').unwrap();
    // Replace _BIT anywhere in the string, also works when there's a trailing
    // vendor extension in the variant name that's not in the enum/type name:
    let new_variant_name = new_variant_name.replace("_BIT", "");
    let is_digit = new_variant_name
        .chars()
        .next()
        .map(|c| c.is_ascii_digit())
        .unwrap_or(false);
    if is_digit {
        format_ident!("TYPE_{}", new_variant_name)
    } else {
        format_ident!("{}", new_variant_name)
    }
}

pub fn bitflags_impl_block(
    ident: Ident,
    enum_name: &str,
    constants: &[&impl ConstantExt],
) -> TokenStream {
    let variants = constants
        .iter()
        .filter(|constant| constant.notation() != Some(BACKWARDS_COMPATIBLE_ALIAS_COMMENT))
        .map(|constant| {
            let variant_ident = constant.variant_ident(enum_name);
            let notation = constant.doc_attribute();
            let constant = constant.constant(enum_name);
            let value = if let Constant::Alias(_) = &constant {
                quote!(#constant)
            } else {
                quote!(Self(#constant))
            };

            quote! {
                #notation
                pub const #variant_ident: Self = #value;
            }
        });

    quote! {
        impl #ident {
            #(#variants)*
        }
    }
}

pub fn generate_enum<'a>(
    enum_: &'a vk_parse::Enums,
    const_cache: &mut HashSet<&'a str>,
    const_values: &mut BTreeMap<Ident, ConstantTypeInfo>,
    bitflags_cache: &mut HashSet<Ident>,
) -> EnumType {
    let name = enum_.name.as_ref().unwrap();
    let clean_name = name.strip_prefix("Vk").unwrap();
    let clean_name = clean_name.replace("FlagBits", "Flags");
    let ident = format_ident!("{}", clean_name.as_str());
    let constants = enum_
        .children
        .iter()
        .filter_map(get_variant!(vk_parse::EnumsChild::Enum))
        .filter(|constant| constant.notation() != Some(BACKWARDS_COMPATIBLE_ALIAS_COMMENT))
        .collect_vec();

    let mut values = Vec::with_capacity(constants.len());
    for constant in &constants {
        const_cache.insert(constant.name.as_str());
        values.push(ConstantMatchInfo {
            ident: constant.variant_ident(name),
            is_alias: constant.is_alias(),
        });
    }
    const_values.insert(
        ident.clone(),
        ConstantTypeInfo {
            values,
            bitwidth: enum_.bitwidth,
        },
    );

    let khronos_link = khronos_link(name);

    if name.contains("Bit") {
        let ident = format_ident!("{}", clean_name.as_str());

        let type_ = if enum_.bitwidth == Some(64u32) {
            quote!(Flags64)
        } else {
            quote!(Flags)
        };

        if !bitflags_cache.insert(ident.clone()) {
            EnumType::Bitflags(quote! {})
        } else {
            let impl_bitflags = bitflags_impl_block(ident.clone(), name, &constants);
            let q = quote! {
                #[repr(transparent)]
                #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
                #[doc = #khronos_link]
                pub struct #ident(pub(crate) #type_);
                vk_bitflags_wrapped!(#ident, #type_);
                #impl_bitflags
            };
            EnumType::Bitflags(q)
        }
    } else {
        let (struct_attribute, special_quote) = match clean_name.as_str() {
            //"StructureType" => generate_structure_type(&_name, _enum, create_info_constants),
            "Result" => (quote!(#[must_use]), generate_result(ident.clone(), enum_)),
            _ => (quote!(), quote!()),
        };

        let impl_block = bitflags_impl_block(ident.clone(), name, &constants);
        let enum_quote = quote! {
            #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
            #[repr(transparent)]
            #[doc = #khronos_link]
            #struct_attribute
            pub struct #ident(pub(crate) i32);
            impl #ident {
                #[inline]
                pub const fn from_raw(x: i32) -> Self { Self(x) }
                #[inline]
                pub const fn as_raw(self) -> i32 { self.0 }
            }
            #impl_block
        };
        let q = quote! {
            #enum_quote
            #special_quote

        };
        EnumType::Enum(q)
    }
}

pub fn generate_result(ident: Ident, enum_: &vk_parse::Enums) -> TokenStream {
    let notation = enum_.children.iter().filter_map(|elem| {
        let (variant_name, notation) = match *elem {
            vk_parse::EnumsChild::Enum(ref constant) => (
                constant.name.as_str(),
                constant.comment.as_deref().unwrap_or(""),
            ),
            _ => {
                return None;
            }
        };

        let variant_ident = variant_ident(enum_.name.as_ref().unwrap(), variant_name);
        Some(quote! {
            Self::#variant_ident => Some(#notation)
        })
    });

    quote! {
        impl ::std::error::Error for #ident {}
        impl fmt::Display for #ident {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                let name = match *self {
                    #(#notation),*,
                    _ => None,
                };
                if let Some(x) = name {
                    fmt.write_str(x)
                } else {
                    // If we don't have a nice message to show, call the generated `Debug` impl
                    // which includes *all* enum variants, including those from extensions.
                    <Self as fmt::Debug>::fmt(self, fmt)
                }
            }
        }
    }
}

fn is_static_array(field: &vkxml::Field) -> bool {
    field
        .array
        .as_ref()
        .map(|ty| matches!(ty, vkxml::ArrayType::Static))
        .unwrap_or(false)
}
pub fn derive_default(struct_: &vkxml::Struct, has_lifetime: bool) -> Option<TokenStream> {
    let name = name_to_tokens(&struct_.name);
    let members = struct_
        .elements
        .iter()
        .filter_map(get_variant!(vkxml::StructElement::Member));
    let is_structure_type = |field: &vkxml::Field| field.basetype == "VkStructureType";

    // These are also pointers, and therefor also don't implement Default. The spec
    // also doesn't mark them as pointers
    let handles = [
        "LPCWSTR",
        "HANDLE",
        "HINSTANCE",
        "HWND",
        "HMONITOR",
        "IOSurfaceRef",
        "MTLBuffer_id",
        "MTLCommandQueue_id",
        "MTLDevice_id",
        "MTLSharedEvent_id",
        "MTLTexture_id",
    ];
    let contains_ptr = members.clone().any(|field| field.reference.is_some());
    let contains_structure_type = members.clone().any(is_structure_type);
    let contains_static_array = members.clone().any(is_static_array);
    if !(contains_ptr || contains_structure_type || contains_static_array) {
        return None;
    };
    let default_fields = members.clone().map(|field| {
        let param_ident = field.param_ident();
        if is_structure_type(field) {
            let ty = field
                .type_enums
                .as_ref()
                .and_then(|ty| ty.split(',').next());
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
        } else if field.reference.is_some() {
            if field.is_const {
                quote!(#param_ident: ::std::ptr::null())
            } else {
                quote!(#param_ident: ::std::ptr::null_mut())
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
    let lifetime = has_lifetime.then(|| quote!(<'_>));
    let marker = has_lifetime.then(|| quote!(_marker: PhantomData,));
    let q = quote! {
        impl ::std::default::Default for #name #lifetime {
            #[inline]
            fn default() -> Self {
                Self {
                    #(
                        #default_fields,
                    )*
                    #marker
                }
            }
        }
    };
    Some(q)
}
pub fn derive_debug(
    struct_: &vkxml::Struct,
    union_types: &HashSet<&str>,
    has_lifetime: bool,
) -> Option<TokenStream> {
    let name = name_to_tokens(&struct_.name);
    let members = struct_
        .elements
        .iter()
        .filter_map(get_variant!(vkxml::StructElement::Member));
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
        let param_str = param_ident.to_string();
        let debug_value = if is_static_array(field) && field.basetype == "char" {
            quote! {
                &unsafe {
                    ::std::ffi::CStr::from_ptr(self.#param_ident.as_ptr())
                }
            }
        } else if param_str.contains("pfn") {
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
    let name_str = name.to_string();
    let lifetime = has_lifetime.then(|| quote!(<'_>));
    let q = quote! {
        #[cfg(feature = "debug")]
        impl fmt::Debug for #name #lifetime {
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
    struct_: &vkxml::Struct,
    struct2: &vk_parse::Type,
    root_structs: &HashSet<Ident>,
    has_lifetimes: &HashSet<Ident>,
) -> Option<TokenStream> {
    if &struct_.name == "VkBaseInStructure"
        || &struct_.name == "VkBaseOutStructure"
        || &struct_.name == "VkTransformMatrixKHR"
        || &struct_.name == "VkAccelerationStructureInstanceKHR"
    {
        return None;
    }

    let members2 = match &struct2.spec {
        vk_parse::TypeSpec::Members(m) => m,
        x => panic!("vk_parse type is not a struct with members but {:?}", x),
    };
    let members2 = members2
        .iter()
        .filter_map(get_variant!(vk_parse::TypeMember::Definition));

    let name = name_to_tokens(&struct_.name);

    let members = struct_
        .elements
        .iter()
        .filter_map(get_variant!(vkxml::StructElement::Member));

    let next_field = members
        .clone()
        .find(|field| field.param_ident() == "p_next");

    let structure_type_field = members2
        .clone()
        .find(|field| field.param_ident() == "s_type");

    // Must either have both, or none:
    assert_eq!(next_field.is_some(), structure_type_field.is_some());

    let nofilter_count_members = [
        ("VkPipelineViewportStateCreateInfo", "pViewports"),
        ("VkPipelineViewportStateCreateInfo", "pScissors"),
        ("VkDescriptorSetLayoutBinding", "pImmutableSamplers"),
    ];
    let filter_members: Vec<String> = members
        .clone()
        .filter_map(|field| {
            let field_name = field.name.as_ref().unwrap();

            // Associated _count members
            if field.array.is_some() {
                if let Some(ref array_size) = field.size {
                    if !nofilter_count_members.contains(&(&struct_.name, field_name)) {
                        return Some((*array_size).clone());
                    }
                }
            }

            // VkShaderModuleCreateInfo requires a custom setter
            if field_name == "codeSize" {
                return Some(field_name.clone());
            }

            None
        })
        .collect();

    let setters = members.clone().filter_map(|field| {
        let param_ident = field.param_ident();
        let param_ty_tokens = field.safe_type_tokens(quote!('a));

        let param_ident_string = param_ident.to_string();
        if param_ident_string == "s_type" || param_ident_string == "p_next" {
            return None;
        }

        let param_ident_short = param_ident_string
            .strip_prefix("p_")
            .or_else(|| param_ident_string.strip_prefix("pp_"))
            .unwrap_or(&param_ident_string);
        let param_ident_short = format_ident!("{}", &param_ident_short);

        if let Some(name) = field.name.as_ref() {
            // Filter
            if filter_members.iter().any(|n| *n == *name) {
                return None;
            }

            // Unique cases
            if name == "pCode" {
                return Some(quote!{
                    #[inline]
                    pub fn code(mut self, code: &'a [u32]) -> Self {
                        self.code_size = code.len() * 4;
                        self.p_code = code.as_ptr();
                        self
                    }
                });
            }

            if name == "pSampleMask" {
                return Some(quote!{
                    /// Sets `p_sample_mask` to `null` if the slice is empty. The mask will
                    /// be treated as if it has all bits set to `1`.
                    ///
                    /// See <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineMultisampleStateCreateInfo.html#_description>
                    /// for more details.
                    #[inline]
                    pub fn sample_mask(mut self, sample_mask: &'a [SampleMask]) -> Self {
                        self.p_sample_mask = if sample_mask.is_empty() {
                            std::ptr::null()
                        } else {
                            sample_mask.as_ptr()
                        };
                        self
                    }
                });
            }

            if name == "ppGeometries" {
                return Some(quote!{
                    #[inline]
                    pub fn geometries_ptrs(mut self, geometries: &'a [&'a AccelerationStructureGeometryKHR<'a>]) -> Self {
                        self.geometry_count = geometries.len() as _;
                        self.pp_geometries = geometries.as_ptr() as *const *const _;
                        self
                    }
                });
            }
        }

        // TODO: Improve in future when https://github.com/rust-lang/rust/issues/53667 is merged id:6
        if field.reference.is_some() {
            if field.basetype == "char" && matches!(field.reference, Some(vkxml::ReferenceType::Pointer)) {
                assert!(field.null_terminate);
                assert_eq!(field.size, None);
                return Some(quote!{
                    #[inline]
                    pub fn #param_ident_short(mut self, #param_ident_short: &'a ::std::ffi::CStr) -> Self {
                        self.#param_ident = #param_ident_short.as_ptr();
                        self
                    }
                });
            }

            if matches!(field.array, Some(vkxml::ArrayType::Dynamic)) {
                if let Some(ref array_size) = field.size {
                    let mut slice_param_ty_tokens = field.safe_type_tokens(quote!('a));

                    let mut ptr = if field.is_const {
                        quote!(.as_ptr())
                    } else {
                        quote!(.as_mut_ptr())
                    };

                    // Interpret void array as byte array
                    if field.basetype == "void" && matches!(field.reference, Some(vkxml::ReferenceType::Pointer)) {
                        let mutable = if field.is_const { quote!(const) } else { quote!(mut) };

                        slice_param_ty_tokens = quote!([u8]);
                        ptr = quote!(#ptr as *#mutable c_void);
                    };

                    let set_size_stmt = if field.is_pointer_to_static_sized_array() {
                        // this is a pointer to a piece of memory with statically known size.
                        let array_size = field.c_size.as_ref().unwrap();
                        let c_size = convert_c_expression(array_size, &BTreeMap::new());
                        let inner_type = field.inner_type_tokens();

                        slice_param_ty_tokens = quote!([#inner_type; #c_size]);
                        ptr = quote!();

                        quote!()
                    } else {
                        let array_size_ident = format_ident!("{}", array_size.to_snake_case().as_str());

                        let size_field = members.clone().find(|m| m.name.as_ref() == Some(array_size)).unwrap();

                        let cast = if size_field.basetype == "size_t" {
                            quote!()
                        }else{
                            quote!(as _)
                        };

                        quote!(self.#array_size_ident = #param_ident_short.len()#cast;)
                    };

                    let mutable = if field.is_const { quote!() } else { quote!(mut) };

                    return Some(quote! {
                        #[inline]
                        pub fn #param_ident_short(mut self, #param_ident_short: &'a #mutable #slice_param_ty_tokens) -> Self {
                            #set_size_stmt
                            self.#param_ident = #param_ident_short#ptr;
                            self
                        }
                    });
                }
            }
        }

        if field.basetype == "VkBool32" {
            return Some(quote!{
                #[inline]
                pub fn #param_ident_short(mut self, #param_ident_short: bool) -> Self {
                    self.#param_ident = #param_ident_short.into();
                    self
                }
            });
        }

        let param_ty_tokens = if is_opaque_type(&field.basetype) {
            //  Use raw pointers for void/opaque types
            field.type_tokens(false)
        } else {
            param_ty_tokens
        };

        let lifetime = has_lifetimes
            .contains(&name_to_tokens(&field.basetype))
            .then(|| quote!(<'a>));

        Some(quote!{
            #[inline]
            pub fn #param_ident_short(mut self, #param_ident_short: #param_ty_tokens #lifetime) -> Self {
                self.#param_ident = #param_ident_short;
                self
            }
        })
    });

    let extends_name = format_ident!("Extends{}", name);

    // The `p_next` field should only be considered if this struct is also a root struct
    let root_struct_next_field = next_field.filter(|_| root_structs.contains(&name));

    // We only implement a next methods for root structs with a `pnext` field.
    let next_function = if let Some(next_field) = root_struct_next_field {
        assert_eq!(next_field.basetype, "void");
        let mutability = if next_field.is_const {
            quote!(const)
        } else {
            quote!(mut)
        };
        quote! {
            /// Prepends the given extension struct between the root and the first pointer. This
            /// method only exists on structs that can be passed to a function directly. Only
            /// valid extension structs can be pushed into the chain.
            /// If the chain looks like `A -> B -> C`, and you call `x.push_next(&mut D)`, then the
            /// chain will look like `A -> D -> B -> C`.
            pub fn push_next<T: #extends_name>(mut self, next: &'a mut T) -> Self {
                unsafe {
                    let next_ptr = <*#mutability T>::cast(next);
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
                    (*last_next).p_next = self.p_next as _;
                    self.p_next = next_ptr;
                }
                self
            }
        }
    } else {
        quote!()
    };

    // Root structs come with their own trait that structs that extend
    // this struct will implement
    let next_trait = if root_struct_next_field.is_some() {
        quote!(pub unsafe trait #extends_name {})
    } else {
        quote!()
    };

    let lifetime = has_lifetimes.contains(&name).then(|| quote!(<'a>));

    // If the struct extends something we need to implement the traits.
    let impl_extend_trait = struct_
        .extends
        .iter()
        .flat_map(|extends| extends.split(','))
        .map(|extends| format_ident!("Extends{}", name_to_tokens(extends)))
        .map(|extends| {
            // Extension structs always have a pNext, and therefore always have a lifetime.
            quote!(unsafe impl #extends for #name<'_> {})
        });

    let impl_structure_type_trait = structure_type_field.map(|s_type| {
        let value = s_type
            .values
            .as_deref()
            .expect("s_type field must have a value in `vk.xml`");

        let value = variant_ident("VkStructureType", value);
        quote! {
            unsafe impl #lifetime TaggedStructure for #name #lifetime {
                const STRUCTURE_TYPE: StructureType = StructureType::#value;
            }
        }
    });

    let q = quote! {
        #impl_structure_type_trait
        #(#impl_extend_trait)*
        #next_trait

        impl #lifetime #name #lifetime {
            #(#setters)*

            #next_function
        }
    };

    Some(q)
}

/// At the moment `Ash` doesn't properly derive all the necessary drives
/// like Eq, Hash etc.
/// To Address some cases, you can add the name of the struct that you
/// require and add the missing derives yourself.
pub fn manual_derives(struct_: &vkxml::Struct) -> TokenStream {
    match struct_.name.as_str() {
        "VkClearRect" | "VkExtent2D" | "VkExtent3D" | "VkOffset2D" | "VkOffset3D" | "VkRect2D"
        | "VkSurfaceFormatKHR" => quote! {PartialEq, Eq, Hash,},
        _ => quote! {},
    }
}
pub fn generate_struct(
    struct_: &vkxml::Struct,
    struct2: &vk_parse::Type,
    root_structs: &HashSet<Ident>,
    union_types: &HashSet<&str>,
    has_lifetimes: &HashSet<Ident>,
) -> TokenStream {
    let name = name_to_tokens(&struct_.name);
    if &struct_.name == "VkTransformMatrixKHR" {
        return quote! {
            #[repr(C)]
            #[derive(Copy, Clone)]
            pub struct TransformMatrixKHR {
                pub matrix: [f32; 12],
            }
        };
    }

    if &struct_.name == "VkAccelerationStructureInstanceKHR" {
        return quote! {
            #[repr(C)]
            #[derive(Copy, Clone)]
            pub union AccelerationStructureReferenceKHR {
                pub device_handle: DeviceAddress,
                pub host_handle: AccelerationStructureKHR,
            }
            #[repr(C)]
            #[derive(Copy, Clone)]
            #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureInstanceKHR.html>"]
            pub struct AccelerationStructureInstanceKHR {
                pub transform: TransformMatrixKHR,
                /// Use [`Packed24_8::new(instance_custom_index, mask)`][Packed24_8::new()] to construct this field
                pub instance_custom_index_and_mask: Packed24_8,
                /// Use [`Packed24_8::new(instance_shader_binding_table_record_offset, flags)`][Packed24_8::new()] to construct this field
                pub instance_shader_binding_table_record_offset_and_flags: Packed24_8,
                pub acceleration_structure_reference: AccelerationStructureReferenceKHR,
            }
        };
    }

    if &struct_.name == "VkAccelerationStructureSRTMotionInstanceNV" {
        return quote! {
            #[repr(C)]
            #[derive(Copy, Clone)]
            #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureSRTMotionInstanceNV.html>"]
            pub struct AccelerationStructureSRTMotionInstanceNV {
                pub transform_t0: SRTDataNV,
                pub transform_t1: SRTDataNV,
                /// Use [`Packed24_8::new(instance_custom_index, mask)`][Packed24_8::new()] to construct this field
                pub instance_custom_index_and_mask: Packed24_8,
                /// Use [`Packed24_8::new(instance_shader_binding_table_record_offset, flags)`][Packed24_8::new()] to construct this field
                pub instance_shader_binding_table_record_offset_and_flags: Packed24_8,
                pub acceleration_structure_reference: AccelerationStructureReferenceKHR,
            }
        };
    }

    if &struct_.name == "VkAccelerationStructureMatrixMotionInstanceNV" {
        return quote! {
            #[repr(C)]
            #[derive(Copy, Clone)]
            #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/AccelerationStructureMatrixMotionInstanceNV.html>"]
            pub struct AccelerationStructureMatrixMotionInstanceNV {
                pub transform_t0: TransformMatrixKHR,
                pub transform_t1: TransformMatrixKHR,
                /// Use [`Packed24_8::new(instance_custom_index, mask)`][Packed24_8::new()] to construct this field
                pub instance_custom_index_and_mask: Packed24_8,
                /// Use [`Packed24_8::new(instance_shader_binding_table_record_offset, flags)`][Packed24_8::new()] to construct this field
                pub instance_shader_binding_table_record_offset_and_flags: Packed24_8,
                pub acceleration_structure_reference: AccelerationStructureReferenceKHR,
            }
        };
    }

    let members = struct_
        .elements
        .iter()
        .filter_map(get_variant!(vkxml::StructElement::Member));

    let params = members.clone().map(|field| {
        let param_ident = field.param_ident();
        let param_ty_tokens = if field.basetype == struct_.name {
            let pointer = field
                .reference
                .as_ref()
                .map(|r| r.to_tokens(field.is_const));
            quote!(#pointer Self)
        } else {
            let lifetime = has_lifetimes
                .contains(&name_to_tokens(&field.basetype))
                .then(|| quote!(<'a>));
            let ty = field.type_tokens(false);
            quote!(#ty #lifetime)
        };
        quote! {pub #param_ident: #param_ty_tokens}
    });

    let has_lifetime = has_lifetimes.contains(&name);
    let (lifetimes, marker) = match has_lifetime {
        true => (quote!(<'a>), quote!(pub _marker: PhantomData<&'a ()>,)),
        false => (quote!(), quote!()),
    };

    let debug_tokens = derive_debug(struct_, union_types, has_lifetime);
    let default_tokens = derive_default(struct_, has_lifetime);
    let setter_tokens = derive_setters(struct_, struct2, root_structs, has_lifetimes);
    let manual_derive_tokens = manual_derives(struct_);
    let dbg_str = if debug_tokens.is_none() {
        quote!(#[cfg_attr(feature = "debug", derive(Debug))])
    } else {
        quote!()
    };
    let default_str = if default_tokens.is_none() {
        quote!(Default,)
    } else {
        quote!()
    };
    let khronos_link = khronos_link(&struct_.name);
    quote! {
        #[repr(C)]
        #dbg_str
        #[derive(Copy, Clone, #default_str #manual_derive_tokens)]
        #[doc = #khronos_link]
        pub struct #name #lifetimes {
            #(#params,)*
            #marker
        }
        #debug_tokens
        #default_tokens
        #setter_tokens
    }
}

pub fn generate_handle(handle: &vkxml::Handle) -> Option<TokenStream> {
    if handle.name.is_empty() {
        return None;
    }
    let khronos_link = khronos_link(&handle.name);
    let tokens = match handle.ty {
        vkxml::HandleType::Dispatch => {
            let name = handle.name.strip_prefix("Vk").unwrap();
            let ty = format_ident!("{}", name.to_shouty_snake_case());
            let name = format_ident!("{}", name);
            quote! {
                define_handle!(#name, #ty, doc = #khronos_link);
            }
        }
        vkxml::HandleType::NoDispatch => {
            let name = handle.name.strip_prefix("Vk").unwrap();
            let ty = format_ident!("{}", name.to_shouty_snake_case());
            let name = format_ident!("{}", name);
            quote! {
                handle_nondispatchable!(#name, #ty, doc = #khronos_link);
            }
        }
    };
    Some(tokens)
}
fn generate_funcptr(fnptr: &vkxml::FunctionPointer) -> TokenStream {
    let name = format_ident!("{}", fnptr.name.as_str());
    let ret_ty_tokens = if fnptr.return_type.is_void() {
        quote!()
    } else {
        let ret_ty_tokens = fnptr.return_type.type_tokens(true);
        quote!(-> #ret_ty_tokens)
    };
    let params = fnptr.param.iter().map(|field| {
        let ident = field.param_ident();
        let type_tokens = field.type_tokens(true);
        quote! {
            #ident: #type_tokens
        }
    });
    let khronos_link = khronos_link(&fnptr.name);
    quote! {
        #[allow(non_camel_case_types)]
        #[doc = #khronos_link]
        pub type #name = Option<unsafe extern "system" fn(#(#params),*) #ret_ty_tokens>;
    }
}

fn generate_union(union: &vkxml::Union, has_lifetimes: &HashSet<Ident>) -> TokenStream {
    let name = name_to_tokens(&union.name);
    let fields = union.elements.iter().map(|field| {
        let name = field.param_ident();
        let ty = field.type_tokens(false);
        let lifetime = has_lifetimes
            .contains(&name_to_tokens(&field.basetype))
            .then(|| quote!(<'a>));
        quote! {
            pub #name: #ty #lifetime
        }
    });
    let khronos_link = khronos_link(&union.name);
    let lifetime = has_lifetimes.contains(&name).then(|| quote!(<'a>));
    quote! {
        #[repr(C)]
        #[derive(Copy, Clone)]
        #[doc = #khronos_link]
        pub union #name #lifetime {
            #(#fields),*
        }
        impl #lifetime ::std::default::Default for #name #lifetime {
            #[inline]
            fn default() -> Self {
                unsafe { ::std::mem::zeroed() }
            }
        }
    }
}
/// Root structs are all structs that are extended by other structs.
pub fn root_structs(definitions: &[&vkxml::DefinitionsElement]) -> HashSet<Ident> {
    let mut root_structs = HashSet::new();
    // Loop over all structs and collect their extends
    for definition in definitions {
        if let vkxml::DefinitionsElement::Struct(ref struct_) = definition {
            if let Some(extends) = &struct_.extends {
                root_structs.extend(extends.split(',').map(name_to_tokens));
            }
        };
    }
    root_structs
}

#[allow(clippy::too_many_arguments)]
pub fn generate_definition(
    definition: &vkxml::DefinitionsElement,
    definition2: Option<&vk_parse::Type>,
    union_types: &HashSet<&str>,
    root_structs: &HashSet<Ident>,
    has_lifetimes: &HashSet<Ident>,
    bitflags_cache: &mut HashSet<Ident>,
    const_values: &mut BTreeMap<Ident, ConstantTypeInfo>,
    identifier_renames: &mut BTreeMap<String, Ident>,
) -> Option<TokenStream> {
    match (definition, definition2) {
        (vkxml::DefinitionsElement::Define(define), _) => {
            Some(generate_define(define, identifier_renames))
        }
        (vkxml::DefinitionsElement::Typedef(typedef), _) => Some(generate_typedef(typedef)),
        (vkxml::DefinitionsElement::Struct(struct_), Some(struct2)) => Some(generate_struct(
            struct_,
            struct2,
            root_structs,
            union_types,
            has_lifetimes,
        )),
        (vkxml::DefinitionsElement::Bitmask(mask), _) => {
            generate_bitmask(mask, bitflags_cache, const_values)
        }
        (vkxml::DefinitionsElement::Handle(handle), _) => generate_handle(handle),
        (vkxml::DefinitionsElement::FuncPtr(fp), _) => Some(generate_funcptr(fp)),
        (vkxml::DefinitionsElement::Union(union), _) => Some(generate_union(union, has_lifetimes)),
        _ => None,
    }
}
pub fn generate_feature<'a>(
    feature: &vkxml::Feature,
    commands: &CommandMap<'a>,
    fn_cache: &mut HashSet<&'a str>,
) -> TokenStream {
    let (static_commands, entry_commands, device_commands, instance_commands) = feature
        .elements
        .iter()
        .filter_map(get_variant!(vkxml::FeatureElement::Require))
        .flat_map(|spec| &spec.elements)
        .filter_map(get_variant!(vkxml::FeatureReference::CommandReference))
        .filter_map(|cmd_ref| commands.get(&cmd_ref.name))
        .fold(
            (Vec::new(), Vec::new(), Vec::new(), Vec::new()),
            |mut accs, &cmd_ref| {
                let acc = match cmd_ref.function_type() {
                    FunctionType::Static => &mut accs.0,
                    FunctionType::Entry => &mut accs.1,
                    FunctionType::Device => &mut accs.2,
                    FunctionType::Instance => &mut accs.3,
                };
                acc.push(cmd_ref);
                accs
            },
        );
    let version = feature.version_string();
    let static_fn = if feature.is_version(1, 0) {
        generate_function_pointers(
            format_ident!("{}", "StaticFn"),
            &static_commands,
            &HashMap::new(),
            fn_cache,
        )
    } else {
        quote! {}
    };
    let entry = generate_function_pointers(
        format_ident!("EntryFnV{}", version),
        &entry_commands,
        &HashMap::new(),
        fn_cache,
    );
    let instance = generate_function_pointers(
        format_ident!("InstanceFnV{}", version),
        &instance_commands,
        &HashMap::new(),
        fn_cache,
    );
    let device = generate_function_pointers(
        format_ident!("DeviceFnV{}", version),
        &device_commands,
        &HashMap::new(),
        fn_cache,
    );
    quote! {
        #static_fn
        #entry
        #instance
        #device
    }
}

pub fn constant_name(name: &str) -> &str {
    name.strip_prefix("VK_").unwrap_or(name)
}

pub fn generate_constant<'a>(
    constant: &'a vkxml::Constant,
    cache: &mut HashSet<&'a str>,
) -> TokenStream {
    cache.insert(constant.name.as_str());
    let c = Constant::from_constant(constant);
    let name = constant_name(&constant.name);
    let ident = format_ident!("{}", name);
    let notation = constant.doc_attribute();

    let ty = if name == "TRUE" || name == "FALSE" {
        CType::Bool32
    } else {
        c.ty()
    };
    quote! {
        #notation
        pub const #ident: #ty = #c;
    }
}

pub fn generate_feature_extension<'a>(
    registry: &'a vk_parse::Registry,
    const_cache: &mut HashSet<&'a str>,
    const_values: &mut BTreeMap<Ident, ConstantTypeInfo>,
) -> TokenStream {
    let constants = registry
        .0
        .iter()
        .filter_map(get_variant!(vk_parse::RegistryChild::Feature))
        .map(|feature| {
            generate_extension_constants(
                &feature.name,
                0,
                &feature.children,
                const_cache,
                const_values,
            )
        });
    quote! {
        #(#constants)*
    }
}

pub struct ConstantMatchInfo {
    pub ident: Ident,
    pub is_alias: bool,
}

#[derive(Default)]
pub struct ConstantTypeInfo {
    values: Vec<ConstantMatchInfo>,
    bitwidth: Option<u32>,
}

pub struct ConstDebugs {
    core: TokenStream,
    extras: TokenStream,
}

pub fn generate_const_debugs(const_values: &BTreeMap<Ident, ConstantTypeInfo>) -> ConstDebugs {
    let mut core = Vec::new();
    let mut extras = Vec::new();
    for (ty, values) in const_values {
        let ConstantTypeInfo { values, bitwidth } = values;
        let out = if ty.to_string().contains("Flags") {
            let cases = values.iter().filter_map(|value| {
                if value.is_alias {
                    None
                } else {
                    let ident = &value.ident;
                    let name = ident.to_string();
                    Some(quote! { (#ty::#ident.0, #name) })
                }
            });

            let type_ = if bitwidth == &Some(64u32) {
                quote!(Flags64)
            } else {
                quote!(Flags)
            };

            quote! {
                impl fmt::Debug for #ty {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        const KNOWN: &[(#type_, &str)] = &[#(#cases),*];
                        debug_flags(f, KNOWN, self.0)
                    }
                }
            }
        } else {
            let cases = values.iter().filter_map(|value| {
                if value.is_alias {
                    None
                } else {
                    let ident = &value.ident;
                    let name = ident.to_string();
                    Some(quote! { Self::#ident => Some(#name), })
                }
            });
            quote! {
                impl fmt::Debug for #ty {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        let name = match *self {
                            #(#cases)*
                            _ => None,
                        };
                        if let Some(x) = name {
                            f.write_str(x)
                        } else {
                            self.0.fmt(f)
                        }
                    }
                }
            }
        };
        if ty == "Result" || ty == "ObjectType" {
            core.push(out);
        } else {
            extras.push(out);
        }
    }

    ConstDebugs {
        core: quote! {
            #(#core)*
        },
        extras: quote! {
            #(#extras)*
        },
    }
}
pub fn extract_native_types(registry: &vk_parse::Registry) -> (Vec<(String, String)>, Vec<String>) {
    // Not a HashMap so that headers are processed in order of definition:
    let mut header_includes = vec![];
    let mut header_types = vec![];

    let types = registry
        .0
        .iter()
        .filter_map(get_variant!(vk_parse::RegistryChild::Types))
        .flat_map(|ty| &ty.children)
        .filter_map(get_variant!(vk_parse::TypesChild::Type));

    for ty in types {
        match ty.category.as_deref() {
            Some("include") => {
                // `category="include"` lacking an `#include` directive are generally "irrelevant" system headers.
                if let vk_parse::TypeSpec::Code(code) = &ty.spec {
                    let name = ty
                        .name
                        .clone()
                        .expect("Include type must provide header name");
                    assert!(
                        header_includes
                            .iter()
                            .all(|(other_name, _)| other_name != &name),
                        "Header `{}` being redefined",
                        name
                    );

                    let (rem, path) = parse_c_include::<VerboseError<&str>>(&code.code)
                        .expect("Failed to parse `#include` from `category=\"include\"` directive");
                    assert!(rem.is_empty());
                    header_includes.push((name, path));
                }
            }
            Some(_) => {}
            None => {
                if let Some(header_name) = ty.requires.clone() {
                    if header_includes.iter().any(|(name, _)| name == &header_name) {
                        // Omit types from system and other headers
                        header_types.push(ty.name.clone().expect("Type must have a name"));
                    }
                }
            }
        };
    }

    (header_includes, header_types)
}
pub fn generate_aliases_of_types(
    types: &vk_parse::Types,
    has_lifetimes: &HashSet<Ident>,
    ty_cache: &mut HashSet<Ident>,
) -> TokenStream {
    let aliases = types
        .children
        .iter()
        .filter_map(get_variant!(vk_parse::TypesChild::Type))
        .filter_map(|ty| {
            let name = ty.name.as_ref()?;
            let alias = ty.alias.as_ref()?;
            let name_ident = name_to_tokens(name);
            if !ty_cache.insert(name_ident.clone()) {
                return None;
            };
            let alias_ident = name_to_tokens(alias);
            let tokens = if has_lifetimes.contains(&alias_ident) {
                quote!(pub type #name_ident<'a> = #alias_ident<'a>;)
            } else {
                quote!(pub type #name_ident = #alias_ident;)
            };
            Some(tokens)
        });
    quote! {
        #(#aliases)*
    }
}
pub fn write_source_code<P: AsRef<Path>>(vk_headers_dir: &Path, src_dir: P) {
    let vk_xml = vk_headers_dir.join("registry/vk.xml");
    use std::fs::File;
    use std::io::Write;
    let (spec2, _errors) = vk_parse::parse_file(&vk_xml).expect("Invalid xml file");
    let extensions: &Vec<vk_parse::Extension> = spec2
        .0
        .iter()
        .find_map(get_variant!(vk_parse::RegistryChild::Extensions))
        .map(|ext| &ext.children)
        .expect("extension");

    let spec = vk_parse::parse_file_as_vkxml(&vk_xml).expect("Invalid xml file.");
    let cmd_aliases: HashMap<String, String> = spec2
        .0
        .iter()
        .filter_map(get_variant!(vk_parse::RegistryChild::Commands))
        .flat_map(|cmds| &cmds.children)
        .filter_map(get_variant!(vk_parse::Command::Alias { name, alias }))
        .map(|(name, alias)| (name.to_string(), alias.to_string()))
        .collect();

    let commands: HashMap<vkxml::Identifier, &vkxml::Command> = spec
        .elements
        .iter()
        .filter_map(get_variant!(vkxml::RegistryElement::Commands))
        .flat_map(|cmds| &cmds.elements)
        .map(|cmd| (cmd.name.clone(), cmd))
        .collect();

    let features: Vec<&vkxml::Feature> = spec
        .elements
        .iter()
        .filter_map(get_variant!(vkxml::RegistryElement::Features))
        .flat_map(|features| &features.elements)
        .collect();

    let definitions: Vec<&vkxml::DefinitionsElement> = spec
        .elements
        .iter()
        .filter_map(get_variant!(vkxml::RegistryElement::Definitions))
        .flat_map(|definitions| &definitions.elements)
        .collect();

    let definitions2: Vec<&vk_parse::TypesChild> = spec2
        .0
        .iter()
        .filter_map(get_variant!(vk_parse::RegistryChild::Types))
        .flat_map(|definitions| &definitions.children)
        .collect();

    let constants: Vec<&vkxml::Constant> = spec
        .elements
        .iter()
        .filter_map(get_variant!(vkxml::RegistryElement::Constants))
        .flat_map(|constants| &constants.elements)
        .collect();

    let mut fn_cache = HashSet::new();
    let mut bitflags_cache = HashSet::new();
    let mut const_cache = HashSet::new();

    let mut const_values: BTreeMap<Ident, ConstantTypeInfo> = BTreeMap::new();

    let (enum_code, bitflags_code) = spec2
        .0
        .iter()
        .filter_map(get_variant!(vk_parse::RegistryChild::Enums))
        .filter(|enums| enums.kind.is_some())
        .map(|e| generate_enum(e, &mut const_cache, &mut const_values, &mut bitflags_cache))
        .fold((Vec::new(), Vec::new()), |mut acc, elem| {
            match elem {
                EnumType::Enum(token) => acc.0.push(token),
                EnumType::Bitflags(token) => acc.1.push(token),
            };
            acc
        });

    let mut constants_code: Vec<_> = constants
        .iter()
        .map(|constant| generate_constant(constant, &mut const_cache))
        .collect();

    constants_code.push(quote! { pub const SHADER_UNUSED_NV : u32 = SHADER_UNUSED_KHR;});

    let extension_code = extensions
        .iter()
        .filter_map(|ext| {
            generate_extension(
                ext,
                &commands,
                &mut const_cache,
                &mut const_values,
                &cmd_aliases,
                &mut fn_cache,
            )
        })
        .collect_vec();

    let union_types = definitions
        .iter()
        .filter_map(get_variant!(vkxml::DefinitionsElement::Union))
        .map(|union_| union_.name.as_str())
        .collect::<HashSet<&str>>();

    let mut identifier_renames = BTreeMap::new();

    // Identify structs that need a lifetime annotation
    // Note that this relies on `vk.xml` defining types before they are used,
    // as is required in C(++) too.
    let mut has_lifetimes = definitions
        .iter()
        .filter_map(get_variant!(vkxml::DefinitionsElement::Struct))
        .filter_map(|s| {
            s.elements
                .iter()
                .filter_map(get_variant!(vkxml::StructElement::Member))
                .any(|x| x.reference.is_some())
                .then(|| name_to_tokens(&s.name))
        })
        .collect::<HashSet<Ident>>();
    for def in &definitions {
        match def {
            vkxml::DefinitionsElement::Struct(s) => s
                .elements
                .iter()
                .filter_map(get_variant!(vkxml::StructElement::Member))
                .any(|field| has_lifetimes.contains(&name_to_tokens(&field.basetype)))
                .then(|| has_lifetimes.insert(name_to_tokens(&s.name))),
            vkxml::DefinitionsElement::Union(u) => u
                .elements
                .iter()
                .any(|field| has_lifetimes.contains(&name_to_tokens(&field.basetype)))
                .then(|| has_lifetimes.insert(name_to_tokens(&u.name))),
            _ => continue,
        };
    }

    let root_structs = root_structs(&definitions);
    let definition_code: Vec<_> = definitions
        .into_iter()
        .filter_map(|def| {
            let def2 = definitions2
                .iter()
                .filter_map(get_variant!(vk_parse::TypesChild::Type))
                .find(|def2| match def {
                    vkxml::DefinitionsElement::Struct(s) => {
                        def2.name.as_deref() == Some(s.name.as_str())
                    }
                    // TODO: Compare the name for more vkxml types
                    _ => false,
                });
            generate_definition(
                def,
                def2,
                &union_types,
                &root_structs,
                &has_lifetimes,
                &mut bitflags_cache,
                &mut const_values,
                &mut identifier_renames,
            )
        })
        .collect();

    let mut ty_cache = HashSet::new();
    let aliases: Vec<_> = spec2
        .0
        .iter()
        .filter_map(get_variant!(vk_parse::RegistryChild::Types))
        .map(|ty| generate_aliases_of_types(ty, &has_lifetimes, &mut ty_cache))
        .collect();

    let feature_code: Vec<_> = features
        .iter()
        .map(|feature| generate_feature(feature, &commands, &mut fn_cache))
        .collect();
    let feature_extensions_code =
        generate_feature_extension(&spec2, &mut const_cache, &mut const_values);

    let ConstDebugs {
        core: core_debugs,
        extras: const_debugs,
    } = generate_const_debugs(&const_values);

    let src_dir = src_dir.as_ref();

    let vk_dir = src_dir.join("vk");
    std::fs::create_dir_all(&vk_dir).expect("failed to create vk dir");

    let mut vk_features_file = File::create(vk_dir.join("features.rs")).expect("vk/features.rs");
    let mut vk_definitions_file =
        File::create(vk_dir.join("definitions.rs")).expect("vk/definitions.rs");
    let mut vk_enums_file = File::create(vk_dir.join("enums.rs")).expect("vk/enums.rs");
    let mut vk_bitflags_file = File::create(vk_dir.join("bitflags.rs")).expect("vk/bitflags.rs");
    let mut vk_constants_file = File::create(vk_dir.join("constants.rs")).expect("vk/constants.rs");
    let mut vk_extensions_file =
        File::create(vk_dir.join("extensions.rs")).expect("vk/extensions.rs");
    let mut vk_feature_extensions_file =
        File::create(vk_dir.join("feature_extensions.rs")).expect("vk/feature_extensions.rs");
    let mut vk_const_debugs_file =
        File::create(vk_dir.join("const_debugs.rs")).expect("vk/const_debugs.rs");
    let mut vk_aliases_file = File::create(vk_dir.join("aliases.rs")).expect("vk/aliases.rs");

    let feature_code = quote! {
        use std::os::raw::*;
        use crate::vk::bitflags::*;
        use crate::vk::definitions::*;
        use crate::vk::enums::*;
        #(#feature_code)*
    };

    let definition_code = quote! {
        use std::marker::PhantomData;
        use std::fmt;
        use std::os::raw::*;
        use crate::vk::{Handle, ptr_chain_iter};
        use crate::vk::aliases::*;
        use crate::vk::bitflags::*;
        use crate::vk::constants::*;
        use crate::vk::enums::*;
        use crate::vk::native::*;
        use crate::vk::platform_types::*;
        use crate::vk::prelude::*;
        #(#definition_code)*
    };

    let enum_code = quote! {
        use std::fmt;
        #(#enum_code)*
        #core_debugs
    };

    let bitflags_code = quote! {
        use crate::vk::definitions::*;
        #(#bitflags_code)*
    };

    let constants_code = quote! {
        use crate::vk::definitions::*;
        #(#constants_code)*
    };

    let extension_code = quote! {
        use std::os::raw::*;
        use crate::vk::platform_types::*;
        use crate::vk::aliases::*;
        use crate::vk::bitflags::*;
        use crate::vk::definitions::*;
        use crate::vk::enums::*;
        #(#extension_code)*
    };

    let feature_extensions_code = quote! {
        use crate::vk::bitflags::*;
        use crate::vk::enums::*;
       #feature_extensions_code
    };

    let const_debugs = quote! {
        use std::fmt;
        use crate::vk::bitflags::*;
        use crate::vk::definitions::*;
        use crate::vk::enums::*;
        use crate::prelude::debug_flags;
        #const_debugs
    };

    let aliases = quote! {
        use crate::vk::bitflags::*;
        use crate::vk::definitions::*;
        use crate::vk::enums::*;
        #(#aliases)*
    };

    write!(&mut vk_features_file, "{}", feature_code).expect("Unable to write vk/features.rs");
    write!(&mut vk_definitions_file, "{}", definition_code)
        .expect("Unable to write vk/definitions.rs");
    write!(&mut vk_enums_file, "{}", enum_code).expect("Unable to write vk/enums.rs");
    write!(&mut vk_bitflags_file, "{}", bitflags_code).expect("Unable to write vk/bitflags.rs");
    write!(&mut vk_constants_file, "{}", constants_code).expect("Unable to write vk/constants.rs");
    write!(&mut vk_extensions_file, "{}", extension_code)
        .expect("Unable to write vk/extensions.rs");
    write!(
        &mut vk_feature_extensions_file,
        "{}",
        feature_extensions_code
    )
    .expect("Unable to write vk/feature_extensions.rs");
    write!(&mut vk_const_debugs_file, "{}", const_debugs)
        .expect("Unable to write vk/const_debugs.rs");
    write!(&mut vk_aliases_file, "{}", aliases).expect("Unable to write vk/aliases.rs");

    let vk_include = vk_headers_dir.join("include");

    let mut bindings = bindgen::Builder::default().clang_arg(format!(
        "-I{}",
        vk_include.to_str().expect("Valid UTF8 string")
    ));

    let (header_includes, header_types) = extract_native_types(&spec2);

    for (_name, path) in header_includes {
        let path = if path == "vk_platform.h" {
            // Fix broken path, https://github.com/KhronosGroup/Vulkan-Docs/pull/1538
            // Reintroduced in: https://github.com/KhronosGroup/Vulkan-Docs/issues/1573
            vk_include.join("vulkan").join(path)
        } else {
            vk_include.join(path)
        };
        bindings = bindings.header(path.to_str().expect("Valid UTF8 string"));
    }

    for typ in header_types {
        bindings = bindings.allowlist_type(typ);
    }

    #[derive(Debug)]
    struct ParseCallbacks;
    impl bindgen::callbacks::ParseCallbacks for ParseCallbacks {
        fn enum_variant_behavior(
            &self,
            _enum_name: Option<&str>,
            original_variant_name: &str,
            _variant_value: bindgen::callbacks::EnumVariantValue,
        ) -> Option<bindgen::callbacks::EnumVariantCustomBehavior> {
            if original_variant_name.ends_with("_MAX_ENUM") {
                Some(bindgen::callbacks::EnumVariantCustomBehavior::Hide)
            } else {
                None
            }
        }
    }

    bindings
        .parse_callbacks(Box::new(ParseCallbacks))
        .generate()
        .expect("Unable to generate native bindings")
        .write_to_file(vk_dir.join("native.rs"))
        .expect("Couldn't write native bindings!");
}
