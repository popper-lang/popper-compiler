
// compiler arduino is a compiler that generate hex file for arduino
// author: NightProg
// license: MIT


mod c_repr;
use c_repr::*;
use crate::ast::expr::{Expr, ExprType, LiteralType};

use crate::ast::stmt::{Stmt, StmtType};

impl Stmt {
    fn to_c_lang(&self) -> String {

        match &*self.stmt_type {
            StmtType::IfElse { ref cond, then, else_ } => {
                let mut cond = cond.to_c_lang();
                let mut then = then.to_c_lang();
                let mut else_ = else_.to_c_lang();
                c_if_else(cond, then, else_)
            }
            StmtType::If { ref cond, then } => {
                let mut cond = cond.to_c_lang();
                let mut then = then.to_c_lang();
                c_if(cond, then)
            }
            StmtType::Block { body } => {
                let mut string = "".to_string();
                for stmt in body {
                    string.push_str((stmt.to_c_lang().as_str().to_owned() + "\n").as_str());

                }
                c_block(string)
            }

            StmtType::Expression { expr } => {
                expr.to_c_lang()
            }

            StmtType::While { cond, body } => {
                let mut cond = cond.to_c_lang();
                let mut body = body.to_c_lang();
                c_while(cond, body)
            }

            StmtType::Let { name, value, type_, .. } => {
                let mut name = name.clone().lexeme;
                let mut type_ = match type_ {
                    Some(type_) => match *type_.expr_type {
                        ExprType::Literal { literal } => match literal {
                            LiteralType::Number(_) => "int".to_string(),
                            LiteralType::String(_) => "char*".to_string(),
                            LiteralType::Bool(_) => "bool".to_string()
                        },
                        _ => panic!("type not supported")
                    }
                    None => "int".to_string()
                };

                c_init_var(type_, name, value.unwrap().to_c_lang())
            }

            StmtType::For { .. } => {}
            StmtType::Match { .. } => {}
            StmtType::Class { .. } => {}
            StmtType::Use { .. } => {}
            StmtType::Import { .. } => {}
            StmtType::Impl { .. } => {}
            StmtType::Struct { .. } => {}
        }
    }
}

impl Expr {
    fn to_c_lang(&self) -> String {
        todo!()
    }
}
