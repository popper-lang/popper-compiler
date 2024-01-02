use crate::*;

#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "extra-trait", derive(Debug, PartialEq, Eq))]
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
}
