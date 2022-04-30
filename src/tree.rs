
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
    Let {
        name: String,
        value: Box<Expr>
    },
    Number {
        value: String
    },
    Identifier {
        name: String
    },
    BinOp {
        op: String,
        left: Box<Expr>,
        right: Box<Expr>
    },
}