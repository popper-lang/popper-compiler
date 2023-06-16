use crate::Span;
use crate::Expression;


/// A parenthesized group of expressions.
/// Syntax: `(expr)`
#[cfg_attr(feature = "extra-trait", derive(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
pub struct ParenGroup {
    span: Span,
    pub expr: Box<Expression>,
}

impl ParenGroup {
    pub fn new(span: Span, expr: Expression) -> Self {
        Self { span, expr: Box::new(expr) }
    }

    pub fn span(&self) -> Span {
        self.span
    }
}