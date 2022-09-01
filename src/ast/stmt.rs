use crate::lexer::Token;
use super::expr::Expr;

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Let {
        name: Token, value: Option<Expr>, mutable: bool, type_: Option<Expr>
    },
    Block {
        body: Vec<Stmt>
    },
    Function {
        name: Token, args: Vec<String>, body: Box<Stmt>
    },
    If {
        cond: Expr, then: Box<Stmt>
    },
    IfElse {
        cond: Expr, then: Box<Stmt>, else_: Box<Stmt>
    },
    For {
        name: Token, iter: Expr, body: Box<Stmt>
    },
    While {
        cond: Expr, body: Box<Stmt>
    },
    Match {
        cond: Expr, cases: Vec<(Expr, Box<Stmt>)>
    },
    Class {
        name: String , methods: Vec<Stmt>
    },
    Expression {
        expr: Expr
    },


}