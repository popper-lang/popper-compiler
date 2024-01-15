use popper_ast::Int;

use crate::parse::Parser;
use crate::cursor::Cursor;
use crate::error::{Error, ErrorType};
use crate::expect::Expect;

impl<I> Parser<I> for Int
where I: Iterator<Item = char> + Clone {
    fn parse(cursor: &mut Cursor<I>) -> Result<Self, Error> {
        cursor.start_recording();
        let mut buf = String::new();

        let next = cursor.next();
        if let Some(c) = next {
            if c.is_numeric() {
                buf.push(c);
            } else {
                return Err(Error {
                    error: ErrorType::UnexpectedToken(c.to_string()),
                    span: Default::default(),
                })
            }
        } else {
            return Err(Error {
                error: ErrorType::UnexpectedEOF,
                span: Default::default(),
            })
        }

        loop {
            let next = cursor.peek();
            if let Some(c) = next {
                if c.is_numeric() {
                    buf.push(c);
                    cursor.next();
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        Ok(Int::new(cursor.end_recording(), buf.parse().unwrap()))
    }
}

impl<I> Expect<I> for Int
where I: Iterator<Item = char> + Clone {
    fn expect(&self, cursor: &mut Cursor<I>) -> Result<Self, Error> {
        let res = Self::parse(cursor)?;
        Ok(res)
    }
}