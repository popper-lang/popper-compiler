mod args;
pub use args::{Arguments, Argument};


use crate::Type;
use crate::Span;
use crate::Statement;

#[cfg_attr(feature = "extra-trait", derive(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
pub struct Function {
    pub name: String,
    pub arguments: Arguments,
    pub returntype: Type,
    pub body: Vec<Statement>,
    pub span: Span
}

impl Function {
    pub fn new(name: String, arguments: Arguments, returntype: Type, body: Vec<Statement>, span: Span) -> Self {
        Self {
            name,
            arguments,
            returntype,
            body,
            span
        }
    }
}