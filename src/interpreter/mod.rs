pub mod environement;
pub mod resolver;
use std::rc::Rc;
use crate::value::{class, BinaryOperation};
use crate::ast::visitor::{ExprVisitor, StmtVisitor};
use crate::ast::expr::{Expr, LiteralType};
use crate::ast::stmt::Stmt;
use crate::lexer::Token;
use crate::value::{Object, Var, Type};
use crate::value::function::Function;
use crate::errors::{error};
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
            value: Rc::new(
                io::Print
            ),
            mutable: false,
            type_: Type::Func,
        });

        inter.env.define("println".to_string(), Var {
            value: Rc::new(
                io::Println
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
        println!("{:?}", distance);
        if let Some(d) = distance {
            self.env.get_at(d.clone(), name)
        } else {
            println!("{:?}", self.env);
            self.env.fetch(name)
        }
    }


}

impl ExprVisitor for Interpreter {
    type Output = Rc<dyn Object>;

    fn visit_bin_op(&mut self, left: Expr, op: Token, right: Expr) -> Self::Output {
        let left = left.accept(self);
        let right = right.accept(self);
        let res = match op.lexeme.as_str() {
            "+" => left.add(&right),
            "-" => left.subtract(&right),
            "*" => left.multiply(&right),
            "/" => left.divide(&right),
            _ => unreachable!()
        };
        if let Some(r) = res {
            r
        } else {
            error!("error when binary operation")
        }

    }

    fn visit_call(&mut self, name: Expr, args: Vec<Expr>) -> Self::Output {
        let resolved_name = name.accept(self);
        let mut arguments = Vec::new();
        for arg in args {
            arguments.push(arg.accept(self));
        };

        resolved_name.call(self, arguments)
        

    }

    fn visit_get(&mut self, name: Expr, attr: String) -> Self::Output {
        let name = name.accept(self);
        if let Some(e) = name.getter() {
            e.fetch(attr)
        } else {
            error!("can't get")
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
            LiteralType::Number(n) => Rc::new(n),
            LiteralType::Bool(b) => Rc::new(b),
            LiteralType::String(s) => Rc::new(s)
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
        Rc::new(())

    }

    fn visit_to(&mut self, _name: Expr, _type_: Expr) -> Self::Output {
        todo!()
    }

    fn visit_unary_op(&mut self, _op: Token, _operand: Expr) -> Self::Output {
        todo!()
    }

    fn visit_ident(&mut self, ident: Token) -> Self::Output {
        let id = ident.lexeme.to_string();
        println!("{}", id);
        match self.look_up_var(id, Expr::Ident { ident: ident }) {
            Some(v) => v.value,
            None => error!("ident not found")
        }
    }

    fn visit_type(&mut self, _type_: Token) -> Self::Output {
        todo!()
    }

}

impl StmtVisitor for Interpreter {
    type Output = Rc<dyn Object>;

    fn visit_let(&mut self, name: Token, value: Option<Expr>, mutable: bool, type_: Option<Expr>) -> Self::Output {
        let name = name.lexeme.to_string();
        if let Some(v) = value {
            let value = v.accept(&mut self.clone());
            let ty = if let Some(e) = type_ {
                e.accept(self).get_type()
            } else {
                value.get_type()
            };
            self.env.define(name, Var { value: value, mutable: mutable, type_: ty});
            println!("{:?}", self.env);
        }
        Rc::new(())
    }

    fn visit_block(&mut self, stmts: Vec<Stmt>) -> Self::Output {
        let previous = self.env.clone();
        let env = Environment::new(
            Some(self.env.clone())
        );
        let mut res: Rc<dyn Object> = Rc::new(());
        for stmt in stmts {
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

        if cond.boolean(){
            then.accept(self)
        } else {
            Rc::new(())
        }

    }

    fn visit_if_else(&mut self, cond: Expr, then: Stmt, else_: Stmt) -> Self::Output {
        let cond = cond.accept(self);

        if cond.boolean() {
            then.accept(self)
        } else if ! cond.boolean() {
            else_.accept(self)
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
        

        self.env.define(n, Var {
            value: Rc::new(Function { declaration: s }),
            mutable: false,
            type_: Type::Function

        });
        Rc::new(())
    }

    fn visit_class(&mut self, name: String, methods: Vec<Stmt>) -> Self::Output {
        let mut functions = Vec::new();
        let env = self.env.clone();
        let mut interpreter = Interpreter::new_with_env(env);

        for method in methods {
            match method {
                Stmt::Function { name: e, args: a, body: b }  => {
                    interpreter.visit_function(e.clone(), a, *b);
                    if let Some(v) = interpreter.env.fetch(e.lexeme.to_string())  {
                        functions.push(v.value)
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
            value: Rc::new(
                Class {
                    name: name.clone(),
                    methods: interpreter.env
                }
            ),
            mutable: false,
            type_: Type::Class(name.clone())
        });
        Rc::new(())
    }
}



