use crate::cursor::Cursor;
use crate::error::{Error, ErrorType};

pub trait Parser<I>: Sized
where I: Iterator + Clone,
      I::Item: PartialEq + Clone {
    fn parse(cursor: &mut Cursor<I>) -> Result<Self, Error>;
}

impl<I> Parser<I> for char
where I: Iterator<Item = char> + Clone {
    fn parse(cursor: &mut Cursor<I>) -> Result<Self, Error> {
        let got = cursor.next();
        if let Some(c) = got {
            Ok(c)
        } else {
            Err(Error {
                error: ErrorType::UnexpectedEOF,
                span: Default::default(),
            })
        }
    }
}