use crate::expr::Expression;
use crate::span::Span;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOpKind {
    Neg,
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

