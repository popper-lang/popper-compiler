pub mod environement;
pub mod class;
pub mod instance;
pub mod resolver;
use std::rc::Rc;

use crate::ast::visitor::{ExprVisitor, StmtVisitor};
use crate::ast::expr::{Expr, LiteralType};
use crate::ast::stmt::Stmt;
use crate::lexer::{Token, TokenType};
use crate::value::{Value, Var, Type, Func};
use crate::value::function::Function;
use crate::value::callable::Callable;
use crate::errors::{error, DisplayError};
use crate::builtin_function::io;
use self::class::Class;
use self::environement::Environment;


#[derive(Debug, Clone)]
pub struct Interpreter {
    pub env: Environment<String, Var>,
    locals: Environment<Expr, i32>
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let mut inter = Interpreter { 
            env: Environment::new(None),
            locals: Environment::new(None)
        };
        inter.env.define("print".to_string(), Var {
            value: Value::Function(
                Func("print".to_string(),
                    Rc::new(io::Print))
                ),
            mutable: false,
            type_: Type::Func,
        });

        inter.env.define("println".to_string(), Var {
            value: Value::Function(
                Func("println".to_string(),
                    Rc::new(io::Println))
                ),
            mutable: false,
            type_: Type::Func,
        });

        inter


    }

    pub fn new_with_env(env: Environment<String, Var>) -> Interpreter {
        Interpreter { env , locals: Environment::new(None)}
    }

    fn resolve(&mut self, expr: Expr, depeth: i32) {
        self.locals.define(expr, depeth);
    }

    fn look_up_var(&mut self, name: String, expr: Expr) -> Option<Var> {
        let distance = self.locals.fetch(expr);
        if let Some(d) = distance {
            self.env.get_at(d.clone(), name)
        } else {
            self.env.fetch(name)
        }
    }


}

impl ExprVisitor for Interpreter {
    type Output = Value;

    fn visit_bin_op(&mut self, left: Expr, op: Token, right: Expr) -> Self::Output {
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

    fn visit_call(&mut self, name: Expr, args: Vec<Expr>) -> Self::Output {
        let resolved_name = name.accept(self);
        let mut arguments = Vec::new();
        for arg in args {
            arguments.push(arg.accept(self));
        };
        match resolved_name {
            Value::Function(f) => f.1.call(self, arguments),
            Value::Class(c) => c.call(self, arguments),
            _ => error!("expected function")
        }

    }

    fn visit_get(&mut self, name: Expr, attr: Expr) -> Self::Output {
        let name = name.accept(self);
        match name {
            Value::Instance(c) => {
                let mut interpreteur = Interpreter::new_with_env(c.class.methods);
                attr.accept(&mut interpreteur)
            },
            e => panic!("expected class")
        }
    }

    fn visit_grouping(&mut self, _group: Expr) -> Self::Output {
        todo!()
    }

    fn visit_index(&mut self, _name: Expr, _index: Expr) -> Self::Output {
        todo!()
    }

    fn visit_iop(&mut self, _name: Token, _op: Token, _value: Expr) -> Self::Output {
        todo!()
    }

    fn visit_list(&mut self, _elems: Vec<Expr>) -> Self::Output {
        todo!()
    }

    fn visit_literal(&mut self, literal: LiteralType) -> Self::Output {
        match literal {
            LiteralType::Number(n) => Value::Number(n as f64),
            LiteralType::Bool(b) => Value::Bool(b),
            LiteralType::String(s) => Value::String(s.clone())
        }
    }

    fn visit_range(&mut self, _start: Expr, _end: Expr) -> Self::Output {
        todo!()
    }

    fn visit_assign(&mut self, name: Token, value: Expr) -> Self::Output {
        let name_string = name.lexeme.to_string();

        let value_evaluated = value.clone().accept(self);
        let distance = self.locals.fetch(Expr::Assign { name: name, value: Box::new(value) });
        if let Some(d) = distance {
            self.env.define_at(d, name_string, Var {
                value: value_evaluated.clone(),
                type_: value_evaluated.get_type(),
                mutable: true,
            });
        } else {
            self.env.define(name_string.clone(), Var {
                value: value_evaluated.clone(),
                type_: value_evaluated.get_type(),
                mutable: true,
            });
        }
        Value::None

    }

    fn visit_to(&mut self, _name: Expr, _type_: Expr) -> Self::Output {
        todo!()
    }

    fn visit_unary_op(&mut self, _op: Token, _operand: Expr) -> Self::Output {
        todo!()
    }

    fn visit_ident(&mut self, ident: Token) -> Self::Output {
        let id = ident.lexeme.to_string();
         
        match self.look_up_var(id, Expr::Ident { ident: ident.clone() }) {
            Some(v) => v.value.clone(),
            None => error!("ident not found", ident.line, ident.pos)
        }
    }

    fn visit_type(&mut self, _type_: Token) -> Self::Output {
        todo!()
    }

}

impl StmtVisitor for Interpreter {
    type Output = Value;

    fn visit_let(&mut self, name: Token, value: Option<Expr>, mutable: bool, type_: Option<Expr>) -> Self::Output {
        let name = name.lexeme.to_string();
        if let Some(v) = value {
            let value = v.accept(&mut self.clone());

            self.env.define(name, Var { value: value.clone(), mutable: mutable, type_: match type_ {
                Some(e) => match e.accept(&mut self.clone()) {
                    Value::Type(t) => t,
                    e => panic!("expected type found {:?}", e.get_type())
                }
                _ => value.get_type()

            } });
            
        }
        Value::None
    }

    fn visit_block(&mut self, stmts: Vec<Stmt>) -> Self::Output {
        let previous = self.env.clone();
        let env = Environment::new(
            Some(self.env.clone())
        );
        let mut res = Value::None;
        for mut stmt in stmts {
            self.env = env.clone();
            res = stmt.accept(self);
        }
        self.env = previous;
        res
    }

    fn visit_expression(&mut self, expr: Expr) -> Self::Output {
        expr.accept(self)
    }

    fn visit_if(&mut self, cond: Expr, then: Stmt) -> Self::Output {
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

    fn visit_if_else(&mut self, cond: Expr, then: Stmt, else_: Stmt) -> Self::Output {
        let cond = cond.accept(self);

        if let Value::Bool(e) = cond {
            if e {
                then.accept(self)
            } else {
                else_.accept(self)
            }
        } else {
            error!("expected bool")
        }
    }

    fn visit_for(&mut self, _name: Token, _iter: Expr, _body: Stmt) -> Self::Output {
        todo!()
    }

    fn visit_while(&mut self, _cond: Expr, _body: Stmt) -> Self::Output {
        todo!()
    }

    fn visit_match(&mut self, _cond: Expr, _cases: Vec<(Expr, Box<Stmt>)>) -> Self::Output {
        todo!()
    }

    fn visit_function(&mut self, name: Token, args: Vec<String>, body: Stmt) -> Self::Output {
        let s = Stmt::Function { name: name.clone(), args: args.clone(), body: Box::new(body.clone()) };
        let n = name.lexeme.to_string();
        let func = Func(n.clone(), Rc::new(Function { declaration: s }));
        

        self.env.define(n, Var {
            value: Value::Function(func),
            mutable: false,
            type_: Type::Function

        });
        Value::None
    }

    fn visit_class(&mut self, name: String, methods: Vec<Stmt>) -> Self::Output {
        let mut functions = Vec::new();
        let mut env = self.env.clone();
        let mut interpreter = Interpreter::new_with_env(env);

        for method in methods {
            match method {
                Stmt::Function { name: e, args: a, body: b }  => {
                    interpreter.visit_function(e.clone(), a, *b);
                    if let Some(v) = interpreter.env.fetch(e.lexeme.to_string())  {
                        functions.push(v.value.clone())
                    } else {
                        unreachable!()
                    }
                   
                }
                _ => unreachable!()
            }
        }

        /*for function in functions {
            println!("function: {:#?}", function);
            env.define(match function {
                Value::Function(ref f) => f.0.to_string(),
                _ => unreachable!()
            }, Var {
                value: function,
                mutable: false,
                type_: Type::Func,
            }
                );
        }*/
        self.env.define(name.clone(), Var { 
            value: Value::Class(
                Class {
                    name: name.clone(),
                    methods: interpreter.env
                }
            ),
            mutable: false,
            type_: Type::Class(name.clone())
        });
        Value::None
    }
}


