#[cfg(test)]
mod test;
mod ident;
mod strings;
mod int;

use popper_ast::Span;
use crate::parse::Parser;
use crate::error::Error;
use crate::cursor::Cursor;
use crate::expect::Expect;

macro_rules! tokens {
    ($($name:ident($c:ident) $b:block),*) => {
        $(
        #[derive(Debug, Clone, Default, PartialEq)]
        pub struct $name {
            pub span: Span,
        }

        impl $name {
            pub fn new(span: Span) -> Self {
                Self {
                    span,
                }
            }

            pub fn zero() -> Self {
                Self::new(Default::default())
            }
        }

        impl<I> Parser<I> for $name
        where I: Iterator<Item = char> + Clone {
            fn parse($c: &mut Cursor<I>) -> Result<Self, Error> {
                $c.start_recording();
                $b
                Ok($name {
                    span: $c.end_recording(),
                })
            }
        }

        impl<I> Expect<I> for $name
        where I: Iterator<Item = char> + Clone {
            fn expect(&self, cursor: &mut Cursor<I>) -> Result<Self, Error> {
                let res = Self::parse(cursor)?;
                Ok(res)
            }
        }

        impl ToString for $name {
            fn to_string(&self) -> String {
                stringify!($name).to_string()
            }
        }

        )*
    };
}

// operators
tokens![
    Lt(cursor) {
        cursor.expect('<')?;
    },
    Gt(cursor) {
        cursor.expect('>')?;
    },
    Le(cursor) {
        cursor.expect("<=")?;
    },
    Ge(cursor) {
        cursor.expect(">=")?;
    },
    Eq(cursor) {
        cursor.expect("==")?;
    },
    Ne(cursor) {
        cursor.expect("!=")?;
    },
    Plus(cursor) {
        cursor.expect('+')?;
    },
    Minus(cursor) {
        cursor.expect('-')?;
    },
    Star(cursor) {
        cursor.expect('*')?;
    },
    Slash(cursor) {
        cursor.expect('/')?;
    },
    Percent(cursor) {
        cursor.expect('%')?;
    },
    Space(cursor) {
        cursor.expect(' ')?;
    }
];

// keyword
tokens![
    Let(cursor) {
        cursor.expect("let")?;
    },
    Struct(cursor) {
        cursor.expect("struct")?;
    },
    Func(cursor) {
        cursor.expect("func")?;
    },
    Return(cursor) {
        cursor.expect("return")?;
    },
    If(cursor) {
        cursor.expect("if")?;
    },
    Else(cursor) {
        cursor.expect("else")?;
    },
    While(cursor) {
        cursor.expect("while")?;
    },
    Import(cursor) {
        cursor.expect("import")?;
    },
    External(cursor) {
        cursor.expect("external")?;
    }
];

// punctuation
tokens![
    Colon(cursor) {
        cursor.expect(':')?;
    }, // :
    Semicolon(cursor) {
        cursor.expect(';')?;
    }, // ;
    Comma(cursor) {
        cursor.expect(',')?;
    }, // ,
    Dot(cursor) {
        cursor.expect('.')?;
    }, // .
    LParen(cursor) {
        cursor.expect('(')?;
    }, // (
    RParen(cursor) {
        cursor.expect(')')?;
    }, // )
    LBrace(cursor) {
        cursor.expect('{')?;
    }, // {
    RBrace(cursor) {
        cursor.expect('}')?;
    }, // }
    LBracket(cursor) {
        cursor.expect('[')?;
    }, // [
    RBracket(cursor) {
        cursor.expect(']')?;
    } // ]
];