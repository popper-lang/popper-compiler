use crate::lexer::Token;
use crate::value::Value;

#[derive(Debug, Clone)]
pub enum Expr { 
    BinOp { left: Box<Expr>, op: Token, right: Box<Expr> },
    Call { name: Token, args: Vec<Expr> },
    CallStruct { name: Vec<(String, Expr)> },
    EnumCall { name: Token, field: Token },
    GetAttr { name: Box<Expr>, attr: Token },
    GetFunc { name: Box<Expr>, func: Token, args: Vec<Expr> },
    GetModAttr { mod_name: Box<Expr>, attr_name: Token },
    GetModFunc { mod_name: Box<Expr>, func_name: Token, args: Vec<Expr> },
    Grouping { group: Box<Expr> },
    Index { name: Token, index: Box<Expr> },
    IOp { name: Token, op: Token, value: Box<Expr> },
    List { elems: Vec<Expr> },
    Literal { literal: Value },
    Range { start: Box<Expr>, end: Box<Expr> },
    SetVar { name: Token, value: Box<Expr> },
    StructDef { name: Token, fields: Vec<(Token, Expr)> },
    To { value: Box<Expr>, type_: Box<Expr> },
    UnaryOp { op: Token, operand: Box<Expr> },
    Ident { ident: Token },
    Type { type_: Token },
}

