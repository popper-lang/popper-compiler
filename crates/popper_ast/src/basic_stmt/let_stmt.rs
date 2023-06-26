use crate::{Expression, Ident, Type};
use crate::Span;


#[cfg_attr(feature = "extra-trait", derive(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
pub struct LetStmt {
    pub name: Ident,
    pub value: Expression,
    pub r#type: Type,
    pub mutable: bool,
    pub span: Span,

}

impl LetStmt {
    pub fn new(span: Span, name: Ident, r#type: Type, mutable: bool, value: Expression) -> Self {
        Self {
            name,
            value,
            r#type,
            mutable,
            span,
        }
    }
}