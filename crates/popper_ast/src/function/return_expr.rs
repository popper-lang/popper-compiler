use crate::Expression;
use crate::Span;

#[cfg_attr(feature = "extra-trait", derive(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
pub struct Return {
    pub expression: Option<Box<Expression>>,
    pub span: Span
}

impl Return {
    pub fn new(expression: Option<Expression>, span: Span) -> Self {
        Self {
            expression: expression.map(Box::new),
            span
        }
    }

    pub fn span(&self) -> Span {
        self.span
    }
}