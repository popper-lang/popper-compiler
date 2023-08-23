use crate::{Expression, Ident, Type};
use crate::Span;


#[cfg_attr(feature = "extra-trait", derive(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
/// A let statement to create variable. Syntax: `let <ident>: <type?> = <expr>;`
pub struct LetStmt {
    pub name: Ident,
    pub value: Expression,
    pub r#type: Option<Type>,
    pub mutable: bool,
    pub span: Span,

}

impl LetStmt {
    pub fn new(span: Span, name: Ident, r#type: Option<Type>, mutable: bool, value: Expression) -> Self {
        Self {
            name,
            value,
            r#type,
            mutable,
            span,
        }
    }
}