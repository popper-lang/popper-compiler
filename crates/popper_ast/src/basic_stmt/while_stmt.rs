use crate::Expression;
use crate::Span;
use crate::Statement;

#[cfg_attr(feature = "extra-trait", derive(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
/// a while statement. Syntax: `while <expr> <stmt>`
pub struct While {
    span: Span,
    pub condition: Expression,
    pub body: Box<Statement>,
}

impl While {
    pub fn new(span: Span, condition: Expression, body: Statement) -> Self {
        Self {
            span,
            condition,
            body: Box::new(body),
        }
    }

    pub fn span(&self) -> Span {
        self.span
    }
}
