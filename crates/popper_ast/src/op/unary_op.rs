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
        Self { op, expr: Box::new(expr), span }
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

impl ToString for UnaryOpKind {
    fn to_string(&self) -> String {
        match self {
            UnaryOpKind::Neg => "-".to_string(),
            UnaryOpKind::Not => "!".to_string(),
        }
    }
}

impl UnaryOpKind {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "-" => Some(UnaryOpKind::Neg),
            "!" => Some(UnaryOpKind::Not),
            _ => None,
        }
    }
}

