
use crate::errors::Error;

use crate::value::Value;
use crate::vm::{Evaluateur, Vm};
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
    typeof_ -> Typeof
);


impl Evaluateur for Expr {
    fn eval(&self, vm: &mut Vm) -> Result<Value, Error> {
        match self {
            Expr::Block(block) => block.eval(vm),
            Expr::FunDef(fun_def) => fun_def.eval(vm),
            Expr::IOp(iop) => iop.eval(vm),
            Expr::BinOp(op) => op.eval(vm),
            Expr::Range(range) => range.eval(vm),
            Expr::SetVar(set_var) => set_var.eval(vm),
            Expr::StructDef(struct_def) => struct_def.eval(vm),
            Expr::To(to) => to.eval(vm),
            Expr::Literal(litteral) => litteral.eval(vm),
            Expr::GetFunc(get_func) => get_func.eval(vm),
            Expr::GetAttr(get_attr) => get_attr.eval(vm),
            Expr::Index(index) => index.eval(vm),
            Expr::Call(call) => call.eval(vm),
            Expr::CallStruct(call_struct) => call_struct.eval(vm),
            Expr::IfThen(if_then) => if_then.eval(vm),
            Expr::IfThenElse(if_then_else) => if_then_else.eval(vm),
            Expr::While(while_) => while_.eval(vm),
            Expr::For(for_) => for_.eval(vm),
            Expr::Impl(impl_) => impl_.eval(vm),
            Expr::List(list) => list.eval(vm),
            Expr::EnumCall(enum_call) => enum_call.eval(vm),
            Expr::Assign(assign) => assign.eval(vm),
            Expr::Match(match_) => match_.eval(vm),
            Expr::Ident(ident) => ident.eval(vm),
            Expr::Enum(enum_) => enum_.eval(vm),
            Expr::Typeof(typeof_) => typeof_.eval(vm),
        }
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
    Or
}

#[derive(Debug, PartialEq, Clone)]
pub enum IOpType {
    IAdd,
    ISub,
    IMul,
    IDiv
}




