use super::expr::Expr;
use crate::lexer::Token;
use std::ops::Range;

#[derive(Debug, Clone, PartialEq)]
pub enum StmtType {
    Let {
        name: Token,
        value: Option<Expr>,
        mutable: bool,
        type_: Option<Expr>,
    },
    Block {
        body: Vec<Stmt>,
    },
    Function {
        name: Token,
        args: Vec<String>,
        body: Stmt,
    },
    If {
        cond: Expr,
        then: Stmt,
    },
    IfElse {
        cond: Expr,
        then: Stmt,
        else_: Stmt,
    },
    For {
        name: Token,
        iter: Expr,
        body: Stmt,
    },
    While {
        cond: Expr,
        body: Stmt,
    },
    Match {
        cond: Expr,
        cases: Vec<(Expr, Stmt)>,
    },
    Class {
        name: String,
        methods: Vec<Stmt>,
    },
    Expression {
        expr: Expr,
    },
    Use {
        path: String,
        as_: String,
    },
    Import {
        name: String,
        imports: Vec<String>,
    },
    Impl {
        struct_name: String,
        methods: Vec<Stmt>
    },
    Struct {
        name: String,
        fields: Vec<(String, Expr)>
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Stmt {
    pub(crate) stmt_type: Box<StmtType>,
    pub(crate) extract: Range<usize>,
    pub(crate) body: String,
}

impl Stmt {
    pub fn new(stmt_type: StmtType, extract: Range<usize>, body: String) -> Stmt {
        Stmt {
            stmt_type: Box::new(stmt_type),
            extract,
            body,
        }
    }
}
