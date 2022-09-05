use crate::lexer::Token;

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum Expr { 
    BinOp { left: Box<Expr>, op: Token, right: Box<Expr> },
    Call { name: Box<Expr>, args: Vec<Expr> },
    Get { name: Box<Expr>, attr: String },
    Grouping { group: Box<Expr> },
    Index { name: Box<Expr>, index: Box<Expr> },
    IOp { name: Token, op: Token, value: Box<Expr> },
    List { elems: Vec<Expr> },
    Literal { literal: LiteralType },
    Range { start: Box<Expr>, end: Box<Expr> },
    Assign { name: Token, value: Box<Expr> },
    To { value: Box<Expr>, type_: Box<Expr> },
    UnaryOp { op: Token, operand: Box<Expr> },
    Ident { ident: Token },
    Type { type_: Token },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LiteralType {
    Number(i32),
    Bool(bool),
    String(String)
}
