use crate::ast::expr::Expr;
use crate::ast::stmt::Stmt;
use crate::lexer::Token;
use crate::value::Value;

pub trait ExprVisitor {
    type Output;

    fn visit_bin_op(&mut self, left: &Expr, op: &Token, right: &Expr) -> Self::Output;
    fn visit_call(&mut self, name: &Token, args: &Vec<Expr>) -> Self::Output;
    fn visit_call_struct(&mut self, name: &Vec<(String, Expr)>) -> Self::Output;
    fn visit_enum_call(&mut self, name: &Token, field: &Token) -> Self::Output;
    fn visit_get_attr(&mut self, name: &Expr, attr: &Token) -> Self::Output;
    fn visit_get_func(&mut self, name: &Expr, func: &Token, args: &Vec<Expr>) -> Self::Output;
    fn visit_get_mod_attr(&mut self, mod_name: &Expr, attr_name: &Token) -> Self::Output;
    fn visit_get_mod_func(&mut self, mod_name: &Expr, func_name: &Token, args: &Vec<Expr>) -> Self::Output;
    fn visit_grouping(&mut self, group: &Expr) -> Self::Output;
    fn visit_index(&mut self, name: &Token, index: &Expr) -> Self::Output;
    fn visit_iop(&mut self, name: &Token, op: &Token, value: &Expr) -> Self::Output;
    fn visit_list(&mut self, elems: &Vec<Expr>) -> Self::Output;
    fn visit_literal(&mut self, literal: &Value) -> Self::Output;
    fn visit_range(&mut self, start: &Expr, end: &Expr) -> Self::Output;
    fn visit_set_var(&mut self, name: &Token, value: &Expr) -> Self::Output;
    fn visit_struct_def(&mut self, name: &Token, fields: &Vec<(Token, Expr)>) -> Self::Output;
    fn visit_to(&mut self, name: &Expr, type_: &Expr) -> Self::Output;
    fn visit_unary_op(&mut self, op: &Token, operand: &Expr) -> Self::Output;
    fn visit_ident(&mut self, ident: &Token) -> Self::Output;
    fn visit_type(&mut self, type_: &Token) -> Self::Output;
}

pub trait StmtVisitor {
    type Output;

    fn visit_assign(&mut self, name: &Token, value: &Box<Expr>, mutable: &bool, type_: &Option<Expr>) -> Self::Output;
    fn visit_block(&mut self, stmts: &Vec<Stmt>) -> Self::Output;
    fn visit_expression(&mut self, expr: &Expr) -> Self::Output;
    fn visit_if(&mut self, cond: &Expr, then: &Stmt) -> Self::Output;
    fn visit_if_else(&mut self, cond: &Expr, then: &Stmt, else_: &Stmt) -> Self::Output;
    fn visit_for(&mut self, name: &Token, iter: &Expr, body: &Stmt) -> Self::Output;
    fn visit_while(&mut self, cond: &Expr, body: &Stmt) -> Self::Output;
    fn visit_impl(&mut self, name_struct: &Token, name_method: &Token, args: &Vec<(Token, Expr)>, body: &Vec<Stmt>) -> Self::Output;
    fn visit_match(&mut self, cond: &Expr, cases: &Vec<(Expr, Box<Stmt>)>) -> Self::Output;
    fn visit_struct(&mut self, name: &Token, fields: &Vec<(Token, Expr)>) -> Self::Output;
    fn visit_function(&mut self, name: &Token, args: &Vec<(Token, Expr)>, body: &Stmt) -> Self::Output;
    fn visit_enum(&mut self, name: &Token, fields: &Vec<(Token, Expr)>) -> Self::Output;
    fn visit_module(&mut self, name: &Token, as_name: &Token) -> Self::Output;
}

impl Expr {
    pub fn accept<V: ExprVisitor>(&self, visitor: &mut V) -> V::Output {
        match self {
            Expr::BinOp { left, op, right } => visitor.visit_bin_op(left, op, right),
            Expr::Call { name, args } => visitor.visit_call(name, args),
            Expr::CallStruct { name } => visitor.visit_call_struct(name),
            Expr::EnumCall { name, field } => visitor.visit_enum_call(name, field),
            Expr::GetAttr { name, attr } => visitor.visit_get_attr(name, attr),
            Expr::GetFunc { name, func, args } => visitor.visit_get_func(name, func, args),
            Expr::GetModAttr { mod_name, attr_name } => visitor.visit_get_mod_attr(mod_name, attr_name),
            Expr::GetModFunc { mod_name, func_name, args } => visitor.visit_get_mod_func(mod_name, func_name, args),
            Expr::Grouping { group } => visitor.visit_grouping(group),
            Expr::Index { name, index } => visitor.visit_index(name, index),
            Expr::IOp { name, op, value } => visitor.visit_iop(name, op, value),
            Expr::List { elems } => visitor.visit_list(elems),
            Expr::Literal { literal } => visitor.visit_literal(literal),
            Expr::Range { start, end } => visitor.visit_range(start, end),
            Expr::SetVar { name, value } => visitor.visit_set_var(name, value),
            Expr::StructDef { name, fields } => visitor.visit_struct_def(name, fields),
            Expr::To { value, type_ } => visitor.visit_to(value, type_),
            Expr::UnaryOp { op, operand } => visitor.visit_unary_op(op, &operand),
            Expr::Ident { ident } => visitor.visit_ident(ident),
            Expr::Type { type_ } => visitor.visit_type(type_),
        }
    }
}

impl Stmt {
    pub fn accept<V: StmtVisitor>(&self, visitor: &mut V) -> V::Output {
        match self {
            Stmt::Assign { name, value , mutable, type_} => visitor.visit_assign(name, value, mutable, type_),
            Stmt::Block { body } => visitor.visit_block(body),
            Stmt::Expression { expr } => visitor.visit_expression(expr),
            Stmt::If { cond, then } => visitor.visit_if(cond, then),
            Stmt::IfElse { cond, then, else_ } => visitor.visit_if_else(cond, then, else_),
            Stmt::For { name, iter, body } => visitor.visit_for(name, iter, body),
            Stmt::While { cond, body } => visitor.visit_while(cond, body),
            Stmt::Impl { name_struct, name_method, args, body} => visitor.visit_impl(name_struct, name_method, args, body),
            Stmt::Match { cond, cases } => visitor.visit_match(cond, cases),
            Stmt::Struct { name, fields } => visitor.visit_struct(name, fields),
            Stmt::Function { name, args, body } => visitor.visit_function(name, args, body),
            Stmt::Enum { name, fields } => visitor.visit_enum(name, fields),
            Stmt::Module { name, as_name } => visitor.visit_module(name, as_name)
        }
    }
}