use itertools::Itertools;
use std::collections::HashMap;
pub fn render(template_str: &str, args: &[(&str, &str)]) -> String {
    let mut render = template_str.to_string();
    for (variable, value) in args {
        let pattern = format!("{{{{{}}}}}", variable);
        render = render.replace(&pattern, value);
    }
    render
}

use std::fmt::{self, Write};

#[derive(Default)]
pub struct Formatter {
    current_ident: u32,
    data: String,
}
impl Formatter {
    pub fn ident(&self) -> u32 {
        self.current_ident
    }

    pub fn build(self) -> String {
        self.data
    }
    pub fn write_ident(&mut self, ident: u32) -> fmt::Result {
        for _ in 0..ident {
            write!(self, " ")?
        }
        Ok(())
    }

    pub fn write_sourcecode(&mut self, src: &dyn Sourcecode) -> fmt::Result {
        src.fmt(self)
    }
}

impl Write for Formatter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if s.contains('\n') {
            self.current_ident = 0;
        } else {
            self.current_ident += s.chars().count() as u32;
        }
        write!(&mut self.data, "{}", s)?;
        Ok(())
    }
}

pub trait Sourcecode {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result;
}

impl<'a> Sourcecode for &'a str {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "{}", self)
    }
}

impl<'a> Sourcecode for String {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        self.as_str().fmt(fmt)
    }
}
impl<T> Sourcecode for Vec<T>
where
    T: Sourcecode,
{
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        let ident = fmt.ident();
        for (idx, item) in self.iter().enumerate() {
            if idx != 0 {
                fmt.write_ident(ident)?;
            }
            item.fmt(fmt)?;
            if idx != self.len() as _ {
                write!(fmt, "\n")?;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
enum Code<'a> {
    Str(&'a str),
    Ident(&'a str),
}

fn ranges<'a>(src: &'a str) -> impl Iterator<Item = (usize, usize)> + 'a {
    src.match_indices("#").map(|(i, _)| i).tuples()
}
fn code<'a>(src: &'a str) -> impl Iterator<Item = Vec<Code<'a>>> + 'a {
    src.lines().map(|src| {
        let (mut items, range) =
            ranges(src).fold((Vec::new(), None), |(mut items, last_range), range| {
                let start = last_range.map(|(_, end)| end + 1).unwrap_or(0);
                let end = range.0;
                items.push(Code::Str(&src[start..end]));
                items.push(Code::Ident(&src[range.0 + 1..range.1]));
                (items, Some(range))
            });

        if let Some(range) = range {
            items.push(Code::Str(&src[range.1 + 1..]));
        }
        if items.is_empty() {
            vec![Code::Str(src)]
        } else {
            items
        }
    })
}

pub fn source(src: &str, map: &HashMap<&str, &dyn Sourcecode>) -> String {
    let mut fmt = Formatter::default();

    for line in code(src) {
        for code in line {
            match code {
                Code::Str(s) => {
                    fmt.write_sourcecode(&s);
                }
                Code::Ident(ident) => {
                    if let Some(&source_code) = map.get(ident) {
                        fmt.write_sourcecode(source_code);
                    }
                }
            };
        }
        writeln!(fmt);
    }

    crate::remove_ident_from_string(&fmt.build())
}

#[macro_export]
macro_rules! source {
    ($code: expr, $($ident: ident = $value: ident,)*) => {
        {
            use std::collections::HashMap;
            use $crate::template::Sourcecode;

            let mut map: HashMap<&str, &dyn Sourcecode> = HashMap::new();
            $(
                map.insert(stringify!($ident), &$value as &dyn Sourcecode);
            )*

            $crate::template::source($code, &map)
        }
    }
}
