use crate::cursor::Cursor;
use crate::error::Error;
use crate::error::ErrorType;


pub trait Expect<I>: Sized
where I: Iterator + Clone,
      I::Item: PartialEq + Clone {
    fn expect(&self, cursor: &mut Cursor<I>) -> Result<Self, Error>;
}

// str
impl<I> Expect<I> for &str
where I: Iterator<Item = char> + Clone {
    fn expect(&self, cursor: &mut Cursor<I>) -> Result<Self, Error> {
        let mut buf = String::new();
        let len = self.len();

        for _ in 0..len {
            let next = cursor.next();
            if let Some(c) = next {
                buf.push(c);
            } else {
                return Err(Error {
                    error: ErrorType::UnexpectedEOF,
                    span: Default::default(),
                })
            }
        }

        if buf == *self {
            Ok(self)
        } else {
            Err(Error {
                error: ErrorType::ExpectedToken {
                    expected: self.to_string(),
                    got: buf,
                },
                span: Default::default(),
            })
        }
    }
}

// char
impl<I> Expect<I> for char
where I: Iterator<Item = char> + Clone {
    fn expect(&self, cursor: &mut Cursor<I>) -> Result<Self, Error> {
        let next = cursor.next();
        if let Some(c) = next {
            if c == *self {
                Ok(self.clone())
            } else {
                Err(Error {
                    error: ErrorType::ExpectedToken {
                        expected: self.to_string(),
                        got: c.to_string(),
                    },
                    span: Default::default(),
                })
            }
        } else {
            Err(Error {
                error: ErrorType::UnexpectedEOF,
                span: Default::default(),
            })
        }
    }
}
