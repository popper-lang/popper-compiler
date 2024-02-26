use crate::{Expression, Span};


#[cfg_attr(feature = "extra-trait", derive(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
pub struct Index {
    pub value: Box<Expression>,
    pub index: Box<Expression>,
    pub span: Span
}

impl Index {
    pub fn new(value: Expression, index: Expression, span: Span) -> Self {
        Self {
            value: Box::new(value),
            index: Box::new(index),
            span
        }
    }
}
