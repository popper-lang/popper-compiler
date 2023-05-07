pub mod c_repr;

use crate::ast::expr::{Expr, ExprType, LiteralType};
use crate::ast::stmt::{Stmt, StmtType};
use crate::interpreter::Interpreter;
use crate::lexer::TokenType;
use crate::value::Object;
use crate::value::Type;

#[derive(Clone)]
pub struct Compiler {
    pub code: String,
    pub interpreter: Interpreter,
    pub ast: Vec<Stmt>,
    pub stack: Vec<String>,
    current: usize,
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            code: String::new(),
            interpreter: Interpreter::new(),
            ast: Vec::new(),
            stack: Vec::new(),
            current: 0
        }
    }



    pub fn compile_stmt(&mut self, stmt: &Stmt) -> String {
        match &*stmt.stmt_type {
            StmtType::If { cond, then} => {
                let cond_evaluated = self.eval_expr(cond);
                if cond_evaluated.type_ != Type::Int {
                    panic!("Expected int, got {}", cond_evaluated.type_);
                }
                let cond = self.compile_expr(cond);
                let then = self.compile_stmt(then);
                c_repr::c_if(cond, then)
            },
            StmtType::IfElse { cond, then, else_ } => {

                let cond = self.compile_expr(cond);
                let then = self.compile_stmt(then);
                let else_ = self.compile_stmt(else_);
                c_repr::c_if_else(cond, then, else_)
            },
            StmtType::Block { body } => {
                let mut new_body = String::new();
                for stmt in body {
                    new_body.push_str(&self.compile_stmt(stmt).as_str());
                }
                c_repr::c_block(new_body)
            },
            StmtType::While { cond, body } => {
                let cond_evaluated = self.eval_expr(cond);
                if cond_evaluated.type_ != Type::Int {
                    panic!("Expected int, got {}", cond_evaluated.type_);
                }
                let cond = self.compile_expr(cond);
                let body = self.compile_stmt(body);
                c_repr::c_while(cond, body)
            },
            StmtType::Expression { expr } => {
                let expr = self.compile_expr(expr);
                c_repr::c_expr(expr)
            },
            StmtType::Struct { name, fields } => {
                let new_field = fields.into_iter().map(move |e| {
                    let obj = self.eval_expr(&e.1);
                    (e.clone().0, c_repr::c_type(obj.type_))
                }).collect::<Vec<(String, String)>>();
                c_repr::c_struct(name.clone(), c_repr::c_typed_args(new_field, ";"))
            },
            StmtType::Class { .. } => panic!(" Class is not supported in C language"),
            StmtType::Let { name, value , type_, mutable} => {
                let type_ = if let Some(t) = type_ {
                    if let ExprType::Type { type_ } = &*t.expr_type {
                        match type_ {
                            Type::Int => "int",
                            Type::String => "char*",
                            _ => panic!("Type not supported")
                        }.to_string()
                    } else {
                        panic!("Type not supported")
                    }
                } else {
                    let value = self.eval_expr(&value.clone().unwrap());
                    c_repr::c_type(value.type_)
                };
                let value = self.compile_expr(&value.clone().unwrap());
                let name = name.lexeme.clone();
                if *mutable {
                    c_repr::c_init_var(type_, name, value)
                } else {
                    c_repr::c_const(type_, name, value)
                }
            },
            _ => todo!()

        }
    }

    fn compile_expr(&mut self, expr: &Expr) -> String {
        match *expr.expr_type.clone() {
            ExprType::Literal {literal} => self.compile_literal(literal),
            ExprType::Type { type_: _ } => panic!("Error: type alone dont work"),
            ExprType::Call { ref name, args } => {
                let args = args.iter().map(|e| self.compile_expr(e)).collect::<Vec<String>>();
                let name = self.compile_expr(name);
                c_repr::c_call(name, args)
            },
            ExprType::Ident { ident } => {
                ident.lexeme
            },
            ExprType::Asm { asm } => {
                c_repr::c_asm(asm)
            },
            ExprType::BinOp { ref left, op, ref right } => {
                let left = self.compile_expr(left);
                let right = self.compile_expr(right);
                c_repr::c_binop(left, op.lexeme, right)
            },
            ExprType::List { .. } => {
                panic!("List not supported in C language")
            },
            ExprType::Get { ref name , ref attr} => {
                let name = self.compile_expr(name);
                let field = self.compile_expr(attr);
                c_repr::c_get(name, field)
            },
            ExprType::Assign { name, ref value } => {
                let name = name.lexeme;
                let value = self.compile_expr(value);
                c_repr::c_assign(name, value)
            },
            ExprType::Grouping { ref group } => {
                let expr = self.compile_expr(group);
                c_repr::c_group(expr)
            },
            ExprType::CmpOp { ref left, op, ref right } => {
                let left = self.compile_expr(left);
                let right = self.compile_expr(right);
                c_repr::c_cmpop(left, op.lexeme, right)
            },
            ExprType::NsGet { .. } => {
                panic!("Namespace not supported in C language")
            },

            ExprType::Index { ref index , ref name} => {
                let name = self.compile_expr(name);
                let index = self.compile_expr(index);
                c_repr::c_index(name, index)
            },
            ExprType::IOp { op, name, ref value } => {
                let name = name.lexeme;
                let value = self.compile_expr(value);
                c_repr::c_iop(name, op.lexeme, value)
            },
            ExprType::Range { .. } => panic!("Range not supported in C language"),
            ExprType::To { type_, value } => {
                let value = self.compile_expr(&value.clone());
                let type_ = match type_ {
                    Type::Int => "int",
                    Type::String => "char*",
                    Type::Bool => "int",
                    _ => panic!("unexpected boolean")
                };
                c_repr::c_to(type_.to_string(), value)
            },
            ExprType::Lambda { .. } => panic!("Lambda not supporeted in C"),
            ExprType::UnaryOp { operand: _, op: _ } => {todo!()}
            ExprType::InitStruct { .. } => {todo!()}
            ExprType::Eof => {todo!()}
        }
    }

    fn compile_literal(&mut self, literal: LiteralType) -> String {
        match literal {
            LiteralType::String(s) => s,
            LiteralType::Number(n) => n.to_string(),
            LiteralType::Bool(b) => if b { 1.to_string() } else { 0.to_string() }
        }
    }

    fn eval_stmt(&mut self, stmt: &Stmt) -> Object {
        stmt.clone().accept(&mut self.interpreter)
    }

    fn eval_expr(&mut self, expr: &Expr) -> Object {
        expr.clone().accept(&mut self.interpreter)
    }


    fn peek(&mut self) -> &Stmt {
        &self.ast[self.clone().current]
    }

    fn next(&mut self) -> &Stmt {
        self.current += 1;
        self.peek()
    }

    fn previous(&mut self) -> &Stmt {
        self.current -= 1;
        self.peek()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.ast.len()
    }


}