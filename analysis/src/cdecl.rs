use std::num::NonZeroU8;

/// Identifier-category-aware minimal tokenization of a subset of C syntax,
/// sufficient for parsing the C declarations used in `vk.xml`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CTok<'a> {
    /// Identifier referring to a type declaration in scope.
    TypeName(&'a str),

    /// Identifier referring to a value declaration in scope.
    ValueName(&'a str),

    /// Identifier that is being presently declared (exactly one per `CDecl`).
    DeclName(&'a str),

    /// Supported keyword (one of [`CTok::SUPPORTED_KEYWORDS`]).
    Kw(&'static str),

    /// Any ASCII punctuation (i.e. as determined by [`char::is_ascii_punctuation`]).
    // FIXME(eddyb) this could really use the `std::ascii` API.
    Punct(char),

    /// Integer literal (for e.g. array lengths).
    IntLit(&'a str),

    /// Unknown identifier (all known cases are spec bugs or deficiencies).
    StrayIdent(&'a str),
}

#[derive(Debug)]
pub struct UnsupportedCTok<'a>(#[allow(dead_code)] &'a str);

impl<'a> CTok<'a> {
    pub const SUPPORTED_KEYWORDS: &'static [&'static str] = &["const", "struct", "typedef", "void"];

    pub(crate) fn lex_into(
        s: &'a str,
        out: &mut impl Extend<CTok<'a>>,
    ) -> Result<(), UnsupportedCTok<'a>> {
        // FIXME(eddyb) this could really use the `std::ascii` API.
        let mut s = s;
        while let Some(c) = s.chars().next() {
            if !c.is_ascii() {
                return Err(UnsupportedCTok(s));
            }

            let is_ident_or_number = |c: char| c.is_ascii_alphanumeric() || c == '_';
            let tok = if is_ident_or_number(c) {
                let len = s.chars().take_while(|&c| is_ident_or_number(c)).count();
                let (tok, rest) = s.split_at(len);
                s = rest;
                if c.is_ascii_digit() {
                    CTok::IntLit(tok)
                } else if let Some(kw) = CTok::SUPPORTED_KEYWORDS.iter().find(|&&kw| kw == tok) {
                    CTok::Kw(kw)
                } else {
                    CTok::StrayIdent(tok)
                }
            } else if c.is_ascii_punctuation() {
                s = &s[1..];
                CTok::Punct(c)
            } else if c.is_ascii_whitespace() {
                s = s.trim_start();
                continue;
            } else {
                return Err(UnsupportedCTok(s));
            };
            out.extend([tok]);
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CDecl<'a> {
    pub ty: CType<'a>,
    pub name: &'a str,
    pub bitfield_width: Option<NonZeroU8>,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CDeclMode {
    TypeDef,
    StructMember,
    FuncParam,
    FuncTypeParam,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CType<'a> {
    Base(CBaseType<'a>),
    Ptr {
        implicit_for_decay: bool,
        is_const: bool,
        pointee: Box<CType<'a>>,
    },
    Array {
        element: Box<CType<'a>>,
        len: CArrayLen<'a>,
    },
    Func {
        ret_ty: Option<Box<CType<'a>>>,
        params: Vec<CDecl<'a>>,
    },
}

impl CType<'_> {
    pub const VOID: CType<'static> = CType::Base(CBaseType {
        struct_tag: false,
        name: "void",
    });
}

#[derive(Debug, PartialEq, Eq)]
pub struct CBaseType<'a> {
    pub struct_tag: bool,
    pub name: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CArrayLen<'a> {
    Named(&'a str),
    Literal(u128),
}

#[derive(Debug)]
pub struct CDeclParseError<'a, 'b> {
    pub kind: CDeclParseErrorKind<'a>,
    pub tokens: &'b [CTok<'a>],
}

#[derive(Debug)]
pub enum CDeclParseErrorKind<'a> {
    Missing(&'static str),
    Multiple(&'static str),
    Unused(&'static str),
    InvalidIntLit(std::num::ParseIntError),
    UnsupportedLeftmostToken(CTok<'a>),
    UnsupportedRightmostToken(CTok<'a>),
    UnbalancedBrackets,
    UnsupportedArrayLength,
}

impl<'a> CDecl<'a> {
    // HACK(eddyb) this split is literally just to simplify error tracking.
    pub(crate) fn parse<'b>(
        mode: CDeclMode,
        tokens: &'b [CTok<'a>],
    ) -> Result<CDecl<'a>, CDeclParseError<'a, 'b>> {
        CDecl::parse_inner(mode, tokens).map_err(|kind| CDeclParseError { kind, tokens })
    }
    fn parse_inner<'b>(
        mode: CDeclMode,
        tokens: &'b [CTok<'a>],
    ) -> Result<CDecl<'a>, CDeclParseErrorKind<'a>> {
        use CDeclParseErrorKind as ErrorKind;

        trait InsertIfNone<T> {
            fn insert_if_none(&mut self, value: T) -> Option<&mut T>;
        }
        impl<T> InsertIfNone<T> for Option<T> {
            fn insert_if_none(&mut self, value: T) -> Option<&mut T> {
                self.is_none().then(|| self.insert(value))
            }
        }

        let (mut left, decl_name, mut right) = {
            let mut decl_names =
                tokens
                    .iter()
                    .copied()
                    .enumerate()
                    .filter_map(|(i, tok)| match tok {
                        CTok::DeclName(name) => Some((i, name)),

                        // HACK(eddyb) this is only allowed due to the (few)
                        // function pointer typedefs in `vk.xml`, which don't
                        // label parameter names in any special way.
                        CTok::StrayIdent(name) if mode == CDeclMode::FuncTypeParam => {
                            Some((i, name))
                        }

                        _ => None,
                    });
            match (decl_names.next(), decl_names.next()) {
                (Some((i, name)), None) => (&tokens[..i], name, &tokens[i + 1..]),
                (None, _) => return Err(ErrorKind::Missing("DeclName")),
                (Some(_), Some(_)) => return Err(ErrorKind::Multiple("DeclName")),
            }
        };

        if mode == CDeclMode::TypeDef {
            // NOTE(eddyb) `typedef` can appear later on as well, so this is
            // unnecessarily strict, but it avoids much more involved tracking.
            left = left
                .strip_prefix(&[CTok::Kw("typedef")])
                .ok_or(ErrorKind::Missing("typedef"))?;
            right = right
                .strip_suffix(&[CTok::Punct(';')])
                .ok_or(ErrorKind::Missing(";"))?;
        }

        let bitfield_width = match right {
            [rest @ .., CTok::Punct(':'), CTok::IntLit(width_lit)]
                if mode == CDeclMode::StructMember =>
            {
                right = rest;
                Some(width_lit.parse().map_err(ErrorKind::InvalidIntLit)?)
            }
            _ => None,
        };

        // FIXME(eddyb) deduplicate qualifier parsing somehow.
        let mut const_qualif = match left {
            [CTok::Kw("const"), rest @ ..] => {
                left = rest;
                Some(())
            }
            _ => None,
        };

        let mut ty = CType::Base(match left {
            [CTok::Kw("struct"), CTok::TypeName(name), rest @ ..] => {
                left = rest;
                CBaseType {
                    struct_tag: true,
                    name,
                }
            }
            [CTok::TypeName(name) | CTok::Kw(name @ "void"), rest @ ..] => {
                left = rest;
                CBaseType {
                    struct_tag: false,
                    name,
                }
            }
            _ => return Err(ErrorKind::Missing("TypeName")),
        });

        // This is the core of the C declaration parsing strategy: we have some
        // type `T` (held in the variable `ty`) and tokens to either side of the
        // name being declared, and at every step of the loops below there is a
        // "closest binding" (postfix) "type operator", which we pattern-match
        // from its side and then apply to `T`, replacing `T` with any of:
        // - `T*` pointers (like Rust `*T`), from `T* ...`
        //   (only `left` side "type operator", and it takes precedence, making
        //    array-of-pointers much easier to spell out than pointer-to-array)
        // - `T[N]` arrays (like Rust `[T; N]`), from `T ...[N]`
        // - `T(A, B, C)` functions, from `T ...(A, B, C)`
        //   (Rust only has pointers to such types, `fn(A, B, C) -> T`)
        //
        // Notably, both sides are consumed outside-in (`left` LTR, `right` RTL),
        // converging on the middle (where the name being declared is), and that
        // can get confusing (an older comment below also tried to explain it).
        //
        // Once we run out of "type operators", and the declaration isn't trivial,
        // only syntax left is parenthesization *around* the name being declared,
        // with everything inside the parentheses applying *on top of* everything
        // outside: but we've consumed everything outside so we're actually left
        // with `T (...)` and we can simply drop the parentheses!
        while !left.is_empty() || !right.is_empty() {
            while let Some((&leftmost, after_leftmost)) = left.split_first() {
                match leftmost {
                    CTok::Kw("const") => {
                        const_qualif
                            .insert_if_none(())
                            .ok_or(ErrorKind::Multiple("const"))?;
                    }
                    CTok::Punct('*') => {
                        ty = CType::Ptr {
                            implicit_for_decay: false,
                            is_const: const_qualif.take().is_some(),
                            pointee: Box::new(ty),
                        };
                    }

                    // Outermost parentheses around the name being declared,
                    // handled together after both `left` and `right` loops.
                    CTok::Punct('(') => break,

                    _ => return Err(ErrorKind::UnsupportedLeftmostToken(leftmost)),
                }
                left = after_leftmost;
            }
            'right: while let Some(&rightmost) = right.last() {
                // NOTE(eddyb) outermost (i.e. rightmost) suffixes apply first,
                // and the only way this is "intuitive" is that e.g. a 2D array
                // like `T m[A][B]` means `typeof(m[i][j]) = T`, and the lvalue
                // syntax has to match the declaration (so `i < A` and `j < B`),
                // IOW it's equivalent to `(T[B]) m[A]` / `typeof((m[i])[j]) = T`
                // (if C had type parenthesization, or via C++ type aliases).
                match rightmost {
                    CTok::Punct(']' | ')') => {}

                    _ => return Err(ErrorKind::UnsupportedRightmostToken(rightmost)),
                }

                // As `rightmost` is `]`/`)`, the matching `[`/`(` must be found.
                let (before_rightmost_group, rightmost_group) = {
                    let mut i = right.len() - 1;
                    let mut nesting = 0;
                    loop {
                        let checked_dec =
                            |x: usize| x.checked_sub(1).ok_or(ErrorKind::UnbalancedBrackets);
                        match right[i] {
                            CTok::Punct(']' | ')') => nesting += 1,
                            CTok::Punct('[' | '(') => nesting = checked_dec(nesting)?,
                            _ => {}
                        }
                        if nesting == 0 {
                            break;
                        }

                        // Outermost parentheses around the name being declared,
                        // handled together after both `left` and `right` loops.
                        if i == 0 && rightmost == CTok::Punct(')') {
                            break 'right;
                        }

                        i = checked_dec(i)?;
                    }
                    right.split_at(i)
                };

                match rightmost_group {
                    [CTok::Punct('['), len @ .., CTok::Punct(']')] => {
                        ty = CType::Array {
                            element: Box::new(ty),
                            len: match len {
                                [CTok::ValueName(name)] => CArrayLen::Named(name),
                                [CTok::IntLit(lit)] => CArrayLen::Literal(
                                    lit.parse().map_err(ErrorKind::InvalidIntLit)?,
                                ),
                                _ => return Err(ErrorKind::UnsupportedArrayLength),
                            },
                        };
                    }
                    [CTok::Punct('('), params @ .., CTok::Punct(')')] => {
                        if const_qualif.is_some() {
                            return Err(ErrorKind::Unused("const"));
                        }

                        let params = match params {
                            [] => return Err(ErrorKind::Missing("parameters")),
                            [CTok::Kw("void")] => vec![],
                            _ => params
                                .split(|&tok| tok == CTok::Punct(','))
                                .map(|param| CDecl::parse_inner(CDeclMode::FuncTypeParam, param))
                                .collect::<Result<_, _>>()?,
                        };
                        ty = CType::Func {
                            ret_ty: Some(ty).filter(|ty| *ty != CType::VOID).map(Box::new),
                            params,
                        };
                    }
                    _ => return Err(ErrorKind::UnbalancedBrackets),
                }
                right = before_rightmost_group;
            }

            // Outermost parentheses around the name being declared, handled here
            // to ensure there is nothing else left around them, and can therefore
            // be cleanly removed.
            if let ([CTok::Punct('('), left_inner @ ..], [right_inner @ .., CTok::Punct(')')]) =
                (left, right)
            {
                left = left_inner;
                right = right_inner;
            }
        }

        // NOTE(eddyb) parameters to functions decay "into" pointers, but because
        // we control the typesystem, we can keep both the array types, and the
        // implicit pointer, closer to Rust e.g. `&[T; N]` arguments.
        if let (CDeclMode::FuncParam, CType::Array { .. }) = (mode, &ty) {
            ty = CType::Ptr {
                implicit_for_decay: true,
                is_const: const_qualif.take().is_some(),
                pointee: Box::new(ty),
            };
        }

        if const_qualif.is_some() {
            return Err(ErrorKind::Unused("const"));
        }

        Ok(CDecl {
            ty,
            name: decl_name,
            bitfield_width,
        })
    }
}
