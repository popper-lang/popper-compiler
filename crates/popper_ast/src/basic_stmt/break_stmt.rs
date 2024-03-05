use crate::{Expression, Span};



#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "extra-trait", derive(Debug, PartialEq))]
#[derive(Clone)]
/// Represents a break statement
pub struct BreakStmt {
    pub span: Span
}

impl BreakStmt {
    pub fn new(span: Span) -> Self {
        Self { span }
    }
}
