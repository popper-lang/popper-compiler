use crate::Expression;
use crate::Span;

#[cfg_attr(feature = "extra-trait", derive(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
pub struct Call {
    pub name: String,
    pub arguments: Vec<Expression>,
    pub span: Span
}

impl Call {
    pub fn new(name: String, arguments: Vec<Expression>, span: Span) -> Self {
        Self {
            name,
            arguments,
            span
        }
    }
}

