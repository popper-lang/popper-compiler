
use popper_ast::Span;
use crate::error::{Error, ErrorType};
use crate::parse::Parser;

#[derive(Clone)]
pub struct Cursor<I>
where I: Iterator + Clone,
      I::Item: PartialEq + Clone {
    pub iter: I,
    pub pos: usize,
    pub rec: usize
}

impl<I> Cursor<I>
where   I: Iterator + Clone,
        I::Item: PartialEq + Clone
{
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            pos: 0,
            rec: 0,
        }
    }

    pub fn start_recording(&mut self) {
        self.rec = self.pos;
    }

    pub fn end_recording(&mut self) -> Span {
        Span::new(self.rec, self.pos)
    }

    pub fn peek(&mut self) -> I::Item {
        self.iter.nth(self.pos).unwrap()
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<I::Item> {
        self.pos += 1;
        self.iter.next()
    }

    pub fn prev(&mut self) -> Option<I::Item> {
        self.pos -= 1;
        self.iter.nth(self.pos)
    }

    pub fn len(self) -> usize {
        self.iter.count()
    }

    pub fn is_empty(self) -> bool {
        self.len() == 0
    }

    pub fn expect<P: Parser<I> + PartialEq + ToString>(
        &mut self,
        expected: P
    ) -> Result<P, Error> {
        let got = P::parse(self)?;
        if got == expected {
            Ok(got)
        } else {
            Err(Error {
                error: ErrorType::ExpectedToken {
                    expected: expected.to_string(),
                    got: got.to_string(),
                },
                span: Default::default(),
            })
        }

    }

}