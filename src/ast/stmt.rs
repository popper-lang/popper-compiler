use super::expr::Expr;
use crate::lexer::Token;
use std::ops::Range;
use crate::value::Type;

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
        args: ArgsTyped,
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
    },
    Return {
        value: Option<Expr>
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Stmt {
    pub(crate) stmt_type: Box<StmtType>,
    pub(crate) extract: Range<usize>,
    pub(crate) body: String,
    pub(crate) file: String
}

impl Stmt {
    pub fn new(stmt_type: StmtType, extract: Range<usize>, body: String, file: String) -> Stmt {
        Stmt {
            stmt_type: Box::new(stmt_type),
            extract,
            body,
            file
            
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArgsTyped(pub Vec<ArgTyped>);

#[derive(Debug, Clone, PartialEq)]
pub struct ArgTyped {
    pub name: String,
    pub type_: Type
}

impl ArgsTyped {
    fn get(self, name: String) -> Option<Type> {
        for i in self.0 {
            if i.name == name {
                return Some(i.type_);
            }
        }
        None
    }
}

impl Iterator for ArgsTyped {
    type Item = ArgTyped;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.clone().into_iter().next()
    }
}
