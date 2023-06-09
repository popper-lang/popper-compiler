use crate::Span;
use crate::Expression;


/// A parenthesized group of expressions.
/// Syntax: `(expr)`
#[derive(Debug, Clone)]
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