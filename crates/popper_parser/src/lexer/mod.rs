#[cfg(test)]
mod test;

use popper_ast::Span;
use crate::parse::Parser;
use crate::error::Error;
use crate::cursor::Cursor;

macro_rules! tokens {
    ($($name:ident($c:ident) $b:block),*) => {
        $(
        #[derive(Debug, Clone, PartialEq)]
        pub struct $name {
            pub span: Span,
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
        cursor.expect('<')?;
        cursor.expect('=')?;
    },
    Ge(cursor) {
        cursor.expect('>')?;
        cursor.expect('=')?;
    },
    Eq(cursor) {
        cursor.expect('=')?;
        cursor.expect('=')?;
    },
    Ne(cursor) {
        cursor.expect('!')?;
        cursor.expect('=')?;
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
    }
];

// keyword
tokens![
    Let(cursor) {
        cursor.expect('l')?;
        cursor.expect('e')?;
        cursor.expect('t')?;
    },
    Struct(cursor) {
        cursor.expect('s')?;
        cursor.expect('t')?;
        cursor.expect('r')?;
        cursor.expect('u')?;
        cursor.expect('c')?;
        cursor.expect('t')?;
    },
    Func(cursor) {
        cursor.expect('f')?;
        cursor.expect('u')?;
        cursor.expect('n')?;
        cursor.expect('c')?;
    },
    Return(cursor) {
        cursor.expect('r')?;
        cursor.expect('e')?;
        cursor.expect('t')?;
        cursor.expect('u')?;
        cursor.expect('r')?;
        cursor.expect('n')?;
    },
    If(cursor) {
        cursor.expect('i')?;
        cursor.expect('f')?;
    },
    Else(cursor) {
        cursor.expect('e')?;
        cursor.expect('l')?;
        cursor.expect('s')?;
        cursor.expect('e')?;
    },
    While(cursor) {
        cursor.expect('w')?;
        cursor.expect('h')?;
        cursor.expect('i')?;
        cursor.expect('l')?;
        cursor.expect('e')?;
    },
    Import(cursor) {
        cursor.expect('i')?;
        cursor.expect('m')?;
        cursor.expect('p')?;
        cursor.expect('o')?;
        cursor.expect('r')?;
        cursor.expect('t')?;
    },
    External(cursor) {
        cursor.expect('e')?;
        cursor.expect('x')?;
        cursor.expect('t')?;
        cursor.expect('e')?;
        cursor.expect('r')?;
        cursor.expect('n')?;
        cursor.expect('a')?;
        cursor.expect('l')?;
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