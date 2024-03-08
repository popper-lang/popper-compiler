use crate::{Call, Expression, Ident, Reference, Span};




#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "extra-trait", derive(Debug, PartialEq))]
#[derive(Clone)]
pub struct Assign {
    pub name: Expression,
    pub value: Expression,
    pub span: Span,
}


impl Assign {
    pub fn new(name: Expression, value: Expression, span: Span) -> Self {
        Self {
            name,
            value,
            span,
        }
    }
}
