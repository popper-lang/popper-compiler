use crate::lexer::Token;
use std::ops::Range;

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum ExprType {
    BinOp { left: Expr, op: Token, right: Expr },
    Call { name: Expr, args: Vec<Expr> },
    Get { name: Expr, attr: Expr },
    Grouping { group: Expr },
    Index { name: Expr, index: Expr },
    IOp { name: Token, op: Token, value: Expr },
    List { elems: Vec<Expr> },
    Literal { literal: LiteralType },
    Range { start: Expr, end: Expr },
    Assign { name: Token, value: Expr },
    To { value: Expr, type_: Expr },
    UnaryOp { op: Token, operand: Expr },
    Ident { ident: Token },
    Type { type_: Token },
    CmpOp { left: Expr, op: Token, right: Expr },
    NsGet { name: Expr, attr: Expr },
    InitStruct { name: Expr, fields: Vec<(Expr, Expr)> },
    Eof,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Expr {
    pub(crate) expr_type: Box<ExprType>,
    pub(crate) extract: Range<usize>,
    pub(crate) body: String,
}

impl Expr {
    pub fn new(expr_type: Box<ExprType>, extract: Range<usize>, body: String) -> Expr {
        Expr {
            expr_type,
            extract,
            body,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LiteralType {
    Number(i32),
    Bool(bool),
    String(String),
}
