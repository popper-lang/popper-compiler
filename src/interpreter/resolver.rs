use crate::ast::expr::{Expr, ExprType, LiteralType};
use crate::ast::stmt::{Stmt};
use crate::ast::visitor::{ExprVisitor, StmtVisitor};
use crate::error;
use crate::errors::{Error, ErrorType};
use crate::lexer::Token;
use std::collections::HashMap;

use super::Interpreter;

pub struct Resolver {
    pub stack: Vec<HashMap<String, bool>>,
    interpreteur: Interpreter,
}

impl Resolver {
    pub fn new(interpreteur: Interpreter) -> Self {
        let mut resolver = Resolver {
            stack: Vec::new(),
            interpreteur: interpreteur.clone(),
        };
        for (k, _) in interpreteur.env.into_iter() {
            resolver.define(k)
        }
        resolver
    }
    pub fn resolve(&mut self, stmts: Vec<Stmt>) {
        self.begin_scope();
        for (k, _) in self.interpreteur.env.clone().into_iter() {
            self.define(k)
        }
        self.resolve_statements(stmts);
        self.end_scope();
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

    pub fn begin_scope(&mut self) {
        if self.stack.is_empty() {
            self.stack.push(HashMap::new());
        } else {
            self.stack.push(self.stack.last().cloned().unwrap());
        }
    }

    pub fn end_scope(&mut self) {
        self.stack.pop();
    }

    fn declare(&mut self, name: String) {
        if self.stack.is_empty() {
            return;
        }
        self.stack.last_mut().unwrap().insert(name, false);
    }

    fn define(&mut self, name: String) {
        if self.stack.is_empty() {
            return;
        }
        self.stack.last_mut().unwrap().insert(name, true);
    }

    fn resolve_local(&mut self, expr: ExprType, name: String) {
        for (i, scope) in self.stack.clone().into_iter().enumerate() {
            if scope.contains_key(&name) {
                self.interpreteur.resolve(expr.clone(), i as i32);
            }
        }
    }
}

impl ExprVisitor for Resolver {
    type Output = ();

    fn visit_bin_op(&mut self, left: Expr, _op: Token, right: Expr) -> Self::Output {
        self.resolve_expression(left);
        self.resolve_expression(right);
    }

    fn visit_call(&mut self, name: Expr, args: Vec<Expr>) -> Self::Output {
        self.resolve_expression(name);
        for arg in args {
            self.resolve_expression(arg);
        }
    }

    fn visit_get(&mut self, name: Expr, _attr: Expr) -> Self::Output {
        self.resolve_expression(name);
    }

    fn visit_grouping(&mut self, group: Expr) -> Self::Output {
        self.resolve_expression(group);
    }

    fn visit_index(&mut self, name: Expr, index: Expr) -> Self::Output {
        self.resolve_expression(name);
        self.resolve_expression(index);
    }

    fn visit_iop(&mut self, name: Token, _op: Token, value: Expr) -> Self::Output {
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
        self.resolve_local(
            ExprType::Assign {
                name: name.clone(),
                value,
            },
            name.lexeme,
        )
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
                println!(
                    "{:?}, {:?}, {}",
                    self.stack.last().unwrap().get(&ident.lexeme),
                    self.stack.last().unwrap(),
                    &ident.lexeme
                );
                error!(
                    ErrorType::NameError,
                    "Can't read local variable in its own initializer.",
                    0..0,
                    "".to_string()
                )
            }
        }
    }

    fn visit_type(&mut self, _type: Token) -> Self::Output {}

    fn visit_cmp_op(&mut self, left: Expr, _op: Token, right: Expr) -> Self::Output {
        self.resolve_expression(left);
        self.resolve_expression(right);
    }

    fn visit_eof(&mut self) -> Self::Output {}
    // Nothing because its eof

    fn visit_ns_get(&mut self, _name: Expr, _attr: Expr) -> Self::Output {
        todo!()
    }

    fn visit_init_struct(&mut self, _name: Expr, _fields: Vec<(Expr, Expr)>) -> Self::Output {
        todo!()
    }

    fn visit_asm(&mut self, _asm: String) -> Self::Output {}

    fn visit_lambda(&mut self, args: Vec<String>, body: Expr) -> Self::Output {
        todo!()
    }
}

impl StmtVisitor for Resolver {
    type Output = ();

    fn visit_let(
        &mut self,
        name: Token,
        value: Option<Expr>,
        _mutable: bool,
        _type_: Option<Expr>,
    ) -> Self::Output {
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

    fn visit_match(&mut self, cond: Expr, cases: Vec<(Expr, Stmt)>) -> Self::Output {
        self.resolve_expression(cond);
        for case in cases {
            self.resolve_expression(case.0);
            self.resolve_statement(case.1);
        }
    }

    fn visit_function(&mut self, name: Token, _args: Vec<String>, _body: Stmt) -> Self::Output {
        self.define(name.lexeme);
    }

    fn visit_class(&mut self, _name: String, _methods: Vec<Stmt>) -> Self::Output {}
    fn visit_use(&mut self, _path: String, _as_: String) -> Self::Output {
        todo!()
    }
    fn visit_import(&mut self, _name: String, _imports: Vec<String>) -> Self::Output {
        todo!()
    }
    fn visit_impl(&mut self, _name: String, _methods: Vec<Stmt>) -> Self::Output {
        todo!()
    }
    fn visit_struct(&mut self, _name: String, _fields: Vec<(String, Expr)>) -> Self::Output {
        todo!()
    }
}
