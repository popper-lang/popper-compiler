use crate::{Call, Constant, Return};
use crate::BinOp;
use crate::UnaryOp;
use crate::Span;
use crate::ParenGroup;
use crate::StructInstance;
use crate::StructFieldAccess;

#[cfg_attr(feature = "extra-trait", derive(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
pub enum Expression {
    Constant(Constant),
    BinOp(BinOp),
    UnaryOp(UnaryOp),
    Group(ParenGroup),
    Call(Call),
    StructInstance(StructInstance),
    StructFieldAccess(StructFieldAccess),
}

impl Expression {
    pub fn span(&self) -> Span {
        match self {
            Expression::Constant(c) => c.span(),
            Expression::BinOp(b) => b.span(),
            Expression::UnaryOp(u) => u.span(),
            Expression::Group(g) => g.span(),
            Expression::Call(c) => c.span,
            Expression::StructInstance(s) => s.span,
            Expression::StructFieldAccess(s) => s.span,
        }
    }
}

