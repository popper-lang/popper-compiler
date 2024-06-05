use crate::expr::Expression;
use crate::span::Span;

#[cfg_attr(feature = "extra-trait", derive(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
/// binary operation: `<expr> <op> <expr>`
pub struct BinOp {
    pub span: Span,
    pub op: BinOpKind,
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

impl BinOp {
    pub fn new(span: Span, op: BinOpKind, lhs: Expression, rhs: Expression) -> Self {
        Self {
            span,
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    pub fn span(&self) -> Span {
        self.span
    }
}

#[cfg_attr(feature = "extra-trait", derive(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
pub enum BinOpKind {
    /// +
    Add,
    /// -
    Sub,
    /// *
    Mul,
    /// /
    Div,
    /// %
    Mod,
    /// ^
    Pow,
    /// ==
    Eq,
    /// !=
    Neq,
    /// <
    Lt,
    /// <=
    Lte,
    /// >
    Gt,
    /// >=
    Gte,
    /// &&
    And,
    /// ||
    Or,
}

impl BinOpKind {
    pub fn is_comparison(&self) -> bool {
        matches!(self, BinOpKind::Eq | BinOpKind::Neq | BinOpKind::Lt | BinOpKind::Lte | BinOpKind::Gt | BinOpKind::Gte)
    }

    pub fn is_arithmetic(&self) -> bool {
        matches!(self, BinOpKind::Add | BinOpKind::Sub | BinOpKind::Mul | BinOpKind::Div | BinOpKind::Mod | BinOpKind::Pow)
    }

    pub fn is_logical(&self) -> bool {
        matches!(self, BinOpKind::And | BinOpKind::Or)
    }


}
