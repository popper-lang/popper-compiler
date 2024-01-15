use std::fmt::{Debug, Display};
use popper_ast::Span;
use crate::cursor::Cursor;
use crate::error::Error;
use crate::expect::Expect;
use crate::parse::Parser;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Or<P1, P2, I>
where P1: Parser<I>,
      I: Iterator + Clone,
      I::Item: PartialEq + Clone {
    pub span: Span,
    pub p1: Option<P1>,
    pub p2: Option<P2>,
    _marker: std::marker::PhantomData<I>,
}


impl<P1, P2, I> Or<P1, P2, I>
    where P1: Parser<I>,
          I: Iterator + Clone,
          I::Item: PartialEq + Clone {
    pub fn new(span: Span, p1: Option<P1>, p2: Option<P2>) -> Self {
        Self {
            span,
            p1,
            p2,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<P1, P2, I> Or<P1, P2, I>
where   P1: Parser<I> + Default,
        P2: Parser<I> + Default,
        I: Iterator + Clone,
        I::Item: PartialEq + Clone {
        #[allow(dead_code)]
        pub fn zero() -> Self {
            Self::new(Default::default(), Some(P1::default()), Some(P2::default()))
        }
    }

impl<P1, P2, I> Parser<I> for Or<P1, P2, I>
    where P1: Parser<I>,
          P2: Parser<I>,
          I: Iterator + Clone,
          I::Item: PartialEq + Clone {
    fn parse(cursor: &mut Cursor<I>) -> Result<Self, Error> {
        let mut cloned = cursor.clone();
        cloned.start_recording();
        let p1 = P1::parse(&mut cloned);
        if let Ok(res_p1) = p1 {
            cursor.from(cloned.clone());
            Ok(Self::new(cloned.end_recording(), Some(res_p1), None))

        } else {
            let mut cloned = cursor.clone();
            cloned.start_recording();
            let p2 = P2::parse(&mut cloned);
            if let Ok(res_p2) = p2 {
                cursor.from(cloned.clone());
                Ok(Self::new(cloned.end_recording(), None, Some(res_p2)))
            } else {
                Err(p2.err().unwrap())
            }
        }

    }
}

impl<P1, P2, I> Expect<I> for Or<P1, P2, I>
where P1: Parser<I>,
      P2: Parser<I>,
      I: Iterator + Clone,
      I::Item: PartialEq + Clone {
    fn expect(&self, cursor: &mut Cursor<I>) -> Result<Self, Error> {
        let res = Self::parse(cursor)?;
        Ok(res)
    }
}

impl<P1, P2, I> Display for Or<P1, P2, I>
where P1: Parser<I> + Debug,
      P2: Parser<I> + Debug,
      I: Iterator + Clone,
      I::Item: PartialEq + Clone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Or({:?}, {:?})", self.p1, self.p2)
    }
}