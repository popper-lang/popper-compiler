mod args;
mod call;
mod return_expr;
mod sign;
mod va_arg;

pub use args::{Argument, ArgumentValue, Arguments};
pub use call::Call;
pub use return_expr::Return;
pub use sign::FunctionSign;
pub use va_arg::VaArg;

use crate::Span;
use crate::Statement;
use crate::Type;

#[cfg_attr(feature = "extra-trait", derive(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
pub struct Function {
    pub name: String,
    pub arguments: Arguments,
    pub returntype: Type,
    pub body: Vec<Statement>,
    pub is_var_args: bool,
    pub span: Span,
}

impl Function {
    pub fn new(
        name: String,
        arguments: Arguments,
        returntype: Type,
        body: Vec<Statement>,
        is_var_args: bool,
        span: Span,
    ) -> Self {
        Self {
            name,
            arguments,
            returntype,
            body,
            is_var_args,
            span,
        }
    }

    pub fn span(&self) -> Span {
        self.span
    }
}
