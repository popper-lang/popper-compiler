
use crate::errors::Error;
use crate::expr::block::Block;
use crate::expr::fundef::FunDef;
use crate::expr::iop::IOp;
use crate::expr::op::BinOp;
use crate::expr::range::Range;
use crate::expr::setvar::SetVar;
use crate::expr::structdef::StructDef;
use crate::expr::to::To;
use crate::expr::litteral::Literal;
use crate::expr::getfunc::GetFunc;
use crate::expr::getattr::GetAttr;
use crate::expr::index::Index;
use crate::expr::call::Call;
use crate::expr::callstruct::CallStruct;
use crate::expr::ifthen::IfThen;
use crate::expr::ifthenelse::IfThenElse;
use crate::expr::loop_while::While;
use crate::expr::loop_for::For;
use crate::expr::impl_::Impl;
use crate::expr::list::List;
use crate::expr::enumcall::EnumCall;
use crate::expr::assign::Assign;
use crate::expr::match_::Match;
use crate::expr::ident::Ident;
use crate::expr::enum_::Enum;
use crate::expr::typeof_::Typeof;
use crate::value::Value;
use crate::vm::{Evaluateur, Vm};
#[derive(Clone)]
pub enum Expr {
    Block(Block),
    FunDef(FunDef),
    IOp(IOp),
    Op(BinOp),
    Range(Range),
    SetVar(SetVar),
    StructDef(StructDef),
    To(To),
    Litteral(Literal),
    GetFunc(GetFunc),
    GetAttr(GetAttr),
    Index(Index),
    Call(Call),
    CallStruct(CallStruct),
    IfThen(IfThen),
    IfThenElse(IfThenElse),
    While(While),
    For(For),
    Impl(Impl),
    List(List),
    EnumCall(EnumCall),
    Assign(Assign),
    Match(Match),
    Ident(Ident),
    Enum(Enum),
    Typeof(Typeof),


}

impl Evaluateur for Expr {
    fn eval(&self, vm: &mut Vm) -> Result<Value, Error> {
        match self {
            Expr::Block(block) => block.eval(vm),
            Expr::FunDef(fun_def) => fun_def.eval(vm),
            Expr::IOp(iop) => iop.eval(vm),
            Expr::Op(op) => op.eval(vm),
            Expr::Range(range) => range.eval(vm),
            Expr::SetVar(set_var) => set_var.eval(vm),
            Expr::StructDef(struct_def) => struct_def.eval(vm),
            Expr::To(to) => to.eval(vm),
            Expr::Litteral(litteral) => litteral.eval(vm),
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




