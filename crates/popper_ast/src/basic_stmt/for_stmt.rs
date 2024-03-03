use crate::*;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "extra-trait", derive(Debug, PartialEq))]
#[derive(Clone)]
pub struct ForStmt {
    pub it: Ident,
    pub expr: Expression,
    pub body: Block,
    span: Span,
}

impl ForStmt {
    pub fn new(it: Ident, expr: Expression, body: Block, span: Span) -> Self {
        Self {
            it,
            expr,
            body,
            span,
        }
    }

    pub fn span(&self) -> Span {
        self.span
    }
}
