use crate::span::Span;
use crate::expr::Expression;

#[cfg_attr(feature = "extra-trait", derive(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
pub struct BinOp {
    span: Span,
    pub op: BinOpKind,
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

impl BinOp {
    pub fn new(span: Span, op: BinOpKind, lhs: Expression, rhs: Expression) -> Self {
        Self { span, op, lhs: Box::new(lhs), rhs: Box::new(rhs) }
    }

    pub fn span(&self) -> Span {
        self.span
    }
}

#[cfg_attr(feature = "extra-trait", derive(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
pub enum BinOpKind {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Eq,
    Neq,
    Lt,
    Lte,
    Gt,
    Gte,
    And,
    Or
}