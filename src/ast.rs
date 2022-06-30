use crate::errors::Error;

use crate::value::Value;
use crate::vm::{Evaluateur, Vm};

#[allow(unused_attributes)]
#[macro_use]
use crate::import_expr;

import_expr!(
    assign -> Assign,
    binop -> BinOp,
    block -> Block,
    call -> Call,
    callstruct -> CallStruct,
    enum_ -> Enum,
    enumcall -> EnumCall,
    fundef -> FunDef,
    getattr -> GetAttr,
    getfunc -> GetFunc,
    ident -> Ident,
    ifthen -> IfThen,
    ifthenelse -> IfThenElse,
    impl_ -> Impl,
    index -> Index,
    iop -> IOp,
    list -> List,
    literal -> Literal,
    loop_for -> For,
    loop_while -> While,
    match_ -> Match,
    range -> Range,
    setvar -> SetVar,
    structdef -> StructDef,
    to -> To,
    typeof_ -> Typeof,
    type_ -> TypeExpr,
    module -> Module,
    getmodattr -> GetModAttr,
    getmodfunc -> GetModFunc
);

#[derive(Debug, PartialEq, Clone)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Mod,
    Eq,
    Neq,
    Lt,
    Gt,
    Le,
    Ge,
    And,
    Or,
}

#[derive(Debug, PartialEq, Clone)]
pub enum IOpType {
    IAdd,
    ISub,
    IMul,
    IDiv,
}
