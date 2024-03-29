use crate::BinOp;
use crate::Index;
use crate::ParenGroup;
use crate::Span;
use crate::StructFieldAccess;
use crate::StructInstance;
use crate::UnaryOp;
use crate::{Call, Constant, Deref, Reference, Return, VaArg};

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
    Index(Index),
    VaArg(VaArg),
    Reference(Reference),
    Deref(Deref),
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
            Expression::Index(i) => i.span,
            Expression::VaArg(v) => v.span(),
            Expression::Reference(r) => r.span,
            Expression::Deref(p) => p.span,
        }
    }

    pub fn is_assignable(&self) -> bool {
        match self {
            Expression::Reference(_) | Expression::Deref(_) | Expression::Index(_) => true,
            Expression::Group(g) => g.expr.is_assignable(),
            Expression::StructFieldAccess(s) => true,
            Expression::Constant(Constant::Ident(_)) => true,
            _ => false,
        }
    }
}
