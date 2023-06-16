use crate::{Span, Statement};

#[cfg_attr(feature = "extra-trait", derive(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
pub struct Block {
    span: Span,
    pub statements: Vec<Statement>,
}

impl Block {
    pub fn new(span: Span, statements: Vec<Statement>) -> Self {
        Self { span, statements }
    }

    pub fn span(&self) -> Span {
        self.span
    }
}