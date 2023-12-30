use crate::{Span, Arguments, Type};

// a function signature :
// fn printf (a: int, b: int) -> int;

#[cfg_attr(feature = "extra-trait", derive(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
pub struct FunctionSign {
    span: Span,
    pub name: String,
    pub arguments: Arguments,
    pub return_type: Type
}

impl FunctionSign {
    pub fn new(span: Span, name: String, arguments: Arguments, return_type: Type) -> Self {
        Self { span, name, arguments, return_type }
    }

    pub fn span(&self) -> Span {
        self.span
    }
}