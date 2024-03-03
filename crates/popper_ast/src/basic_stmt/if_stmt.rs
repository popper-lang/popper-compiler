use crate::Expression;
use crate::Span;
use crate::Statement;

#[cfg_attr(feature = "extra-trait", derive(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
/// A if statement. Syntax: `if <expr> <stmt>`
pub struct If {
    pub condition: Expression,
    pub body: Box<Statement>,
    pub span: Span,
}

impl If {
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

#[cfg_attr(feature = "extra-trait", derive(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
/// A if-else statement. Syntax: `if <expr> <stmt> else <stmt>`
pub struct IfElse {
    pub condition: Expression,
    pub body: Box<Statement>,
    pub else_body: Box<Statement>,
    pub span: Span,
}

impl IfElse {
    pub fn new(span: Span, condition: Expression, body: Statement, else_body: Statement) -> Self {
        Self {
            span,
            condition,
            body: Box::new(body),
            else_body: Box::new(else_body),
        }
    }

    pub fn span(&self) -> Span {
        self.span
    }
}
