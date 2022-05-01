
#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    IfThen {
        cond: Box<Expr>,
        then: Box<Expr>    
    },
    IfThenElse {
        cond: Box<Expr>,
        then: Box<Expr>,
        else_: Box<Expr>,
    },
    While {
        cond: Box<Expr>,
        body: Box<Expr>
    },
    Assign {
        name: String,
        value: Box<Expr>
    },
    Number {
        value: f64
    },
    Identifier {
        name: String
    },
    BinOp {
        op: Op,
        left: Box<Expr>,
        right: Box<Expr>
    },
    String {
        value: String
    },
    For {
        name: String,
        iter: Box<Expr>,
        body: Box<Expr>
    },
    Bool {
        value: bool
    }
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
    Invalid
}