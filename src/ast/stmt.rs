use crate::lexer::Token;
use crate::value::Value;
use super::expr::Expr;

#[derive(Debug, Clone)]
pub enum Stmt {
    Assign {
        name: Token, value: Box<Expr>, mutable: bool, type_: Option<Expr>
    },
    Block {
        body: Vec<Stmt>
    },
    Enum {
        name: Token, fields: Vec<(Token, Expr)>
    },
    Function {
        name: Token, args: Vec<(Token, Expr)>, body: Box<Stmt>
    },
    If {
        cond: Expr, then: Box<Stmt>
    },
    IfElse {
        cond: Expr, then: Box<Stmt>, else_: Box<Stmt>
    },
    Impl {
        name_struct: Token, name_method: Token, args: Vec<(Token, Expr)>, body: Vec<Stmt>
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
    Module {
        name: Token, as_name: Token
    },
    Struct {
        name: Token,
        fields: Vec<(Token, Expr)>
    },
    Expression {
        expr: Expr
    },


}