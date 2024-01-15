use popper_ast::Span;
use crate::cursor::Cursor;
use crate::error::Error;
use crate::parse::Parser;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Ignore<P, I>
where P: Parser<I>,
      I: Iterator + Clone,
      I::Item: PartialEq + Clone {
    _marker_iter: std::marker::PhantomData<I>,
    _marker_parser: std::marker::PhantomData<P>,
}

impl<P, I> Ignore<P, I>
    where P: Parser<I>,
          I: Iterator + Clone,
          I::Item: PartialEq + Clone {
    pub fn new() -> Self {
        Self {
            _marker_iter: std::marker::PhantomData,
            _marker_parser: std::marker::PhantomData,
        }
    }
}

impl<P, I> Parser<I> for Ignore<P, I>
    where P: Parser<I>,
          I: Iterator<Item = char> + Clone, {
    fn parse(cursor: &mut Cursor<I>) -> Result<Self, Error> {
        let mut cloned = cursor.clone();
        cursor.filter(move |e| {
            dbg!(&e);
            dbg!(P::parse(
                &mut cloned
            ).is_err())
        });

        // while ! cloned.at_end() {
        //     let old_pos = cloned.pos;
        //     let res = P::parse(&mut cloned);
        //     let new_pos = cloned.pos;
        //     if res.is_ok() {
        //         (old_pos..new_pos).for_each(|e| { cursor.remove(e); });
        //         n += new_pos - old_pos;
        //         dbg!(cursor.collect());
        //     } else {
        //
        //     }
        // }

        Ok(Self::new())

    }
}