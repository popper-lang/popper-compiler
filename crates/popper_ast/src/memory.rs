use crate::{Expression, Span};


#[cfg_attr(feature = "extra-trait", derive(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
pub struct Reference {
    pub expr: Box<Expression>,
    pub span: Span
}

impl Reference {
    pub fn new(expr: Expression, span: Span) -> Self {
        Self {
            expr: Box::new(expr),
            span
        }
    }
}


#[cfg_attr(feature = "extra-trait", derive(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
pub struct Deref {
    pub expr: Box<Expression>,
    pub span: Span
}

impl Deref {
    pub fn new(expr: Expression, span: Span) -> Self {
        Self {
            expr: Box::new(expr),
            span
        }
    }
}
