#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    IfThen {
        cond: Box<Expr>,
        then: Box<Expr>,
    },
    IfThenElse {
        cond: Box<Expr>,
        then: Box<Expr>,
        else_: Box<Expr>,
    },
    While {
        cond: Box<Expr>,
        body: Box<Expr>,
    },
    Assign {
        name: String,
        value: Box<Expr>,
    },
    Literal {
        value: Literal,
    },
    BinOp {
        op: Op,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    For {
        name: Box<Expr>,
        iter: Box<Expr>,
        body: Box<Expr>,
    },
    FunDef {
        name: String,
        args: Vec<Expr>,
        body: Box<Expr>,
    },
    Call {
        name: String,
        args: Vec<Expr>,
    },
    Block {
        body: Vec<Expr>,
    },
    Ident {
        name: String,
    },
    Empty,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Neq,
    Lt,
    Gt,
    Le,
    Ge,
    And,
    Or,
    Invalid,
    Assign,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Number(f64),
    String(String),
    Bool(bool),
}
