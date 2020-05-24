use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{char, digit1, hex_digit1, multispace0},
    combinator::{all_consuming, map, map_res, not, opt, peek},
    error::VerboseError,
    multi::separated_nonempty_list,
    number::complete::float,
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
pub enum CDecoration {
    None,
    Pointer,
    PointerToConst,
    PointerToPointer,
    PointerToConstPointerToConst,
}

#[derive(Debug)]
pub struct CType<'a> {
    pub name: &'a str,
    pub decoration: CDecoration,
    pub array_size: Option<&'a str>,
}

#[derive(Debug)]
pub struct CVariableDecl<'a> {
    pub name: &'a str,
    pub ty: CType<'a>,
}

#[derive(Debug)]
pub struct CFunctionDecl<'a> {
    pub proto: CVariableDecl<'a>,
    pub parameters: Vec<CVariableDecl<'a>>,
}

#[derive(Debug)]
pub enum CExpr {
    Literal(usize),
    Uint32(u32),
    Uint64(u64),
    Float(f32),
}

fn ignore_remainder<T>((_i, o): (&str, T)) -> T {
    o
}

type Res<'a, T> = IResult<&'a str, T, VerboseError<&'a str>>;

fn version_num(i: &str) -> Res<u16> {
    map_res(digit1, str::parse::<u16>)(i)
}

fn version(i: &str) -> Res<(u16, u16)> {
    tuple((
        preceded(tag("VK_VERSION_"), version_num),
        preceded(tag("_"), version_num),
    ))(i)
}

pub fn c_try_parse_version(i: &str) -> Option<(u16, u16)> {
    all_consuming(version)(i).map(ignore_remainder).ok()
}

fn parse_i32(i: &str) -> Res<i32> {
    alt((
        preceded(
            tag("0x"),
            map_res(hex_digit1, |s: &str| i32::from_str_radix(s, 16)),
        ),
        preceded(char('-'), map(map_res(digit1, str::parse::<i32>), |n| -n)),
        map_res(digit1, str::parse::<i32>),
    ))(i)
}

pub fn c_parse_int(i: &str) -> i32 {
    all_consuming(parse_i32)(i)
        .map(ignore_remainder)
        .unwrap_or_else(|res| panic!("parse fail: {} -> {:?}", i, res))
}

fn is_ident(c: char) -> bool {
    match c {
        'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => true,
        _ => false,
    }
}

fn ident(i: &str) -> Res<&str> {
    preceded(multispace0, take_while1(is_ident))(i)
}

fn keyword<'a>(k: &'static str) -> impl Fn(&'a str) -> Res<'a, &str> {
    delimited(multispace0, tag(k), not(peek(take_while1(is_ident))))
}

fn op<'a>(c: char) -> impl Fn(&'a str) -> Res<'a, char> {
    preceded(multispace0, char(c))
}

fn variable_decl(i: &str) -> Res<CVariableDecl> {
    let (i, const0) = opt(keyword("const"))(i)?;
    let (i, _) = opt(keyword("struct"))(i)?;
    let (i, type_name) = ident(i)?;
    let (i, ptr0) = opt(op('*'))(i)?;
    let (i, const1) = opt(keyword("const"))(i)?;
    let (i, ptr1) = opt(op('*'))(i)?;
    let (i, var_name) = ident(i)?;
    let (i, array_size) = opt(delimited(op('['), ident, op(']')))(i)?;
    Ok((
        i,
        CVariableDecl {
            name: var_name,
            ty: CType {
                name: type_name,
                decoration: match (
                    const0.is_some(),
                    ptr0.is_some(),
                    const1.is_some(),
                    ptr1.is_some(),
                ) {
                    (false, false, false, false) | (true, false, false, false) => CDecoration::None,
                    (false, true, false, false) => CDecoration::Pointer,
                    (true, true, false, false) => CDecoration::PointerToConst,
                    (false, true, false, true) => CDecoration::PointerToPointer,
                    (true, true, true, true) => CDecoration::PointerToConstPointerToConst,
                    v => panic!("unsupported decoration {:?}", v),
                },
                array_size,
            },
        },
    ))
}

pub fn c_try_parse_variable_decl(i: &str) -> Option<CVariableDecl> {
    all_consuming(terminated(variable_decl, multispace0))(i)
        .map(ignore_remainder)
        .ok()
}

pub fn c_parse_variable_decl(i: &str) -> CVariableDecl {
    all_consuming(terminated(variable_decl, multispace0))(i)
        .map(ignore_remainder)
        .unwrap_or_else(|res| panic!("parse fail: {} -> {:?}", i, res))
}

fn function_decl(i: &str) -> Res<CFunctionDecl> {
    let (i, ret_type_name) = ident(i)?;
    let (i, ret_ptr) = opt(op('*'))(i)?;
    let (i, func_name) = ident(i)?;
    let (i, parameters) = delimited(
        op('('),
        alt((
            separated_nonempty_list(op(','), variable_decl),
            map(keyword("void"), |_| Vec::new()),
        )),
        tuple((op(')'), op(';'))),
    )(i)?;
    Ok((
        i,
        CFunctionDecl {
            proto: CVariableDecl {
                name: func_name,
                ty: CType {
                    name: ret_type_name,
                    decoration: if ret_ptr.is_some() {
                        CDecoration::Pointer
                    } else {
                        CDecoration::None
                    },
                    array_size: None,
                },
            },
            parameters,
        },
    ))
}

pub fn c_parse_function_decl(i: &str) -> CFunctionDecl {
    all_consuming(terminated(function_decl, multispace0))(i)
        .map(ignore_remainder)
        .unwrap_or_else(|res| panic!("parse fail: {} -> {:?}", i, res))
}

fn function_ptr_typedef<'a>(i: &'a str) -> Res<'a, CFunctionDecl> {
    let (i, ret_type_name) = preceded(keyword("typedef"), ident)(i)?;
    let (i, ret_ptr) = opt(op('*'))(i)?;
    let (i, func_name) = delimited(
        tuple((op('('), keyword("VKAPI_PTR"), op('*'))),
        ident,
        op(')'),
    )(i)?;
    let (i, parameters) = delimited(
        op('('),
        alt((
            separated_nonempty_list(op(','), variable_decl),
            map(keyword("void"), |_| Vec::new()),
        )),
        tuple((op(')'), op(';'))),
    )(i)?;
    Ok((
        i,
        CFunctionDecl {
            proto: CVariableDecl {
                name: func_name,
                ty: CType {
                    name: ret_type_name,
                    decoration: if ret_ptr.is_some() {
                        CDecoration::Pointer
                    } else {
                        CDecoration::None
                    },
                    array_size: None,
                },
            },
            parameters,
        },
    ))
}

pub fn c_parse_func_pointer_typedef(i: &str) -> CFunctionDecl {
    all_consuming(terminated(function_ptr_typedef, multispace0))(i)
        .map(ignore_remainder)
        .unwrap_or_else(|res| panic!("parse fail: {} -> {:?}", i, res))
}

fn typedef(i: &str) -> Res<CVariableDecl> {
    let (i, type_name) = preceded(keyword("typedef"), ident)(i)?;
    let (i, var_name) = terminated(ident, op(';'))(i)?;
    Ok((
        i,
        CVariableDecl {
            name: var_name,
            ty: CType {
                name: type_name,
                decoration: CDecoration::None,
                array_size: None,
            },
        },
    ))
}

pub fn c_try_parse_typedef(i: &str) -> Option<CVariableDecl> {
    all_consuming(terminated(typedef, multispace0))(i)
        .map(ignore_remainder)
        .ok()
}

fn expr_inner(i: &str) -> Res<CExpr> {
    alt((
        map(terminated(float, char('f')), CExpr::Float),
        map(
            terminated(map_res(digit1, str::parse::<u64>), tag("ULL")),
            CExpr::Uint64,
        ),
        map(
            terminated(map_res(digit1, str::parse::<u32>), tag("U")),
            CExpr::Uint32,
        ),
        map(map_res(digit1, str::parse::<usize>), CExpr::Literal),
        delimited(char('('), expr, char(')')),
        map(preceded(char('~'), expr_inner), |e| match e {
            CExpr::Uint32(x) => CExpr::Uint32(!x),
            CExpr::Uint64(x) => CExpr::Uint64(!x),
            _ => panic!("cannot bitwise invert unsized literal"),
        }),
    ))(i)
}

fn expr(i: &str) -> Res<CExpr> {
    alt((
        map(
            separated_pair(expr_inner, char('-'), expr_inner),
            |(a, b)| match a {
                CExpr::Uint32(x) => match b {
                    CExpr::Uint32(y) => CExpr::Uint32(x - y),
                    CExpr::Literal(y) => CExpr::Uint32(x - y as u32),
                    _ => panic!("bad rhs type in arithmetic"),
                },
                _ => panic!("bad lhs type in arithmetic"),
            },
        ),
        expr_inner,
    ))(i)
}

pub fn c_parse_expr(i: &str) -> CExpr {
    all_consuming(expr)(i)
        .map(ignore_remainder)
        .unwrap_or_else(|res| panic!("parse fail: {} -> {:?}", i, res))
}
