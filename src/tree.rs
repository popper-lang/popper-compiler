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
        mutable: bool,
        type_: Option<crate::executer::value::Type>,
    },
    Literal {
        value: Literal,
    },
    BinOp {
        op: Op,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    IOp {
        op: IOp,
        value: Box<Expr>,
        name: String
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
        ident: String,
    },
    List {
        elems: Vec<Expr>,
    },
    Index {
        name: Box<Expr>,
        index: Box<Expr>,
    },
    Range {
        start: Box<Expr>,
        end: Box<Expr>,
    },
    StructDef {
        name: String,
        fields: Vec<Expr>,
    },
    CallStruct {
        name: String,
        args: Vec<(Expr, Expr)>,
    },
    GetAttr {
        name: String,
        attr: String,
    },
    Impl {
        name_struct: String,
        name_method: String,
        args: Vec<Expr>,
        body: Box<Expr>,

    },
    GetFunc {
        name: String,
        func: String,
        args: Vec<Expr>,
    },
    SetVar {
        name: String,
        value: Box<Expr>,
    },
    Match {
        value: Box<Expr>,
        cases: Vec<(Expr, Expr)>,
    },
    Enum {
        name: String,
        fields: Vec<String>
    },
    EnumCall {
        name: String,
        field: String
    },
    To {
        value: Box<Expr>,
        to: crate::executer::value::Type,

    },
    Empty
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
    Or
}

#[derive(Debug, PartialEq, Clone)]
pub enum IOp {
    IAdd,
    ISub,
    IMul,
    IDiv
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Number(f64),
    String(String),
    Bool(bool),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    StringType, 
    IntType,
    BoolType,
    Any
}
