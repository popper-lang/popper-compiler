use std::fmt::Display;
use std::str::FromStr;

use crate::expr::Expression;
use crate::span::Span;

#[cfg_attr(feature = "extra-trait", derive(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
/// Unary operation: `<op> <expr>`
pub struct UnaryOp {
    pub op: UnaryOpKind,
    pub expr: Box<Expression>,
    span: Span,
}

impl UnaryOp {
    pub fn new(span: Span, op: UnaryOpKind, expr: Expression) -> Self {
        Self {
            op,
            expr: Box::new(expr),
            span,
        }
    }

    pub fn span(&self) -> Span {
        self.span
    }
}

#[cfg_attr(feature = "extra-trait", derive(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy)]
pub enum UnaryOpKind {
    /// -
    Neg,
    /// !
    Not,
}

impl Display for UnaryOpKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOpKind::Neg => write!(f, "-"),
            UnaryOpKind::Not => write!(f, "!"),
        }
    }
}

impl FromStr for UnaryOpKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-" => Ok(UnaryOpKind::Neg),
            "!" => Ok(UnaryOpKind::Not),
            _ => Err(()),
        }
    }
}
