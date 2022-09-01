use crate::ast::expr::{Expr, LiteralType};
use crate::ast::stmt::Stmt;
use crate::ast::visitor::{ExprVisitor, StmtVisitor};
use crate::error;
use crate::lexer::Token;
use std::collections::HashMap;

use super::Interpreter;

struct Resolver {
    pub stack: Vec<HashMap<String, bool>>,
    interpreteur: Interpreter,
    current_function: FunctionType
}

enum FunctionType {
    FUNCTION,
    METHOD,
    NONE
}


impl Resolver {
    pub fn new(interpreteur: Interpreter) -> Self {
        Resolver { stack: Vec::new(), interpreteur, current_function: FunctionType::NONE }
    }

    fn resolve_statements(&mut self, stmts: Vec<Stmt>) {
        for stmt in stmts {
            self.resolve_statement(stmt)
        }
    }
    
    fn resolve_statement(&mut self, stmt: Stmt) {
        stmt.accept(self)
    }

    fn resolve_expression(&mut self, expr: Expr) {
        expr.accept(self)
    }

    fn begin_scope(&mut self) {
        self.stack.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.stack.pop();
    }

    fn declare(&mut self, name: String) {
        if self.stack.is_empty() { return; }
        self.stack.last_mut().unwrap().insert(name, false);
    }

    fn define(&mut self, name: String) {
        if self.stack.is_empty() { return; }
        self.stack.last_mut().unwrap().insert(name, true);
    }

    fn declare_local(&mut self, name: String) {
        let mut reversed_stack = self.stack.clone();
        reversed_stack.reverse();
        for s in reversed_stack {

        }
    }

    fn resolve_local(&mut self, expr: Expr, name: String) {
        for (i, scope) in self.stack.clone().into_iter().enumerate() {
            if scope.contains_key(&name) {
                self.interpreteur.resolve(expr.clone(), i as i32);
            }
        }
    }
}

impl ExprVisitor for Resolver { 
    type Output = ();

    fn visit_bin_op(&mut self, left: Expr, op: Token, right: Expr) -> Self::Output {
        self.resolve_expression(left);
        self.resolve_expression(right);
    }

    fn visit_call(&mut self, name: Expr, args: Vec<Expr>) -> Self::Output {
        self.resolve_expression(name);
        for arg in args {
            self.resolve_expression(arg);
        }
    }

    fn visit_get(&mut self, name: Expr, attr: Expr) -> Self::Output {
        todo!()
    }

    fn visit_grouping(&mut self, group: Expr) -> Self::Output {
        self.resolve_expression(group);
    }

    fn visit_index(&mut self, name: Expr, index: Expr) -> Self::Output {
        self.resolve_expression(name);
        self.resolve_expression(index);
    }

    fn visit_iop(&mut self, name: Token, op: Token, value: Expr) -> Self::Output {
        self.visit_ident(name);
        self.resolve_expression(value);
    }

    fn visit_list(&mut self, elems: Vec<Expr>) -> Self::Output {
        for elem in elems {
            self.resolve_expression(elem);
        }
    }

    fn visit_literal(&mut self, _literal: LiteralType) -> Self::Output {}

    fn visit_range(&mut self, start: Expr, end: Expr) -> Self::Output {
        self.resolve_expression(start);
        self.resolve_expression(end);
    }

    fn visit_assign(&mut self, name: Token, value: Expr) -> Self::Output {
        self.resolve_expression(value.clone());
        self.resolve_local(Expr::Assign { name: name.clone(), value: Box::new(value) }, name.lexeme)
        
    }

    fn visit_to(&mut self, name: Expr, type_: Expr) -> Self::Output {
        self.resolve_expression(name);
        self.resolve_expression(type_);
    }

    fn visit_unary_op(&mut self, _op: Token, operand: Expr) -> Self::Output {
        self.resolve_expression(operand);
    }

    fn visit_ident(&mut self, ident: Token) -> Self::Output {
        if !self.stack.is_empty() {
            if let None = self.stack.last().unwrap().get(&ident.lexeme) {
                error!("Can't read local variable in its own initializer.", ident.line, ident.pos)
            }
        }
    }

    fn visit_type(&mut self, _type: Token) -> Self::Output {}
}

impl StmtVisitor for Resolver {
    type Output = ();

    fn visit_let(&mut self, name: Token, value: Option<Expr>, _mutable: bool, _type_: Option<Expr>) -> Self::Output {
        self.declare(name.clone().lexeme);
        if let Some(_) = value {
            self.define(name.lexeme);
        }

    }

    fn visit_block(&mut self, stmts: Vec<Stmt>) -> Self::Output {
        self.begin_scope();
        self.resolve_statements(stmts);
        self.end_scope();

    }

    fn visit_expression(&mut self, expr: Expr) -> Self::Output {
        self.resolve_expression(expr);
    }

    fn visit_if(&mut self, cond: Expr, then: Stmt) -> Self::Output {
        self.resolve_expression(cond);
        self.resolve_statement(then);
    }

    fn visit_if_else(&mut self, cond: Expr, then: Stmt, else_: Stmt) -> Self::Output {
        self.resolve_expression(cond);
        self.resolve_statement(then);    
        self.resolve_statement(else_);
    }

    fn visit_for(&mut self, name: Token, iter: Expr, body: Stmt) -> Self::Output {
        self.declare(name.lexeme.to_string());
        self.resolve_expression(iter);
        self.resolve_statement(body);
    }

    fn visit_while(&mut self, cond: Expr, body: Stmt) -> Self::Output {
        self.resolve_expression(cond);
        self.begin_scope();
        self.resolve_statement(body);
        self.end_scope();
    }

    fn visit_match(&mut self, cond: Expr, cases: Vec<(Expr, Box<Stmt>)>) -> Self::Output {
        self.resolve_expression(cond);
        for case in cases {
            self.resolve_expression(case.0);
            self.resolve_statement(*case.1);
        }
    }

    fn visit_function(&mut self, name: Token, args: Vec<String>, body: Stmt) -> Self::Output {
        self.define(name.lexeme);
    }

    fn visit_class(&mut self, name: String, methods: Vec<Stmt>) -> Self::Output {
        todo!()
    }
    
}
