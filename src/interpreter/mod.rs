mod environement;
use crate::ast::visitor::{ExprVisitor, StmtVisitor};
use crate::ast::expr::Expr;
use crate::ast::stmt::Stmt;
use crate::lexer::{Token, TokenType};
use crate::value::Value;
use crate::errors::{error, DisplayError};
use self::environement::Environment;


#[derive(Debug, Clone)]
pub struct Interpreter {
    pub env: Environment
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter { 
            env: Environment::new(),
            
        }
    }
}

impl ExprVisitor for Interpreter {
    type Output = Value;

    fn visit_bin_op(&mut self, left: &Expr, op: &Token, right: &Expr) -> Self::Output {
        let left = left.accept(self);
        let right = right.accept(self);
        let res = match op {
            Token { token_type: TokenType::ADD, .. } => left.add(&right),
            Token { token_type: TokenType::SUB, .. } => left.sub(&right),
            Token { token_type: TokenType::MUL, .. } => left.mul(&right),
            Token { token_type: TokenType::DIV, .. } => left.div(&right),
            Token { token_type: TokenType::MOD, .. } => left.modulo(&right),
            _ => error!("Unexpected operand type", op.line, op.pos)

        };
        match res {
            Ok(e) => e,
            Err(e) => error!(e.display_error(), op.line, op.pos)
        }

    }

    fn visit_call(&mut self, name: &Token, args: &Vec<Expr>) -> Self::Output {
        todo!()
    }

    fn visit_call_struct(&mut self, name: &Vec<(String, Expr)>) -> Self::Output {
        todo!()
    }

    fn visit_enum_call(&mut self, name: &Token, field: &Token) -> Self::Output {
        todo!()
    }

    fn visit_get_attr(&mut self, name: &Expr, attr: &Token) -> Self::Output {
        todo!()
    }

    fn visit_get_func(&mut self, name: &Expr, func: &Token, args: &Vec<Expr>) -> Self::Output {
        todo!()
    }

    fn visit_get_mod_attr(&mut self, mod_name: &Expr, attr_name: &Token) -> Self::Output {
        todo!()
    }

    fn visit_get_mod_func(&mut self, mod_name: &Expr, func_name: &Token, args: &Vec<Expr>) -> Self::Output {
        todo!()
    }

    fn visit_grouping(&mut self, group: &Expr) -> Self::Output {
        todo!()
    }

    fn visit_index(&mut self, name: &Token, index: &Expr) -> Self::Output {
        todo!()
    }

    fn visit_iop(&mut self, name: &Token, op: &Token, value: &Expr) -> Self::Output {
        todo!()
    }

    fn visit_list(&mut self, elems: &Vec<Expr>) -> Self::Output {
        todo!()
    }

    fn visit_literal(&mut self, literal: &Value) -> Self::Output {
        literal.clone()
    }

    fn visit_range(&mut self, start: &Expr, end: &Expr) -> Self::Output {
        todo!()
    }

    fn visit_set_var(&mut self, name: &Token, value: &Expr) -> Self::Output {
        todo!()
    }

    fn visit_struct_def(&mut self, name: &Token, fields: &Vec<(Token, Expr)>) -> Self::Output {
        todo!()
    }

    fn visit_to(&mut self, name: &Expr, type_: &Expr) -> Self::Output {
        todo!()
    }

    fn visit_unary_op(&mut self, op: &Token, operand: &Expr) -> Self::Output {
        todo!()
    }

    fn visit_ident(&mut self, ident: &Token) -> Self::Output {
        todo!()
    }

    fn visit_type(&mut self, type_: &Token) -> Self::Output {
        todo!()
    }

}

impl StmtVisitor for Interpreter {
    type Output = Value;

    fn visit_assign(&mut self, name: &Token, value: &Box<Expr>, mutable: &bool, type_: &Option<Expr>) -> Self::Output {
        todo!()
    }

    fn visit_block(&mut self, stmts: &Vec<Stmt>) -> Self::Output {
        let mut res = Value::None;
        for stmt in stmts {
            res = stmt.accept(self);
        }
        res
    }

    fn visit_expression(&mut self, expr: &Expr) -> Self::Output {
        expr.accept(self)
    }

    fn visit_if(&mut self, cond: &Expr, then: &Stmt) -> Self::Output {
        let cond = cond.accept(self);

        if let Value::Bool(e) = cond {
            if e {
                then.accept(self)
            } else {
                Value::None
            }
        } else {
            error!("expected bool")
        }

    }

    fn visit_if_else(&mut self, cond: &Expr, then: &Stmt, else_: &Stmt) -> Self::Output {
        todo!()
    }

    fn visit_for(&mut self, name: &Token, iter: &Expr, body: &Stmt) -> Self::Output {
        todo!()
    }

    fn visit_while(&mut self, cond: &Expr, body: &Stmt) -> Self::Output {
        todo!()
    }

    fn visit_impl(&mut self, name_struct: &Token, name_method: &Token, args: &Vec<(Token, Expr)>, body: &Vec<Stmt>) -> Self::Output {
        todo!()
    }

    fn visit_match(&mut self, cond: &Expr, cases: &Vec<(Expr, Box<Stmt>)>) -> Self::Output {
        todo!()
    }

    fn visit_struct(&mut self, name: &Token, fields: &Vec<(Token, Expr)>) -> Self::Output {
        todo!()
    }

    fn visit_function(&mut self, name: &Token, args: &Vec<(Token, Expr)>, body: &Stmt) -> Self::Output {
        todo!()
    }

    fn visit_enum(&mut self, name: &Token, fields: &Vec<(Token, Expr)>) -> Self::Output {
        todo!()
    }

    fn visit_module(&mut self, name: &Token, as_name: &Token) -> Self::Output {
        todo!()
    }
}


