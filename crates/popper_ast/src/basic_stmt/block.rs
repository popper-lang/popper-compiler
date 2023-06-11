use crate::{Span, Statement};

#[derive(Debug, Clone)]
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