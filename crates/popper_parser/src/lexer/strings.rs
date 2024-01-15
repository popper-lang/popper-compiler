use popper_ast::StringLiteral;

use crate::parse::Parser;
use crate::cursor::Cursor;
use crate::error::{Error, ErrorType};
use crate::expect::Expect;


impl<I> Parser<I> for StringLiteral
where I: Iterator<Item = char> + Clone {
    fn parse(cursor: &mut Cursor<I>) -> Result<Self, Error> {
        cursor.start_recording();
        let mut buf = String::new();

        let next = cursor.next();
        if let Some(c) = next {
            if c == '"' {
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
            let next = cursor.peek().unwrap();
            if next != '"' {
                buf.push(next);
                cursor.next();
            } else {
                break;
            }
        }

        let next = cursor.next();
        if let Some(c) = next {
            if c == '"' {
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

        // remove quotes
        buf.remove(0);
        buf.pop();

        Ok(StringLiteral::new(cursor.end_recording(), buf))
    }
}

impl<I> Expect<I> for StringLiteral
where I: Iterator<Item = char> + Clone {
    fn expect(&self, cursor: &mut Cursor<I>) -> Result<Self, Error> {
        let res = Self::parse(cursor)?;
        Ok(res)
    }
}