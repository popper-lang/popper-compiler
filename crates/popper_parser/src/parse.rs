use crate::cursor::Cursor;
use crate::error::Error;

pub trait Parser<I>: Sized
where I: Iterator + Clone,
      I::Item: PartialEq + Clone {
    fn parse(cursor: &mut Cursor<I>) -> Result<Self, Error>;
}