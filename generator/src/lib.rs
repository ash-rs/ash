#![recursion_limit = "256"]
#![warn(
    clippy::use_self,
    deprecated_in_future,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unused_qualifications
)]

use heck::{ToShoutySnakeCase, ToSnakeCase, ToUpperCamelCase};
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while1},
    character::complete::{
        char, digit1, hex_digit1, multispace0, multispace1, newline, none_of, one_of,
    },
    combinator::{map, map_res, opt, value},
    multi::{many1, separated_list1},
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    IResult, Parser,
};
use once_cell::sync::Lazy;
use proc_macro2::{Delimiter, Group, Literal, Span, TokenStream, TokenTree};
use quote::*;
use regex::Regex;
use std::{
    borrow::Cow,
    collections::{BTreeMap, HashMap, HashSet},
    fmt::Display,
    ops::Not,
    path::Path,
};
use syn::Ident;

const DESIRED_API: &str = "vulkan";

fn contains_desired_api(api: &str) -> bool {
    api.split(',').any(|n| n == DESIRED_API)
}

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

fn parse_ctype(i: &str) -> IResult<&str, CType> {
    (alt((value(CType::U64, tag("ULL")), value(CType::U32, tag("U")))))(i)
}

fn parse_cexpr(i: &str) -> IResult<&str, (CType, String)> {
    (alt((
        map(parse_cfloat, |f| (CType::Float, format!("{f:.2}"))),
        parse_inverse_number,
        parse_decimal_number,
        parse_hexadecimal_number,
    )))(i)
}

fn parse_cfloat(i: &str) -> IResult<&str, f32> {
    (terminated(nom::number::complete::float, one_of("fF")))(i)
}

fn parse_inverse_number(i: &str) -> IResult<&str, (CType, String)> {
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
                format!("!{num}-{minus}")
            } else {
                format!("!{num}")
            };
            (ctyp, expr)
        },
    ))(i)
}

// Like a C string, but does not support quote escaping and expects at least one character.
// If needed, use https://github.com/Geal/nom/blob/8e09f0c3029d32421b5b69fb798cef6855d0c8df/tests/json.rs#L61-L81
fn parse_c_include_string(i: &str) -> IResult<&str, String> {
    (delimited(
        char('"'),
        map(many1(none_of("\"")), |c| {
            c.iter().map(char::to_string).join("")
        }),
        char('"'),
    ))(i)
}

fn parse_c_include(i: &str) -> IResult<&str, String> {
    (preceded(
        tag("#include"),
        preceded(multispace1, parse_c_include_string),
    ))(i)
}

fn parse_decimal_number(i: &str) -> IResult<&str, (CType, String)> {
    (map(
        pair(digit1.map(str::to_string), parse_ctype),
        |(dig, ctype)| (ctype, dig),
    ))(i)
}

fn parse_hexadecimal_number(i: &str) -> IResult<&str, (CType, String)> {
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

fn parse_c_identifier(i: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c == '_' || c.is_alphanumeric())(i)
}

fn parse_comment_suffix(i: &str) -> IResult<&str, Option<&str>> {
    opt(delimited(tag("//"), take_until("\n"), newline))(i)
}

fn parse_parameter_names(i: &str) -> IResult<&str, Vec<&str>> {
    delimited(
        char('('),
        separated_list1(tag(", "), parse_c_identifier),
        char(')'),
    )(i)
}

/// Parses a C macro define optionally prefixed by a comment and optionally
/// containing parameter names. The expression is left in the remainder
#[allow(clippy::type_complexity)]
fn parse_c_define_header(i: &str) -> IResult<&str, (Option<&str>, (&str, Option<Vec<&str>>))> {
    (pair(
        parse_comment_suffix,
        preceded(
            tag("#define "),
            pair(parse_c_identifier, opt(parse_parameter_names)),
        ),
    ))(i)
}

#[derive(Debug)]
enum CReferenceType {
    Value,
    PointerToConst,
    Pointer,
    PointerToPointer,
    PointerToPointerToConst,
    PointerToConstPointer,
    PointerToConstPointerToConst,
}

#[derive(Debug)]
struct CParameterType<'a> {
    name: &'a str,
    reference_type: CReferenceType,
}

fn parse_c_type(i: &str) -> IResult<&str, CParameterType<'_>> {
    (map(
        separated_pair(
            tuple((
                opt(tag("const ")),
                preceded(opt(tag("struct ")), parse_c_identifier),
                opt(char('*')),
            )),
            multispace0,
            opt(pair(opt(tag("const")), char('*'))),
        ),
        |((const_, name, firstptr), secondptr)| CParameterType {
            name,
            reference_type: match (firstptr, secondptr) {
                (None, None) => CReferenceType::Value,

                (Some(_), None) if const_.is_some() => CReferenceType::PointerToConst,
                (Some(_), None) => CReferenceType::Pointer,

                (Some(_), Some((Some(_), _))) if const_.is_some() => {
                    CReferenceType::PointerToConstPointerToConst
                }
                (Some(_), Some((Some(_), _))) => CReferenceType::PointerToConstPointer,

                (Some(_), Some((None, _))) if const_.is_some() => {
                    CReferenceType::PointerToPointerToConst
                }
                (Some(_), Some((None, _))) => CReferenceType::PointerToPointer,
                (None, Some(_)) => unreachable!(),
            },
        },
    ))(i)
}

#[derive(Debug)]
struct CParameter<'a> {
    type_: CParameterType<'a>,
    // Code only used to dissect the type surrounding this field name,
    // not interested in the name itself.
    _name: &'a str,
    static_array: Option<usize>,
}

/// Parses a single C parameter instance, for example:
///
/// ```c
/// VkSparseImageMemoryRequirements2* pSparseMemoryRequirements
/// ```
fn parse_c_parameter(i: &str) -> IResult<&str, CParameter<'_>> {
    (map(
        separated_pair(
            parse_c_type,
            multispace0,
            pair(
                parse_c_identifier,
                opt(delimited(char('['), map_res(digit1, str::parse), char(']'))),
            ),
        ),
        |(type_, (name, static_array))| CParameter {
            type_,
            _name: name,
            static_array,
        },
    ))(i)
}

fn khronos_link<S: Display + ?Sized>(name: &S) -> Literal {
    Literal::string(&format!(
        "<https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/{name}.html>"
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
            Self::U64(n) => *n,
            _ => panic!("Constval not supported"),
        }
    }
}
pub trait ConstantExt {
    fn constant(&self, enum_name: &str) -> Constant;
    fn variant_ident(&self, enum_name: &str) -> Ident;
    fn notation(&self) -> Option<&str>;
    fn formatted_notation(&self) -> Option<Cow<'_, str>> {
        static DOC_LINK: Lazy<Regex> = Lazy::new(|| Regex::new(r"<<([\w-]+)>>").unwrap());
        self.notation().map(|n| {
            DOC_LINK.replace(
                n,
                "<https://registry.khronos.org/vulkan/specs/1.3-extensions/html/vkspec.html#${1}>",
            )
        })
    }
    fn is_alias(&self) -> bool {
        false
    }
    fn is_deprecated(&self) -> bool;
    fn doc_attribute(&self) -> Option<TokenStream> {
        self.formatted_notation().map(|n| quote!(#[doc = #n]))
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
    fn is_deprecated(&self) -> bool {
        todo!()
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
    fn is_deprecated(&self) -> bool {
        self.deprecated.is_some()
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
    fn is_deprecated(&self) -> bool {
        todo!()
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
            Self::Number(n) => {
                let number = interleave_number('_', 3, &n.to_string());
                syn::LitInt::new(&number, Span::call_site()).to_tokens(tokens);
            }
            Self::Hex(ref s) => {
                let number = interleave_number('_', 4, s);
                syn::LitInt::new(&format!("0x{number}"), Span::call_site()).to_tokens(tokens);
            }
            Self::Text(ref text) => text.to_tokens(tokens),
            Self::CExpr(ref expr) => {
                let (rem, (_, rexpr)) = parse_cexpr(expr).expect("Unable to parse cexpr");
                assert!(rem.is_empty());
                tokens.extend(rexpr.parse::<TokenStream>());
            }
            Self::BitPos(pos) => {
                let value = 1u64 << pos;
                let bit_string = format!("{value:b}");
                let bit_string = interleave_number('_', 4, &bit_string);
                syn::LitInt::new(&format!("0b{bit_string}"), Span::call_site()).to_tokens(tokens);
            }
            Self::Alias(ref value) => tokens.extend(quote!(Self::#value)),
        }
    }
}

impl quote::ToTokens for ConstVal {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::U32(n) => n.to_tokens(tokens),
            Self::U64(n) => n.to_tokens(tokens),
            Self::Float(f) => f.to_tokens(tokens),
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
            Self::Number(n) => Some(ConstVal::U64(n as u64)),
            Self::Hex(ref hex) => u64::from_str_radix(hex, 16).ok().map(ConstVal::U64),
            Self::BitPos(pos) => Some(ConstVal::U64(1u64 << pos)),
            _ => None,
        }
    }

    pub fn ty(&self) -> CType {
        match self {
            Self::Number(_) | Self::Hex(_) => CType::USize,
            Self::CExpr(expr) => {
                let (rem, (ty, _)) = parse_cexpr(expr).expect("Unable to parse cexpr");
                assert!(rem.is_empty());
                ty
            }
            _ => unimplemented!(),
        }
    }

    pub fn from_extension_enum(constant: &vkxml::ExtensionEnum) -> Option<Self> {
        let number = constant.number.map(Constant::Number);
        let hex = constant.hex.as_ref().map(|hex| Self::Hex(hex.clone()));
        let bitpos = constant.bitpos.map(Constant::BitPos);
        let expr = constant
            .c_expression
            .as_ref()
            .map(|e| Self::CExpr(e.clone()));
        number.or(hex).or(bitpos).or(expr)
    }

    pub fn from_constant(constant: &vkxml::Constant) -> Self {
        let number = constant.number.map(Constant::Number);
        let hex = constant.hex.as_ref().map(|hex| Self::Hex(hex.clone()));
        let bitpos = constant.bitpos.map(Constant::BitPos);
        let expr = constant
            .c_expression
            .as_ref()
            .map(|e| Self::CExpr(e.clone()));
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
            version = format!("{version}_0")
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
    fn function_type(&self) -> FunctionType;
}

impl CommandExt for vk_parse::CommandDefinition {
    fn function_type(&self) -> FunctionType {
        let is_first_param_device = self.params.get(0).map_or(false, |field| {
            matches!(
                field.definition.type_name.as_deref(),
                Some("VkDevice" | "VkCommandBuffer" | "VkQueue")
            )
        });
        match self.proto.name.as_str() {
            "vkGetInstanceProcAddr" => FunctionType::Static,
            "vkCreateInstance"
            | "vkEnumerateInstanceLayerProperties"
            | "vkEnumerateInstanceExtensionProperties"
            | "vkEnumerateInstanceVersion" => FunctionType::Entry,
            // This is actually not a device level function
            "vkGetDeviceProcAddr" => FunctionType::Instance,
            _ if is_first_param_device => FunctionType::Device,
            _ => FunctionType::Instance,
        }
    }
}

pub trait FieldExt {
    /// Returns the name of the parameter that doesn't clash with Rusts reserved
    /// keywords
    fn param_ident(&self) -> Ident;

    /// The inner type of this field, with one level of pointers removed
    fn inner_type_tokens(
        &self,
        lifetime: Option<TokenStream>,
        inner_length: Option<usize>,
    ) -> TokenStream;

    /// Returns reference-types wrapped in their safe variant. (Dynamic) arrays become
    /// slices, pointers become Rust references.
    fn safe_type_tokens(
        &self,
        lifetime: TokenStream,
        type_lifetime: Option<TokenStream>,
        inner_length: Option<usize>,
    ) -> TokenStream;

    /// Returns the basetype ident and removes the 'Vk' prefix. When `is_ffi_param` is `true`
    /// array types (e.g. `[f32; 3]`) will be converted to pointer types (e.g. `&[f32; 3]`),
    /// which is needed for `C` function parameters. Set to `false` for struct definitions.
    fn type_tokens(&self, is_ffi_param: bool, type_lifetime: Option<TokenStream>) -> TokenStream;

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
            Self::Pointer => quote!(#r),
            Self::PointerToPointer => quote!(#r *mut),
            Self::PointerToConstPointer => quote!(#r *const),
        }
    }

    fn to_safe_tokens(&self, is_const: bool, lifetime: TokenStream) -> TokenStream {
        let r = if is_const {
            quote!(&#lifetime)
        } else {
            quote!(&#lifetime mut)
        };
        match self {
            Self::Pointer => quote!(#r),
            Self::PointerToPointer => quote!(#r *mut),
            Self::PointerToConstPointer => quote!(#r *const),
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
    format_ident!("{}", new_name)
}

/// Parses and rewrites a C literal into Rust
///
/// If no special pattern is recognized the original literal is returned.
/// Any new conversions need to be added to the [`parse_cexpr()`] [`nom`] parser.
///
/// Examples:
/// - `0x3FFU` -> `0x3ffu32`
fn convert_c_literal(lit: Literal) -> Literal {
    if let Ok(("", (_, rexpr))) = parse_cexpr(&lit.to_string()) {
        // lit::SynInt uses the same `.parse` method to create hexadecimal
        // literals because there is no `Literal` constructor for it.
        let mut stream = rexpr.parse::<TokenStream>().unwrap().into_iter();
        // If expression rewriting succeeds this should parse into a single literal
        match (stream.next(), stream.next()) {
            (Some(TokenTree::Literal(l)), None) => l,
            x => panic!("Stream must contain a single literal, not {x:?}"),
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
        .unwrap_or_else(|_| panic!("Failed to parse `{c_expr}` as Rust"));
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
        let name = self.name.as_deref().unwrap();
        let name_corrected = match name {
            "type" => "ty",
            _ => name,
        };
        format_ident!("{}", name_corrected.to_snake_case())
    }

    fn inner_type_tokens(
        &self,
        lifetime: Option<TokenStream>,
        inner_length: Option<usize>,
    ) -> TokenStream {
        assert!(!self.is_void());
        let ty = name_to_tokens(&self.basetype);

        let (const_, borrow) = match (lifetime, inner_length) {
            // If the nested "dynamic array" has length 1, it's just a pointer which we convert to a safe borrow for convenience
            (Some(lifetime), Some(1)) => (quote!(), quote!(&#lifetime)),
            _ => (quote!(const), quote!(*)),
        };

        match self.reference {
            Some(vkxml::ReferenceType::PointerToPointer) => quote!(#borrow mut #ty),
            Some(vkxml::ReferenceType::PointerToConstPointer) => quote!(#borrow #const_ #ty),
            _ => quote!(#ty),
        }
    }

    fn safe_type_tokens(
        &self,
        lifetime: TokenStream,
        type_lifetime: Option<TokenStream>,
        inner_length: Option<usize>,
    ) -> TokenStream {
        assert!(!self.is_void());
        match self.array {
            // The outer type fn type_tokens() returns is [], which fits our "safe" prescription
            Some(vkxml::ArrayType::Static) => self.type_tokens(false, type_lifetime),
            Some(vkxml::ArrayType::Dynamic) => {
                let ty = self.inner_type_tokens(Some(lifetime), inner_length);
                quote!([#ty #type_lifetime])
            }
            None => {
                let ty = name_to_tokens(&self.basetype);
                let pointer = self
                    .reference
                    .as_ref()
                    .map(|r| r.to_safe_tokens(self.is_const, lifetime));
                quote!(#pointer #ty #type_lifetime)
            }
        }
    }

    fn type_tokens(&self, is_ffi_param: bool, type_lifetime: Option<TokenStream>) -> TokenStream {
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
                    quote!([#ty #type_lifetime; #size])
                }
            }
            _ => {
                let pointer = self.reference.as_ref().map(|r| r.to_tokens(self.is_const));
                if self.is_pointer_to_static_sized_array() {
                    let size = self.c_size.as_ref().expect("Should have c_size");
                    let size = convert_c_expression(size, &BTreeMap::new());
                    quote!(#pointer [#ty #type_lifetime; #size])
                } else {
                    quote!(#pointer #ty #type_lifetime)
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

impl FieldExt for vk_parse::CommandParam {
    fn param_ident(&self) -> Ident {
        let name = self.definition.name.as_str();
        let name_corrected = match name {
            "type" => "ty",
            _ => name,
        };
        format_ident!("{}", name_corrected.to_snake_case())
    }

    fn inner_type_tokens(
        &self,
        _lifetime: Option<TokenStream>,
        _inner_length: Option<usize>,
    ) -> TokenStream {
        unimplemented!()
    }

    fn safe_type_tokens(
        &self,
        _lifetime: TokenStream,
        _type_lifetime: Option<TokenStream>,
        _inner_length: Option<usize>,
    ) -> TokenStream {
        unimplemented!()
    }

    fn type_tokens(&self, is_ffi_param: bool, type_lifetime: Option<TokenStream>) -> TokenStream {
        assert!(!self.is_void(), "{:?}", self);
        let (rem, ty) = parse_c_parameter(&self.definition.code).unwrap();
        assert!(rem.is_empty());
        let type_name = name_to_tokens(ty.type_.name);
        let type_name = quote!(#type_name #type_lifetime);
        let inner_ty = match ty.type_.reference_type {
            CReferenceType::Value => quote!(#type_name),
            CReferenceType::Pointer => {
                quote!(*mut #type_name)
            }
            CReferenceType::PointerToConst => quote!(*const #type_name),
            CReferenceType::PointerToPointer => quote!(*mut *mut #type_name),
            CReferenceType::PointerToPointerToConst => quote!(*mut *const #type_name),
            CReferenceType::PointerToConstPointer => quote!(*const *mut #type_name),
            CReferenceType::PointerToConstPointerToConst => quote!(*const *const #type_name),
        };

        match ty.static_array {
            None => inner_ty,
            Some(len) if is_ffi_param => quote!(*const [#inner_ty; #len]),
            Some(len) => quote!([#inner_ty; #len]),
        }
    }

    fn is_void(&self) -> bool {
        self.definition.type_name.as_deref() == Some("void")
            && self.len.is_none()
            && !self.definition.name.starts_with('p')
    }

    fn is_pointer_to_static_sized_array(&self) -> bool {
        unimplemented!()
    }
}

pub type CommandMap<'a> = HashMap<vkxml::Identifier, &'a vk_parse::CommandDefinition>;

fn generate_function_pointers<'a>(
    ident: Ident,
    commands: &[&'a vk_parse::CommandDefinition],
    rename_commands: &HashMap<&'a str, &'a str>,
    fn_cache: &mut HashSet<&'a str>,
    has_lifetimes: &HashSet<Ident>,
) -> TokenStream {
    // Commands can have duplicates inside them because they are declared per features. But we only
    // really want to generate one function pointer.
    let commands = commands
        .iter()
        .unique_by(|cmd| cmd.proto.name.as_str())
        .collect::<Vec<_>>();

    struct Command<'a> {
        type_needs_defining: bool,
        type_name: Ident,
        pfn_type_name: Ident,
        function_name_c: &'a str,
        function_name_rust: Ident,
        parameters: TokenStream,
        parameters_unused: TokenStream,
        returns: TokenStream,
        parameter_validstructs: Vec<(Ident, Vec<String>)>,
    }

    let commands = commands
        .iter()
        .map(|cmd| {
            let name = &cmd.proto.name;
            let pfn_type_name = format_ident!("PFN_{}", name);

            // We might need to generate a function pointer for an extension, where we are given the original
            // `cmd` and a rename back to the extension alias (typically with vendor suffix) in `rename_commands`:
            let function_name_c = rename_commands.get(name.as_str()).cloned().unwrap_or(name);

            let type_name = function_name_c.strip_prefix("vk").unwrap();
            let function_name_rust = format_ident!("{}", type_name.to_snake_case());
            let type_name = format_ident!("{}", type_name);

            let params = cmd
                .params
                .iter()
                .filter(|param| matches!(param.api.as_deref(), None | Some(DESIRED_API)));

            let params_tokens: Vec<_> = params
                .clone()
                .map(|param| {
                    let name = param.param_ident();
                    let type_lifetime = has_lifetimes
                        .contains(&name_to_tokens(
                            param.definition.type_name.as_ref().unwrap(),
                        ))
                        .then(|| quote!(<'_>));
                    let ty = param.type_tokens(true, type_lifetime);
                    (name, ty)
                })
                .collect();

            let params_iter = params_tokens
                .iter()
                .map(|(param_name, param_ty)| quote!(#param_name: #param_ty));
            let parameters = quote!(#(#params_iter,)*);

            let params_iter = params_tokens.iter().map(|(param_name, param_ty)| {
                let unused_name = format_ident!("_{}", param_name);
                quote!(#unused_name: #param_ty)
            });
            let parameters_unused = quote!(#(#params_iter,)*);

            let parameter_validstructs: Vec<_> = params
                .filter(|param| !param.validstructs.is_empty())
                .map(|param| (param.param_ident(), param.validstructs.clone()))
                .collect();

            let ret = cmd
                .proto
                .type_name
                .as_ref()
                .expect("Command must have return type");

            Command {
                // PFN function pointers are global and can not have duplicates.
                // This can happen because there are aliases to commands
                type_needs_defining: fn_cache.insert(name),
                type_name,
                pfn_type_name,
                function_name_c,
                function_name_rust,
                parameters,
                parameters_unused,
                returns: if ret == "void" {
                    quote!()
                } else {
                    let ret_ty_tokens = name_to_tokens(ret);
                    quote!(-> #ret_ty_tokens)
                },
                parameter_validstructs,
            }
        })
        .collect::<Vec<_>>();

    struct CommandToParamTraits<'a>(&'a Command<'a>);
    impl<'a> quote::ToTokens for CommandToParamTraits<'a> {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            for (param_ident, validstructs) in &self.0.parameter_validstructs {
                let param_ident = param_ident.to_string();
                let param_ident = param_ident
                    .strip_prefix("pp_")
                    .or_else(|| param_ident.strip_prefix("p_"))
                    .unwrap_or(&param_ident);

                let doc_string = format!(
                    "Implemented for all types that can be passed as argument to `{}` in [`{}`]",
                    param_ident, self.0.pfn_type_name
                );
                let param_trait_name = format_ident!(
                    "{}Param{}",
                    self.0.type_name,
                    param_ident.to_upper_camel_case()
                );
                quote! {
                    #[allow(non_camel_case_types)]
                    #[doc = #doc_string]
                    pub unsafe trait #param_trait_name {}
                }
                .to_tokens(tokens);

                for validstruct in validstructs {
                    let structname = name_to_tokens(validstruct);
                    quote!(unsafe impl #param_trait_name for #structname<'_> {}).to_tokens(tokens);
                }
            }
        }
    }

    struct CommandToType<'a>(&'a Command<'a>);
    impl<'a> quote::ToTokens for CommandToType<'a> {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let type_name = &self.0.pfn_type_name;
            let parameters = &self.0.parameters;
            let returns = &self.0.returns;
            quote!(
                #[allow(non_camel_case_types)]
                pub type #type_name = unsafe extern "system" fn(#parameters) #returns;
            )
            .to_tokens(tokens)
        }
    }

    struct CommandToMember<'a>(&'a Command<'a>);
    impl<'a> quote::ToTokens for CommandToMember<'a> {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let type_name = &self.0.pfn_type_name;
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

    struct CommandToLoader<'a>(&'a Command<'a>);
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
    let loaders = commands.iter().map(CommandToLoader);

    let loader = commands.is_empty().not().then(|| {
        quote! {
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
    });

    let param_traits = commands.iter().map(CommandToParamTraits);
    let pfn_typedefs = commands
        .iter()
        .filter(|pfn| pfn.type_needs_defining)
        .map(CommandToType);
    let members = commands.iter().map(CommandToMember);

    let struct_contents = if commands.is_empty() {
        quote! { pub struct #ident; }
    } else {
        quote! {
            pub struct #ident {
                #(#members,)*
            }

            unsafe impl Send for #ident {}
            unsafe impl Sync for #ident {}
        }
    };

    quote! {
        #(#param_traits)*
        #(#pfn_typedefs)*

        #[derive(Clone)]
        #struct_contents

        #loader
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
    fn is_deprecated(&self) -> bool {
        // We won't create this struct if the extension constant was deprecated
        false
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
        .filter_map(get_variant!(vk_parse::ExtensionChild::Require {
            api,
            items
        }))
        .filter(|(api, _items)| matches!(api.as_deref(), None | Some(DESIRED_API)))
        .flat_map(|(_api, items)| items);

    let mut extended_enums = BTreeMap::<String, Vec<ExtensionConstant<'_>>>::new();

    for item in items {
        if let vk_parse::InterfaceItem::Enum(enum_) = item {
            if !const_cache.insert(enum_.name.as_str()) {
                continue;
            }

            if !matches!(enum_.api.as_deref(), None | Some(DESIRED_API)) {
                continue;
            }

            if enum_.deprecated.is_some() {
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
        let doc_string = format!("Generated from '{extension_name}'");
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
    items: &'a [vk_parse::ExtensionChild],
    cmd_map: &CommandMap<'a>,
    cmd_aliases: &HashMap<&'a str, &'a str>,
    fn_cache: &mut HashSet<&'a str>,
    has_lifetimes: &HashSet<Ident>,
) -> TokenStream {
    let mut commands = Vec::new();
    let mut rename_commands = HashMap::new();
    let names = items
        .iter()
        .filter_map(get_variant!(vk_parse::ExtensionChild::Require {
            api,
            items
        }))
        .filter(|(api, _items)| matches!(api.as_deref(), None | Some(DESIRED_API)))
        .flat_map(|(_api, items)| items)
        .filter_map(get_variant!(vk_parse::InterfaceItem::Command { name }));

    // Collect a subset of `CommandDefinition`s to generate
    for name in names {
        let mut name = name.as_str();
        if let Some(&cmd) = cmd_aliases.get(name) {
            // This extension is referencing the base command under a different name,
            // make sure it is generated with a rename to it.
            rename_commands.insert(cmd, name);
            name = cmd;
        }

        commands.push(cmd_map[name]);
    }

    let ident = format_ident!(
        "{}Fn",
        extension_name
            .to_upper_camel_case()
            .strip_prefix("Vk")
            .unwrap()
    );
    let fp = generate_function_pointers(
        ident.clone(),
        &commands,
        &rename_commands,
        fn_cache,
        has_lifetimes,
    );

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

    let byte_name_ident = Literal::byte_string(format!("{extension_name}\0").as_bytes());
    let extension_cstr = quote! {
        impl #ident {
            pub const NAME: &'static ::std::ffi::CStr = unsafe {
                ::std::ffi::CStr::from_bytes_with_nul_unchecked(#byte_name_ident)
            };
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
    cmd_aliases: &HashMap<&'a str, &'a str>,
    fn_cache: &mut HashSet<&'a str>,
    has_lifetimes: &HashSet<Ident>,
) -> Option<TokenStream> {
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
        has_lifetimes,
    );
    let q = quote! {
        #fp
        #extension_tokens
    };
    Some(q)
}
pub fn generate_define(
    define: &vk_parse::Type,
    allowed_types: &HashSet<&str>,
    identifier_renames: &mut BTreeMap<String, Ident>,
) -> TokenStream {
    let vk_parse::TypeSpec::Code(spec) = &define.spec else {
        return quote!();
    };
    let [vk_parse::TypeCodeMarkup::Name(define_name), ..] = &spec.markup[..] else {
        return quote!();
    };

    if !allowed_types.contains(define_name.as_str()) {
        return quote!();
    }

    let name = constant_name(define_name);
    let ident = format_ident!("{}", name);

    if define_name.contains("VERSION") && !spec.code.contains("//#define") {
        let link = khronos_link(define_name);
        let (c_expr, (comment, (_name, parameters))) = parse_c_define_header(&spec.code).unwrap();
        let c_expr = c_expr.trim().trim_start_matches('\\');
        let c_expr = c_expr.replace("(uint32_t)", "");
        let c_expr = convert_c_expression(&c_expr, identifier_renames);
        let c_expr = discard_outmost_delimiter(c_expr);

        let deprecated = comment
            .and_then(|c| c.trim().strip_prefix("DEPRECATED: "))
            .map(|comment| quote!(#[deprecated = #comment]))
            .or_else(|| match define.deprecated.as_ref()?.as_str() {
                "true" => Some(quote!(#[deprecated])),
                "aliased" => {
                    Some(quote!(#[deprecated = "an old name not following Vulkan conventions"]))
                }
                x => panic!("Unknown deprecation reason {}", x),
            });

        let (code, ident) = if let Some(parameters) = parameters {
            let params = parameters
                .iter()
                .map(|param| format_ident!("{}", param))
                .map(|i| quote!(#i: u32));
            let ident = format_ident!("{}", name.to_lowercase());
            (
                quote!(pub const fn #ident(#(#params),*) -> u32 { #c_expr }),
                ident,
            )
        } else {
            (quote!(pub const #ident: u32 = #c_expr;), ident)
        };

        identifier_renames.insert(define_name.clone(), ident);

        quote! {
            #deprecated
            #[doc = #link]
            #code
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
        "_AMD",
        "_AMDX",
        "_ANDROID",
        "_ARM",
        "_BRCM",
        "_CHROMIUM",
        "_EXT",
        "_FB",
        "_FSL",
        "_FUCHSIA",
        "_GGP",
        "_GOOGLE",
        "_HUAWEI",
        "_IMG",
        "_INTEL",
        "_JUICE",
        "_KDAB",
        "_KHR",
        "_KHX",
        "_LUNARG",
        "_MESA",
        "_MSFT",
        "_MVK",
        "_NN",
        "_NV",
        "_NVX",
        "_NXP",
        "_NZXT",
        "_QCOM",
        "_QNX",
        "_RASTERGRID",
        "_RENDERDOC",
        "_SAMSUNG",
        "_SEC",
        "_TIZEN",
        "_VALVE",
        "_VIV",
        "_VSI",
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
                panic!("Failed to strip {struct_name} prefix from enum variant {variant_name}")
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
        .filter(|constant| !constant.is_deprecated())
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
    let ident = format_ident!("{}", clean_name);
    let constants = enum_
        .children
        .iter()
        .filter_map(get_variant!(vk_parse::EnumsChild::Enum))
        .filter(|constant| !constant.is_deprecated())
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
        let ident = format_ident!("{}", clean_name);

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

fn generate_result(ident: Ident, enum_: &vk_parse::Enums) -> TokenStream {
    let notation = enum_.children.iter().filter_map(|elem| {
        let (variant_name, notation) = match elem {
            vk_parse::EnumsChild::Enum(constant) => (
                &constant.name,
                constant.formatted_notation().unwrap_or(Cow::Borrowed("")),
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
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
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
    matches!(field.array, Some(vkxml::ArrayType::Static))
}

fn derive_default(
    struct_: &vkxml::Struct,
    members: &[PreprocessedMember<'_>],
    has_lifetime: bool,
) -> Option<TokenStream> {
    let name = name_to_tokens(&struct_.name);
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
    let contains_ptr = members
        .iter()
        .any(|member| member.vkxml_field.reference.is_some());
    let contains_structure_type = members
        .iter()
        .any(|member| is_structure_type(member.vkxml_field));
    let contains_static_array = members
        .iter()
        .any(|member| is_static_array(member.vkxml_field));
    let contains_deprecated = members.iter().any(|member| member.deprecated.is_some());
    let allow_deprecated = contains_deprecated.then(|| quote!(#[allow(deprecated)]));
    if !(contains_ptr || contains_structure_type || contains_static_array) {
        return None;
    };
    let default_fields = members.iter().map(|member| {
        let param_ident = member.vkxml_field.param_ident();
        if is_structure_type(member.vkxml_field) {
            if member.vkxml_field.type_enums.is_some() {
                quote!(#param_ident: Self::STRUCTURE_TYPE)
            } else {
                quote!(#param_ident: unsafe { ::std::mem::zeroed() })
            }
        } else if member.vkxml_field.reference.is_some() {
            if member.vkxml_field.is_const {
                quote!(#param_ident: ::std::ptr::null())
            } else {
                quote!(#param_ident: ::std::ptr::null_mut())
            }
        } else if is_static_array(member.vkxml_field)
            || handles.contains(&member.vkxml_field.basetype.as_str())
        {
            quote!(#param_ident: unsafe { ::std::mem::zeroed() })
        } else {
            let ty = member.vkxml_field.type_tokens(false, None);
            quote!(#param_ident: #ty::default())
        }
    });
    let lifetime = has_lifetime.then(|| quote!(<'_>));
    let marker = has_lifetime.then(|| quote!(_marker: PhantomData,));
    let q = quote! {
        impl ::std::default::Default for #name #lifetime {
            #[inline]
            fn default() -> Self {
                #allow_deprecated
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

fn derive_debug(
    struct_: &vkxml::Struct,
    members: &[PreprocessedMember<'_>],
    union_types: &HashSet<&str>,
    has_lifetime: bool,
) -> Option<TokenStream> {
    let name = name_to_tokens(&struct_.name);
    let contains_pfn = members.iter().any(|member| {
        member
            .vkxml_field
            .name
            .as_ref()
            .map(|n| n.contains("pfn"))
            .unwrap_or(false)
    });
    let contains_static_array = members
        .iter()
        .any(|member| is_static_array(member.vkxml_field) && member.vkxml_field.basetype == "char");
    let contains_union = members
        .iter()
        .any(|member| union_types.contains(member.vkxml_field.basetype.as_str()));
    if !(contains_union || contains_static_array || contains_pfn) {
        return None;
    }
    let debug_fields = members.iter().map(|member| {
        let field = &member.vkxml_field;
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
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_struct(#name_str)
                #(#debug_fields)*
                .finish()
            }
        }
    };
    Some(q)
}

fn derive_setters(
    struct_: &vkxml::Struct,
    members: &[PreprocessedMember<'_>],
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

    let name = name_to_tokens(&struct_.name);

    let next_field = members
        .iter()
        .find(|member| member.vkxml_field.param_ident() == "p_next");

    let structure_type_field = members
        .iter()
        .find(|member| member.vkxml_field.param_ident() == "s_type");

    // Must either have both, or none:
    assert_eq!(next_field.is_some(), structure_type_field.is_some());

    let allowed_count_members = [
        // pViewports is allowed to be empty if the viewport state is empty
        ("VkPipelineViewportStateCreateInfo", "viewportCount"),
        // Must match viewportCount
        ("VkPipelineViewportStateCreateInfo", "scissorCount"),
        // descriptorCount is settable regardless of having pImmutableSamplers
        ("VkDescriptorSetLayoutBinding", "descriptorCount"),
        // No ImageView attachments when VK_FRAMEBUFFER_CREATE_IMAGELESS_BIT is set
        ("VkFramebufferCreateInfo", "attachmentCount"),
        // descriptorCount also describes descriptor length in pNext extension structures
        // https://github.com/ash-rs/ash/issues/806
        ("VkWriteDescriptorSet", "descriptorCount"),
    ];
    let skip_members = members
        .iter()
        .filter_map(|member| {
            let field = &member.vkxml_field;

            // Associated _count members
            if field.array.is_some() {
                if let Some(array_size) = &field.size {
                    if !allowed_count_members.contains(&(&struct_.name, array_size)) {
                        return Some(array_size);
                    }
                }
            }

            if let Some(objecttype) = &member.vk_parse_type_member.objecttype {
                let objecttype_field = members
                    .iter()
                    .find(|m| m.vkxml_field.name.as_ref().unwrap() == objecttype)
                    .unwrap();
                // Extensions using this type are deprecated exactly because of the existence of VkObjectType, hence
                // there won't be an additional ash trait to support VkDebugReportObjectTypeEXT.
                // See also https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_debug_utils.html#_description
                if objecttype_field.vkxml_field.basetype != "VkDebugReportObjectTypeEXT" {
                    return Some(objecttype);
                }
            }

            None
        })
        .collect::<Vec<_>>();

    let setters = members.iter().filter_map(|member| {
        let field = &member.vkxml_field;

        let name = field.name.as_ref().unwrap();
        if skip_members.contains(&name) {
            return None;
        }

        let deprecated = member.deprecated.as_ref().map(|d| quote!(#d #[allow(deprecated)]));
        let param_ident = field.param_ident();
        let type_lifetime = has_lifetimes
            .contains(&name_to_tokens(&field.basetype))
            .then(|| quote!(<'a>));
        let param_ty_tokens = field.safe_type_tokens(quote!('a), type_lifetime.clone(), None);

        let param_ident_string = param_ident.to_string();
        if param_ident_string == "s_type" || param_ident_string == "p_next" {
            return None;
        }

        let param_ident_short = param_ident_string
            .strip_prefix("p_")
            .or_else(|| param_ident_string.strip_prefix("pp_"))
            .unwrap_or(&param_ident_string);
        let mut param_ident_short = format_ident!("{}", param_ident_short);

        // Unique cases
        if struct_.name == "VkShaderModuleCreateInfo" && name == "codeSize" {
            return None;
        }

        if struct_.name == "VkShaderModuleCreateInfo" && name == "pCode" {
            return Some(quote! {
                #[inline]
                pub fn code(mut self, code: &'a [u32]) -> Self {
                    self.code_size = code.len() * 4;
                    self.p_code = code.as_ptr();
                    self
                }
            });
        }

        if name == "pSampleMask" {
            return Some(quote! {
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

        // TODO: Improve in future when https://github.com/rust-lang/rust/issues/53667 is merged id:6
        if field.reference.is_some() {
            if field.basetype == "char" && matches!(field.reference, Some(vkxml::ReferenceType::Pointer)) {
                assert!(field.null_terminate);
                assert_eq!(field.size, None);
                return Some(quote!{
                    #[inline]
                    #deprecated
                    pub fn #param_ident_short(mut self, #param_ident_short: &'a ::std::ffi::CStr) -> Self {
                        self.#param_ident = #param_ident_short.as_ptr();
                        self
                    }
                });
            }

            if matches!(field.array, Some(vkxml::ArrayType::Dynamic)) {
                if let Some(ref array_size) = field.size {
                    let mut slice_param_ty_tokens = field.safe_type_tokens(quote!('a), type_lifetime.clone(), None);

                    let mut ptr = if field.is_const {
                        quote!(.as_ptr())
                    } else {
                        quote!(.as_mut_ptr())
                    };

                    // Interpret void array as byte array
                    if field.basetype == "void" && matches!(field.reference, Some(vkxml::ReferenceType::Pointer)) {
                        slice_param_ty_tokens = quote!([u8]);
                        ptr = quote!(#ptr.cast());
                    };

                    let set_size_stmt = if field.is_pointer_to_static_sized_array() {
                        // this is a pointer to a piece of memory with statically known size.
                        let array_size = field.c_size.as_ref().unwrap();
                        let c_size = convert_c_expression(array_size, &BTreeMap::new());
                        let inner_type = field.inner_type_tokens(None, None);

                        slice_param_ty_tokens = quote!([#inner_type; #c_size]);
                        ptr = quote!();

                        quote!()
                    } else {
                        // Deal with a "special" 2D dynamic array with an inner size of 1 (effectively an array containing pointers to single objects)
                        let array_size = if let Some(array_size) = array_size.strip_suffix(",1") {
                            param_ident_short = format_ident!("{}_ptrs", param_ident_short);
                            slice_param_ty_tokens = field.safe_type_tokens(quote!('a), type_lifetime.clone(), Some(1));
                            ptr = quote!(#ptr.cast());
                            array_size
                        } else {
                            array_size
                        };

                        let array_size_ident = format_ident!("{}", array_size.to_snake_case());

                        let size_field = members.iter().find(|member| member.vkxml_field.name.as_deref() == Some(array_size)).unwrap();

                        let cast = if size_field.vkxml_field.basetype == "size_t" {
                            quote!()
                        } else {
                            quote!(as _)
                        };

                        quote!(self.#array_size_ident = #param_ident_short.len()#cast;)
                    };

                    let mutable = if field.is_const { quote!() } else { quote!(mut) };

                    return Some(quote! {
                        #[inline]
                        #deprecated
                        pub fn #param_ident_short(mut self, #param_ident_short: &'a #mutable #slice_param_ty_tokens) -> Self {
                            #set_size_stmt
                            self.#param_ident = #param_ident_short #ptr;
                            self
                        }
                    });
                }
            }
        }

        if field.basetype == "VkBool32" {
            return Some(quote!{
                #[inline]
                #deprecated
                pub fn #param_ident_short(mut self, #param_ident_short: bool) -> Self {
                    self.#param_ident = #param_ident_short.into();
                    self
                }
            });
        }

        if let Some(objecttype) = &member.vk_parse_type_member.objecttype {
                let objecttype_field = members
                    .iter()
                    .find(|m| m.vkxml_field.name.as_ref().unwrap() == objecttype)
                    .unwrap();

            // Extensions using this type are deprecated exactly because of the existence of VkObjectType, hence
            // there won't be an additional ash trait to support VkDebugReportObjectTypeEXT.
            if objecttype_field.vkxml_field.basetype != "VkDebugReportObjectTypeEXT" {
                let objecttype_ident = format_ident!("{}", objecttype.to_snake_case());

                return Some(quote!{
                    #[inline]
                    #deprecated
                    pub fn #param_ident_short<T: Handle>(mut self, #param_ident_short: T) -> Self {
                        self.#param_ident = #param_ident_short.as_raw();
                        self.#objecttype_ident = T::TYPE;
                        self
                    }
                });
            }
        };

        let param_ty_tokens = if is_opaque_type(&field.basetype) {
            //  Use raw pointers for void/opaque types
            field.type_tokens(false, type_lifetime)
        } else {
            param_ty_tokens
        };

        Some(quote!{
            #[inline]
            #deprecated
            pub fn #param_ident_short(mut self, #param_ident_short: #param_ty_tokens) -> Self {
                self.#param_ident = #param_ident_short;
                self
            }
        })
    });

    let extends_name = format_ident!("Extends{}", name);

    // The `p_next` field should only be considered if this struct is also a root struct
    let root_struct_next_field = next_field.filter(|_| root_structs.contains(&name));

    // We only implement a next method for root structs with a `pnext` field.
    let next_function = if let Some(next_member) = root_struct_next_field {
        let next_field = &next_member.vkxml_field;
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

    let impl_structure_type_trait = structure_type_field.map(|member| {
        let value = member
            .vkxml_field
            .type_enums
            .as_deref()
            .expect("s_type field must have a value in `vk.xml`");

        assert!(!value.contains(','));

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

struct PreprocessedMember<'a> {
    vkxml_field: &'a vkxml::Field,
    vk_parse_type_member: &'a vk_parse::TypeMemberDefinition,
    deprecated: Option<TokenStream>,
}

pub fn generate_struct(
    struct_: &vkxml::Struct,
    vk_parse_types: &HashMap<String, &vk_parse::Type>,
    root_structs: &HashSet<Ident>,
    union_types: &HashSet<&str>,
    has_lifetimes: &HashSet<Ident>,
) -> TokenStream {
    let name = name_to_tokens(&struct_.name);
    let vk_parse_struct = vk_parse_types[&struct_.name];
    let vk_parse::TypeSpec::Members(vk_parse_members) = &vk_parse_struct.spec else {
        panic!()
    };

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
        .filter_map(get_variant!(vkxml::StructElement::Member))
        .zip(
            vk_parse_members
                .iter()
                .filter_map(get_variant!(vk_parse::TypeMember::Definition)),
        )
        .filter(|(_, vk_parse_field)| {
            matches!(vk_parse_field.api.as_deref(), None | Some(DESIRED_API))
        })
        .map(|(field, vk_parse_field)| {
            let deprecated = vk_parse_field
                .deprecated
                .as_ref()
                .map(|deprecated| match deprecated.as_str() {
                    "true" => quote!(#[deprecated]),
                    "ignored" => {
                        quote!(#[deprecated = "functionality described by this member no longer operates"])
                    }
                    x => panic!("Unknown deprecation reason {}", x),
                });
                PreprocessedMember {
                    vkxml_field: field,
                    vk_parse_type_member: vk_parse_field,
                    deprecated,
                }
        })
        .collect::<Vec<_>>();

    let params = members.iter().map(|member| {
        let field = &member.vkxml_field;
        let deprecated = &member.deprecated;
        let param_ident = field.param_ident();
        let param_ty_tokens = if field.basetype == struct_.name {
            let pointer = field
                .reference
                .as_ref()
                .map(|r| r.to_tokens(field.is_const));
            quote!(#pointer Self)
        } else {
            let type_lifetime = has_lifetimes
                .contains(&name_to_tokens(&field.basetype))
                .then(|| quote!(<'a>));
            field.type_tokens(false, type_lifetime)
        };

        quote!(#deprecated pub #param_ident: #param_ty_tokens)
    });

    let has_lifetime = has_lifetimes.contains(&name);
    let (lifetimes, marker) = match has_lifetime {
        true => (quote!(<'a>), quote!(pub _marker: PhantomData<&'a ()>,)),
        false => (quote!(), quote!()),
    };

    let debug_tokens = derive_debug(struct_, &members, union_types, has_lifetime);
    let default_tokens = derive_default(struct_, &members, has_lifetime);
    let setter_tokens = derive_setters(struct_, &members, root_structs, has_lifetimes);
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
fn generate_funcptr(fnptr: &vkxml::FunctionPointer, has_lifetimes: &HashSet<Ident>) -> TokenStream {
    let name = format_ident!("{}", fnptr.name);
    let ret_ty_tokens = if fnptr.return_type.is_void() {
        quote!()
    } else {
        let ret_ty_tokens = fnptr.return_type.type_tokens(true, None);
        quote!(-> #ret_ty_tokens)
    };
    let params = fnptr.param.iter().map(|field| {
        let ident = field.param_ident();
        let type_lifetime = has_lifetimes
            .contains(&name_to_tokens(&field.basetype))
            .then(|| quote!(<'_>));
        let type_tokens = field.type_tokens(true, type_lifetime);
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
        let type_lifetime = has_lifetimes
            .contains(&name_to_tokens(&field.basetype))
            .then(|| quote!(<'a>));
        let ty = field.type_tokens(false, type_lifetime);
        quote! {
            pub #name: #ty
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
pub fn root_structs(
    definitions: &[&vk_parse::Type],
    allowed_types: &HashSet<&str>,
) -> HashSet<Ident> {
    // Loop over all structs and collect their extends
    definitions
        .iter()
        .filter(|type_| {
            type_
                .name
                .as_ref()
                .map_or(false, |name| allowed_types.contains(name.as_str()))
        })
        .filter_map(|type_| type_.structextends.as_ref())
        .flat_map(|e| e.split(','))
        .map(name_to_tokens)
        .collect()
}
pub fn generate_definition_vk_parse(
    definition: &vk_parse::Type,
    allowed_types: &HashSet<&str>,
    identifier_renames: &mut BTreeMap<String, Ident>,
) -> Option<TokenStream> {
    if let Some(api) = &definition.api {
        if api != DESIRED_API {
            return None;
        }
    }

    match definition.category.as_deref() {
        Some("define") => Some(generate_define(
            definition,
            allowed_types,
            identifier_renames,
        )),
        _ => None,
    }
}
#[allow(clippy::too_many_arguments)]
pub fn generate_definition(
    definition: &vkxml::DefinitionsElement,
    allowed_types: &HashSet<&str>,
    union_types: &HashSet<&str>,
    root_structs: &HashSet<Ident>,
    has_lifetimes: &HashSet<Ident>,
    vk_parse_types: &HashMap<String, &vk_parse::Type>,
    bitflags_cache: &mut HashSet<Ident>,
    const_values: &mut BTreeMap<Ident, ConstantTypeInfo>,
) -> Option<TokenStream> {
    match *definition {
        vkxml::DefinitionsElement::Typedef(ref typedef)
            if allowed_types.contains(typedef.name.as_str()) =>
        {
            Some(generate_typedef(typedef))
        }
        vkxml::DefinitionsElement::Struct(ref struct_)
            if allowed_types.contains(struct_.name.as_str()) =>
        {
            Some(generate_struct(
                struct_,
                vk_parse_types,
                root_structs,
                union_types,
                has_lifetimes,
            ))
        }
        vkxml::DefinitionsElement::Bitmask(ref mask)
            if allowed_types.contains(mask.name.as_str()) =>
        {
            generate_bitmask(mask, bitflags_cache, const_values)
        }
        vkxml::DefinitionsElement::Handle(ref handle)
            if allowed_types.contains(handle.name.as_str()) =>
        {
            generate_handle(handle)
        }
        vkxml::DefinitionsElement::FuncPtr(ref fp) if allowed_types.contains(fp.name.as_str()) => {
            Some(generate_funcptr(fp, has_lifetimes))
        }
        vkxml::DefinitionsElement::Union(ref union)
            if allowed_types.contains(union.name.as_str()) =>
        {
            Some(generate_union(union, has_lifetimes))
        }
        _ => None,
    }
}
pub fn generate_feature<'a>(
    feature: &vkxml::Feature,
    commands: &CommandMap<'a>,
    fn_cache: &mut HashSet<&'a str>,
    has_lifetimes: &HashSet<Ident>,
) -> TokenStream {
    if !contains_desired_api(&feature.api) {
        return quote!();
    }

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
            has_lifetimes,
        )
    } else {
        quote! {}
    };
    let entry = generate_function_pointers(
        format_ident!("EntryFnV{}", version),
        &entry_commands,
        &HashMap::new(),
        fn_cache,
        has_lifetimes,
    );
    let instance = generate_function_pointers(
        format_ident!("InstanceFnV{}", version),
        &instance_commands,
        &HashMap::new(),
        fn_cache,
        has_lifetimes,
    );
    let device = generate_function_pointers(
        format_ident!("DeviceFnV{}", version),
        &device_commands,
        &HashMap::new(),
        fn_cache,
        has_lifetimes,
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
        .filter(|feature| contains_desired_api(&feature.api))
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
                    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
                    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
                        "Header `{name}` being redefined",
                    );

                    let (rem, path) = parse_c_include(&code.code)
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
    allowed_types: &HashSet<&str>,
    has_lifetimes: &HashSet<Ident>,
    ty_cache: &mut HashSet<Ident>,
) -> TokenStream {
    let aliases = types
        .children
        .iter()
        .filter_map(get_variant!(vk_parse::TypesChild::Type))
        .filter_map(|ty| {
            let name = ty.name.as_ref()?;
            if !allowed_types.contains(name.as_str()) {
                return None;
            }
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
    let extensions: Vec<&vk_parse::Extension> = spec2
        .0
        .iter()
        .find_map(get_variant!(vk_parse::RegistryChild::Extensions))
        .expect("extension")
        .children
        .iter()
        .filter(|e| {
            if let Some(supported) = &e.supported {
                contains_desired_api(supported) ||
                // VK_ANDROID_native_buffer is for internal use only, but types defined elsewhere
                // reference enum extension constants.  Exempt the extension from this check until
                // types are properly folded in with their extension (where applicable).
                e.name == "VK_ANDROID_native_buffer"
            } else {
                true
            }
        })
        .collect();

    let spec = vk_parse::parse_file_as_vkxml(&vk_xml).expect("Invalid xml file.");

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

    let constants: Vec<&vkxml::Constant> = spec
        .elements
        .iter()
        .filter_map(get_variant!(vkxml::RegistryElement::Constants))
        .flat_map(|constants| &constants.elements)
        .collect();

    let features_children = spec2
        .0
        .iter()
        .filter_map(get_variant!(vk_parse::RegistryChild::Feature))
        .filter(|feature| contains_desired_api(&feature.api))
        .flat_map(|features| &features.children);

    let extension_children = extensions.iter().flat_map(|extension| &extension.children);

    let (required_types, required_commands) = features_children
        .chain(extension_children)
        .filter_map(get_variant!(vk_parse::FeatureChild::Require { api, items }))
        .filter(|(api, _items)| matches!(api.as_deref(), None | Some(DESIRED_API)))
        .flat_map(|(_api, items)| items)
        .fold((HashSet::new(), HashSet::new()), |mut acc, elem| {
            match elem {
                vk_parse::InterfaceItem::Type { name, .. } => {
                    acc.0.insert(name.as_str());
                }
                vk_parse::InterfaceItem::Command { name, .. } => {
                    acc.1.insert(name.as_str());
                }
                _ => {}
            };
            acc
        });

    let commands: CommandMap<'_> = spec2
        .0
        .iter()
        .filter_map(get_variant!(vk_parse::RegistryChild::Commands))
        .flat_map(|cmds| &cmds.children)
        .filter_map(get_variant!(vk_parse::Command::Definition))
        .filter(|cmd| required_commands.contains(&cmd.proto.name.as_str()))
        .map(|cmd| (cmd.proto.name.clone(), cmd))
        .collect();

    let cmd_aliases: HashMap<_, _> = spec2
        .0
        .iter()
        .filter_map(get_variant!(vk_parse::RegistryChild::Commands))
        .flat_map(|cmds| &cmds.children)
        .filter_map(get_variant!(vk_parse::Command::Alias { name, alias }))
        .filter(|(name, _alias)| required_commands.contains(name.as_str()))
        .map(|(name, alias)| (name.as_str(), alias.as_str()))
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
        .filter(|enums| {
            enums.name.as_ref().map_or(true, |n| {
                required_types.contains(n.replace("FlagBits", "Flags").as_str())
            })
        })
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
        .filter(|s| {
            s.elements
                .iter()
                .filter_map(get_variant!(vkxml::StructElement::Member))
                .any(|x| x.reference.is_some())
        })
        .map(|s| name_to_tokens(&s.name))
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
    for type_ in spec2
        .0
        .iter()
        .filter_map(get_variant!(vk_parse::RegistryChild::Types))
        .flat_map(|types| &types.children)
        .filter_map(get_variant!(vk_parse::TypesChild::Type))
    {
        if let (Some(name), Some(alias)) = (&type_.name, &type_.alias) {
            if has_lifetimes.contains(&name_to_tokens(alias)) {
                has_lifetimes.insert(name_to_tokens(name));
            }
        }
    }

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
                &has_lifetimes,
            )
        })
        .collect_vec();

    let vk_parse_types = spec2
        .0
        .iter()
        .filter_map(get_variant!(vk_parse::RegistryChild::Types))
        .flat_map(|ty| {
            ty.children
                .iter()
                .filter_map(get_variant!(vk_parse::TypesChild::Type))
        })
        .collect::<Vec<_>>();
    let vk_parse_definitions: Vec<_> = vk_parse_types
        .iter()
        .filter_map(|def| {
            generate_definition_vk_parse(def, &required_types, &mut identifier_renames)
        })
        .collect();

    let root_structs = root_structs(&vk_parse_types, &required_types);

    let vk_parse_types = vk_parse_types
        .into_iter()
        .filter_map(|t| t.name.clone().map(|n| (n, t)))
        .collect::<HashMap<_, _>>();
    let definition_code: Vec<_> = vk_parse_definitions
        .into_iter()
        .chain(definitions.into_iter().filter_map(|def| {
            generate_definition(
                def,
                &required_types,
                &union_types,
                &root_structs,
                &has_lifetimes,
                &vk_parse_types,
                &mut bitflags_cache,
                &mut const_values,
            )
        }))
        .collect();

    let mut ty_cache = HashSet::new();
    let aliases: Vec<_> = spec2
        .0
        .iter()
        .filter_map(get_variant!(vk_parse::RegistryChild::Types))
        .map(|ty| generate_aliases_of_types(ty, &required_types, &has_lifetimes, &mut ty_cache))
        .collect();

    let feature_code: Vec<_> = features
        .iter()
        .map(|feature| generate_feature(feature, &commands, &mut fn_cache, &has_lifetimes))
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

    let vk_features_file = File::create(vk_dir.join("features.rs")).expect("vk/features.rs");
    let vk_definitions_file =
        File::create(vk_dir.join("definitions.rs")).expect("vk/definitions.rs");
    let vk_enums_file = File::create(vk_dir.join("enums.rs")).expect("vk/enums.rs");
    let vk_bitflags_file = File::create(vk_dir.join("bitflags.rs")).expect("vk/bitflags.rs");
    let vk_constants_file = File::create(vk_dir.join("constants.rs")).expect("vk/constants.rs");
    let vk_extensions_file = File::create(vk_dir.join("extensions.rs")).expect("vk/extensions.rs");
    let vk_feature_extensions_file =
        File::create(vk_dir.join("feature_extensions.rs")).expect("vk/feature_extensions.rs");
    let vk_const_debugs_file =
        File::create(vk_dir.join("const_debugs.rs")).expect("vk/const_debugs.rs");
    let vk_aliases_file = File::create(vk_dir.join("aliases.rs")).expect("vk/aliases.rs");

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
        #![allow(unused_qualifications)] // Because we do not know in what file the PFNs are defined

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

    fn write_formatted(text: &[u8], out: File) -> std::process::Child {
        let mut child = std::process::Command::new("rustfmt")
            .stdin(std::process::Stdio::piped())
            .stdout(out)
            .spawn()
            .expect("Failed to spawn `rustfmt`");
        let mut stdin = child.stdin.take().expect("Failed to open stdin");
        stdin.write_all(text).unwrap();
        drop(stdin);
        child
    }

    let processes = [
        write_formatted(feature_code.to_string().as_bytes(), vk_features_file),
        write_formatted(definition_code.to_string().as_bytes(), vk_definitions_file),
        write_formatted(enum_code.to_string().as_bytes(), vk_enums_file),
        write_formatted(bitflags_code.to_string().as_bytes(), vk_bitflags_file),
        write_formatted(constants_code.to_string().as_bytes(), vk_constants_file),
        write_formatted(extension_code.to_string().as_bytes(), vk_extensions_file),
        write_formatted(
            feature_extensions_code.to_string().as_bytes(),
            vk_feature_extensions_file,
        ),
        write_formatted(const_debugs.to_string().as_bytes(), vk_const_debugs_file),
        write_formatted(aliases.to_string().as_bytes(), vk_aliases_file),
    ];
    for mut p in processes {
        let status = p.wait().unwrap();
        assert!(status.success());
    }

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
