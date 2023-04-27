use crate::ast::expr::{Expr, ExprType};
use crate::ast::stmt::{Stmt, StmtType};
use crate::lexer::Token;

use super::expr::LiteralType;

pub trait ExprVisitor {
    type Output;

    fn visit_bin_op(&mut self, left: Expr, op: Token, right: Expr) -> Self::Output;
    fn visit_call(&mut self, name: Expr, args: Vec<Expr>) -> Self::Output;
    fn visit_get(&mut self, name: Expr, attr: Expr) -> Self::Output;
    fn visit_grouping(&mut self, group: Expr) -> Self::Output;
    fn visit_index(&mut self, name: Expr, index: Expr) -> Self::Output;
    fn visit_iop(&mut self, name: Token, op: Token, value: Expr) -> Self::Output;
    fn visit_list(&mut self, elems: Vec<Expr>) -> Self::Output;
    fn visit_literal(&mut self, literal: LiteralType) -> Self::Output;
    fn visit_range(&mut self, start: Expr, end: Expr) -> Self::Output;
    fn visit_assign(&mut self, name: Token, value: Expr) -> Self::Output;
    fn visit_to(&mut self, name: Expr, type_: Expr) -> Self::Output;
    fn visit_unary_op(&mut self, op: Token, operand: Expr) -> Self::Output;
    fn visit_ident(&mut self, ident: Token) -> Self::Output;
    fn visit_type(&mut self, type_: Token) -> Self::Output;
    fn visit_cmp_op(&mut self, left: Expr, op: Token, right: Expr) -> Self::Output;
    fn visit_ns_get(&mut self, name: Expr, attr: Expr) -> Self::Output;
    fn visit_init_struct(&mut self, name: Expr, fields: Vec<(Expr, Expr)>) -> Self::Output;
    fn visit_asm(&mut self, asm: String) -> Self::Output;
    fn visit_eof(&mut self) -> Self::Output;
}

pub trait StmtVisitor {
    type Output;

    fn visit_let(
        &mut self,
        name: Token,
        value: Option<Expr>,
        mutable: bool,
        type_: Option<Expr>,
    ) -> Self::Output;
    fn visit_block(&mut self, stmts: Vec<Stmt>) -> Self::Output;
    fn visit_expression(&mut self, expr: Expr) -> Self::Output;
    fn visit_if(&mut self, cond: Expr, then: Stmt) -> Self::Output;
    fn visit_if_else(&mut self, cond: Expr, then: Stmt, else_: Stmt) -> Self::Output;
    fn visit_for(&mut self, name: Token, iter: Expr, body: Stmt) -> Self::Output;
    fn visit_while(&mut self, cond: Expr, body: Stmt) -> Self::Output;
    fn visit_match(&mut self, cond: Expr, cases: Vec<(Expr, Stmt)>) -> Self::Output;
    fn visit_function(&mut self, name: Token, args: Vec<String>, body: Stmt) -> Self::Output;
    fn visit_class(&mut self, name: String, methods: Vec<Stmt>) -> Self::Output;
    fn visit_use(&mut self, path: String, as_: String) -> Self::Output;
    fn visit_import(&mut self, name: String, imports: Vec<String>) -> Self::Output;
    fn visit_impl(&mut self, struct_name: String, methods: Vec<Stmt>) -> Self::Output;
    fn visit_struct(&mut self, name: String, fields: Vec<(String, Expr)>) -> Self::Output;
}

impl Expr {
    pub fn accept<V: ExprVisitor> (self, visitor: &mut V) -> V::Output {
        match *self.expr_type {
            ExprType::BinOp { left, op, right } => visitor.visit_bin_op(left, op, right),
            ExprType::Call { name, args } => visitor.visit_call(name, args),
            ExprType::Get { name, attr } => visitor.visit_get(name, attr),
            ExprType::Grouping { group } => visitor.visit_grouping(group),
            ExprType::Index { name, index } => visitor.visit_index(name, index),
            ExprType::IOp { name, op, value } => visitor.visit_iop(name, op, value),
            ExprType::List { elems } => visitor.visit_list(elems),
            ExprType::Literal { literal } => visitor.visit_literal(literal),
            ExprType::Range { start, end } => visitor.visit_range(start, end),
            ExprType::Assign { name, value } => visitor.visit_assign(name, value),
            ExprType::To { value, type_ } => visitor.visit_to(value, type_),
            ExprType::UnaryOp { op, operand } => visitor.visit_unary_op(op, operand),
            ExprType::Ident { ident } => visitor.visit_ident(ident),
            ExprType::Type { type_ } => visitor.visit_type(type_),
            ExprType::CmpOp { left, op, right } => visitor.visit_cmp_op(left, op, right),
            ExprType::NsGet { name, attr } => visitor.visit_ns_get(name, attr),
            ExprType::InitStruct { name, fields } => visitor.visit_init_struct(name, fields),
            ExprType::Asm { asm } => visitor.visit_asm(asm),
            ExprType::Eof => visitor.visit_eof(),
        }
    }
}

impl Stmt {
    pub fn accept<V: StmtVisitor>(self, visitor: &mut V) -> V::Output {
        match *self.stmt_type {
            StmtType::Let {
                name,
                value,
                mutable,
                type_,
            } => visitor.visit_let(name, value, mutable, type_),
            StmtType::Block { body } => visitor.visit_block(body),
            StmtType::Expression { expr } => visitor.visit_expression(expr),
            StmtType::If { cond, then } => visitor.visit_if(cond, then),
            StmtType::IfElse { cond, then, else_ } => visitor.visit_if_else(cond, then, else_),
            StmtType::For { name, iter, body } => visitor.visit_for(name, iter, body),
            StmtType::While { cond, body } => visitor.visit_while(cond, body),
            StmtType::Match { cond, cases } => visitor.visit_match(cond, cases),
            StmtType::Function { name, args, body } => visitor.visit_function(name, args, body),
            StmtType::Class { name, methods } => visitor.visit_class(name.to_string(), methods),
            StmtType::Use { path, as_ } => visitor.visit_use(path, as_),
            StmtType::Import { name, imports } => visitor.visit_import(name, imports),
            StmtType::Impl { struct_name, methods } => visitor.visit_impl(struct_name, methods),
            StmtType::Struct { name, fields } => visitor.visit_struct(name, fields),

        }
    }
}
