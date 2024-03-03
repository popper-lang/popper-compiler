use crate::{Arguments, Span, Type};

// a function signature :
// func add (a: int, b: int): int;
#[cfg_attr(feature = "extra-trait", derive(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
pub struct FunctionSign {
    span: Span,
    pub name: String,
    pub arguments: Arguments,
    pub return_type: Type,
    pub is_var_args: bool,
}

impl FunctionSign {
    pub fn new(
        span: Span,
        name: String,
        arguments: Arguments,
        return_type: Type,
        is_var_args: bool,
    ) -> Self {
        Self {
            span,
            name,
            arguments,
            return_type,
            is_var_args,
        }
    }

    pub fn span(&self) -> Span {
        self.span
    }
}
