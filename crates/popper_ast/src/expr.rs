use crate::Constant;
use crate::BinOp;
use crate::UnaryOp;
use crate::Span;
use crate::ParenGroup;

#[derive(Debug, Clone)]
pub enum Expression {
    Constant(Constant),
    BinOp(BinOp),
    UnaryOp(UnaryOp),
    Group(ParenGroup)
}

impl Expression {
    pub fn span(&self) -> Span {
        match self {
            Expression::Constant(c) => c.span(),
            Expression::BinOp(b) => b.span(),
            Expression::UnaryOp(u) => u.span(),
            Expression::Group(g) => g.span(),
        }
    }
}

