use crate::Span;
use crate::Expression;
use crate::Statement;

#[derive(Debug, Clone)]
pub struct While {
    span: Span,
    pub condition: Expression,
    pub body: Box<Statement>,
}

impl While {
    pub fn new(span: Span, condition: Expression, body: Statement) -> Self {
        Self { span, condition, body: Box::new(body) }
    }

    pub fn span(&self) -> Span {
        self.span
    }
}

