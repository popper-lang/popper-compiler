pub mod environement;
pub mod resolver;

use std::collections::HashMap;
use self::class::Class;
use self::environement::Environment;
use crate::ast::expr::{Expr, ExprType, LiteralType};
use crate::ast::stmt::{Stmt, StmtType};
use crate::ast::visitor::{ExprVisitor, StmtVisitor};
use crate::builtin_function::{io, cmp, list_util};
use crate::errors::{error, Error, ErrorType};
use crate::lexer::{Token, TokenType};
use crate::value::{class, RustValue};
use crate::value::function::Function;
use crate::value::get::Getter;
use crate::value::{Object, Type, Var, Implementation};
use crate::value::litteral::{boolean, none, number, string};
use crate::value::list::list;
use crate::get_impl_if_exist;
use std::rc::Rc;
use std::path::{PathBuf, Path};
use std::fs;
use crate::value::Implementation::PartialEq;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::value::namespace::Namespace;
use crate::value::struct_type::StructField;
use crate::value::struct_type::StructType;
use crate::value::struct_type::struct_instance;






macro_rules! import_builtin {
    ($env:expr, $name:expr, $module:path) => {
        $env.define(
            $name.to_string(),
            Var {
                value: $module(),
                mutable: false,
                type_: Type::Func,
            },
        )
    };
}

macro_rules! import_rs_module {
    ($module:ident, $name:ident) => {
        use crate::$module::$name;
    };
}


// import library from directory , it is std library
fn import_library(interpreteur: &mut Interpreter, directory: String) {
   

    
}

#[derive(Debug, Clone)]
pub struct Interpreter {
    pub env: Environment<String, Var>,
    locals: Environment<ExprType, i32>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let mut inter = Interpreter {
            env: Environment::new(None),
            locals: Environment::new(None),
        };
        import_builtin!(inter.env, "print", io::Print::create);
        import_builtin!(inter.env, "println", io::Println::create);
        import_builtin!(inter.env, "is_equal", cmp::IsEqual::create);
        import_builtin!(inter.env, "is_not_equal", cmp::IsNotEqual::create);
        import_builtin!(inter.env, "map", list_util::Map::create);


        inter
    }

    pub fn new_with_env(env: Environment<String, Var>) -> Interpreter {
        Interpreter {
            env,
            locals: Environment::new(None),
        }
    }

    fn resolve(&mut self, expr: ExprType, depeth: i32) {
        self.locals.define(expr, depeth);
    }

    fn look_up_var(&mut self, name: String, expr: ExprType) -> Option<Var> {
        let distance = self.locals.fetch(expr);
        if let Some(d) = distance {
            self.env.get_at(d.clone(), name)
        } else {
            self.env.fetch(name)
        }
    }

    fn add_module(&mut self, namespace: Namespace, name: String) {
        self.env.define(name, Var {
            value: namespace.create(),
            type_: Type::Namespace,
            mutable: false,
        });
    }
}

impl ExprVisitor for Interpreter {
    type Output = Object;

    fn visit_bin_op(&mut self, left: Expr, op: Token, right: Expr) -> Self::Output {
        let left = left.accept(self);
        let right = right.accept(self);
        let res = match op.lexeme.as_str() {
            "+" =>  {
                let impl_left = get_impl_if_exist!(Add, left);
                let impl_right = get_impl_if_exist!(Add, right);
                if impl_left.is_some() && impl_right.is_some() {
                    impl_left.unwrap().add(right)
                } else {
                    error!(ErrorType::TypeError, "can't add", 0..1, op.lexeme);
                    unreachable!()
                }
            },
            "-" =>  {
                let impl_left = get_impl_if_exist!(Sub, left);
                let impl_right = get_impl_if_exist!(Sub, right);
                if impl_left.is_some() && impl_right.is_some() {
                    impl_left.unwrap().sub(right)
                } else {
                    error!(ErrorType::TypeError, "can't sub", 0..1, op.lexeme);
                    unreachable!()
                }
            },
            "*" => {
                let impl_left = get_impl_if_exist!(Mul, left);
                let impl_right = get_impl_if_exist!(Mul, right);
                if impl_left.is_some() && impl_right.is_some() {
                    impl_left.unwrap().mul(right)
                } else {
                    error!(ErrorType::TypeError, "can't mul", 0..1, op.lexeme);
                    unreachable!()
                }
            },
            "/" => {
                let impl_left = get_impl_if_exist!(Div, left);
                let impl_right = get_impl_if_exist!(Div, right);
                if impl_left.is_some() && impl_right.is_some() {
                    impl_left.unwrap().div(right)
                } else {
                    error!(ErrorType::TypeError, "can't div", 0..1, op.lexeme);
                    unreachable!()
                }
            },
            _ => unreachable!(),
        };
        res
    }

    fn visit_call(&mut self, name: Expr, args: Vec<Expr>) -> Self::Output {
        let resolved_name = name.clone().accept(self);
        let mut arguments = Vec::new();
        for arg in args {
            arguments.push(arg.accept(self));
        }

        let impl_call = get_impl_if_exist!(Call, resolved_name);
        if let Some(func) = impl_call {
            func.call(self, arguments)
        } else {
            error!(ErrorType::TypeError, "can't call", name.clone().extract, name.body);
            unreachable!()
        }
    }

    fn visit_get(&mut self, name_: Expr, attr: Expr) -> Self::Output {

        let name = name_.clone().accept(self);
        let impl_get = get_impl_if_exist!(Get, name);
        if let Some(mut e) = impl_get {

            e.fetch(self, attr).unwrap()
        } else {
            error!(
                ErrorType::AttributeError,
                "can't get", name_.extract, name_.body
            );
            unreachable!()
        }
    }

    fn visit_grouping(&mut self, group: Expr) -> Self::Output {
        return group.accept(self);
    }

    fn visit_index(&mut self, _name: Expr, _index: Expr) -> Self::Output {
        todo!()
    }

    fn visit_iop(&mut self, _name: Token, _op: Token, _value: Expr) -> Self::Output {
        todo!()
    }

    fn visit_list(&mut self, elems: Vec<Expr>) -> Self::Output {
        let mut list_object = Vec::new();
        for elem in elems {
            list_object.push(elem.accept(self));
        }

        list(list_object)

    }

    fn visit_literal(&mut self, literal: LiteralType) -> Self::Output {
        match literal {
            LiteralType::Number(n) => number(n),
            LiteralType::Bool(b) => boolean(b),
            LiteralType::String(s) => string(s.as_str()),
        }
    }

    fn visit_range(&mut self, _start: Expr, _end: Expr) -> Self::Output {
        todo!()
    }

    fn visit_assign(&mut self, name: Token, value: Expr) -> Self::Output {
        let name_string = name.lexeme.to_string();

        let value_evaluated = value.clone().accept(self);
        let distance = self.locals.fetch(ExprType::Assign { name, value });
        if let Some(d) = distance {
            self.env.define_at(
                d,
                name_string,
                Var {
                    value: value_evaluated.clone(),
                    type_: value_evaluated.clone().type_,
                    mutable: true,
                },
            );
        } else {
            self.env.modify(
                name_string.clone(),
                Var {
                    value: value_evaluated.clone(),
                    type_: value_evaluated.clone().type_,
                    mutable: true,
                },
            );
        }
        value_evaluated
    }

    fn visit_to(&mut self, _name: Expr, _type_: Expr) -> Self::Output {
        todo!()
    }

    fn visit_unary_op(&mut self, _op: Token, _operand: Expr) -> Self::Output {
        todo!()
    }

    fn visit_ident(&mut self, ident: Token) -> Self::Output {
        let id = ident.lexeme.to_string();



        match self.env.fetch(id.clone()) {
            Some(v) => v.value,
            None => {
                error!(
                    ErrorType::NameError,
                    "ident not found",
                    0..0,
                    "".to_string()
                );
                unreachable!()
            }
        }
    }

    fn visit_type(&mut self, _type_: Token) -> Self::Output {
        match _type_.lexeme.as_str() {
            "int" => Object {
                type_: Type::Int,
                value: RustValue::Type(Type::Int),
                implementations: vec![]
            },
            "bool" => Object {
                type_: Type::Bool,
                value: RustValue::Type(Type::Bool),
                implementations: vec![]
            },
            "str" => Object {
                type_: Type::String,
                value: RustValue::Type(Type::String),
                implementations: vec![]
            },
            "list" => Object {
                type_: Type::List,
                value: RustValue::Type(Type::List),
                implementations: vec![]
            },
            _ => {
                error!(
                    ErrorType::TypeError,
                    "type not found",
                    0..0,
                    "".to_string()
                );
                unreachable!()
            }
        }
    }

    fn visit_cmp_op(&mut self, left: Expr, op: Token, right: Expr) -> Self::Output {
        let left = left.accept(self);
        let right = right.accept(self);
        match op.lexeme.as_str() {
            "<" => {
                let impl_left = get_impl_if_exist!(PartialOrd, left);
                let impl_right = get_impl_if_exist!(PartialOrd, right);
                if impl_left.is_some() && impl_right.is_some() {
                    boolean(impl_left.unwrap().lt(right))
                } else {
                    //error!(ErrorType::TypeError, "can't lt", op.extract, op.body);
                    unreachable!()
                }
            },
            ">" => {
                let impl_left = get_impl_if_exist!(PartialOrd, left);
                let impl_right = get_impl_if_exist!(PartialOrd, right);
                if impl_left.is_some() && impl_right.is_some() {
                    boolean(impl_left.unwrap().gt(right))
                } else {

                    unreachable!()
                }
            }
            "==" => {
                let impl_left = get_impl_if_exist!(PartialEq, left);
                let impl_right = get_impl_if_exist!(PartialEq, right);
                if impl_left.is_some() && impl_right.is_some() {
                    boolean(impl_left.unwrap().eq(right))
                } else {
                    //error!(ErrorType::TypeError, "can't eq", op.extract, op.body);
                    unreachable!()
                }
            }
            "<=" => {
                let impl_left = get_impl_if_exist!(PartialOrd, left);
                let impl_right = get_impl_if_exist!(PartialOrd, right);
                if impl_left.is_some() && impl_right.is_some() {
                    boolean(impl_left.unwrap().le(right))
                } else {
                    //error!(ErrorType::TypeError, "can't lte", op.extract, op.body);
                    unreachable!()
                }
            }
            ">=" => {
                let impl_left = get_impl_if_exist!(PartialOrd, left);
                let impl_right = get_impl_if_exist!(PartialOrd, right);
                if impl_left.is_some() && impl_right.is_some() {
                    boolean(impl_left.unwrap().ge(right))
                } else {
                    //error!(ErrorType::TypeError, "can't gte", op.extract, op.body);
                    unreachable!()
                }
            }
            _ => unreachable!(),
        }
    }

    fn visit_ns_get(&mut self, name: Expr, attr: Expr) -> Self::Output {
        let name = name.clone().accept(self);
        let impl_get = get_impl_if_exist!(NsGet, name);
        if let Some(mut e) = impl_get {

            e.fetch(self, attr).unwrap()
        } else {
            error!(
                ErrorType::AttributeError,
                "can't get", 0..0, "".to_string()
            );
            unreachable!()
        }

    }

    fn visit_init_struct(&mut self, name: Expr, fields: Vec<(Expr, Expr)>) -> Self::Output {
        let name = name.accept(self);
        let fields = fields
            .into_iter()
            .map(|(name, value)| (match *name.expr_type {
                ExprType::Ident { ref ident } => ident.lexeme.clone(),
                _ => unreachable!()
            }, value.accept(self)))
            .collect::<HashMap<_, _>>();



        return struct_instance(match name {
            Object { type_: Type::Struct(_), value: RustValue::Struct(s), .. } => s,
            _ => unreachable!()
        }, fields);



    }

    fn visit_asm(&mut self, asm: String) -> Self::Output {
        todo!()
    }

    fn visit_eof(&mut self) -> Self::Output {
        none()
    }
}

impl StmtVisitor for Interpreter {
    type Output = Object;

    fn visit_let(
        &mut self,
        name: Token,
        value: Option<Expr>,
        mutable: bool,
        type_: Option<Expr>,
    ) -> Self::Output {
        let name = name.lexeme.to_string();
        if let Some(v) = value {
            let value = v.accept(&mut self.clone());
            let ty = if let Some(e) = type_ {
                e.accept(self).type_
            } else {
                value.clone().type_
            };
            self.env.define(
                name,
                Var {
                    value: value.clone(),
                    mutable: mutable,
                    type_: ty,
                },
            );
        }
        none()
    }

    fn visit_block(&mut self, stmts: Vec<Stmt>) -> Self::Output {
        let previous = self.env.clone();
        let env = Environment::new(Some(self.env.clone()));
        let mut res = none();
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

        if let RustValue::Bool(true) = cond.value {
            then.accept(self)
        } else {
            none()
        }
    }

    fn visit_if_else(&mut self, cond_: Expr, then: Stmt, else_: Stmt) -> Self::Output {
        let cond = cond_.clone().accept(self);
        if let RustValue::Bool(e) = cond.value {
            if e {
                then.accept(self)
            } else {
                else_.accept(self)
            }
        } else {
            error!(
                ErrorType::TypeError,
                "expected bool", cond_.extract, cond_.body
            );
            unreachable!()
        }
    }

    fn visit_for(&mut self, name: Token, iter: Expr, body: Stmt) -> Self::Output {
        let it = iter.accept(self);
        if let RustValue::List(v) = it.value {
            for i in v {
                self.env.define(
                    name.lexeme.clone(),
                    Var {
                        value: i.clone(),
                        mutable: false,
                        type_: i.type_,
                    },
                );
                body.clone().accept(self);
            }
            return none();
        } else {
            Error::new(
                ErrorType::TypeError,
                format!("{} isnt a iterable", name.lexeme).as_str(),
                0..0,
                "".to_string(),
            )
            .panic();
            unimplemented!()
        }
    }

    fn visit_while(&mut self, cond: Expr, body: Stmt) -> Self::Output {

        while let RustValue::Bool(true) = cond.clone().accept(self).value {
            body.clone().accept(self);
        }
        return none();
    }

    fn visit_match(&mut self, _cond: Expr, _cases: Vec<(Expr, Stmt)>) -> Self::Output {
        todo!()
    }

    fn visit_function(&mut self, name: Token, args: Vec<String>, body: Stmt) -> Self::Output {
        let s = Stmt::new(
            StmtType::Function {
                name: name.clone(),
                args: args.clone(),
                body: body.clone(),
            },
            0..0,
            "".to_string(),
        );
        let n = name.lexeme.to_string();

        self.env.define(
            n,
            Var {
                value: Function::create_function(s),
                mutable: false,
                type_: Type::Function,
            },
        );
        none()
    }

    fn visit_class(&mut self, name: String, methods: Vec<Stmt>) -> Self::Output {
        let mut functions = Vec::new();
        let env = self.env.clone();
        let mut interpreter = Interpreter::new_with_env(env);

        for method in methods {
            match *method.stmt_type {
                StmtType::Function {
                    name: e,
                    args: a,
                    body: b,
                } => {
                    interpreter.visit_function(e.clone(), a, b);
                    if let Some(v) = interpreter.env.fetch(e.lexeme.to_string()) {
                        functions.push(v.value)
                    } else {
                        unreachable!()
                    }
                }
                _ => unreachable!(),
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
        self.env.define(
            name.clone(),
            Var {
                value: Class::create_class(name.as_str()),
                mutable: false,
                type_: Type::Class(name.clone()),
            },
        );
        none()
    }

    fn visit_use(&mut self, path: String, as_: String) -> Self::Output {

        let relative_path = PathBuf::from(path);
        let mut absolute_path = std::env::current_dir().unwrap();
        absolute_path.push("src");
        absolute_path.push(relative_path);
        let content = fs::read_to_string(dbg!(absolute_path)).unwrap();

        let mut lexer = Lexer::new(content.clone());
        let mut parser = Parser::new(lexer.scan_token(), content.clone());
        let mut interpreter = Interpreter::new();
        let mut res = none();
        for stmt in parser.parse() {
            res = stmt.accept(&mut interpreter);
        }

        let ns = Namespace::new(interpreter.env.clone());
        self.env.define(
            as_,
            Var {
                value: ns.create(),
                mutable: false,
                type_: Type::Namespace,
            },
        );

        none()

    }

    fn visit_import(&mut self, name: String, imports: Vec<String>) -> Self::Output {
        todo!()
    }

    fn visit_impl(&mut self, struct_name: String, methods: Vec<Stmt>) -> Self::Output {
        todo!()
    }
    fn visit_struct(&mut self, name: String, fields: Vec<(String, Expr)>) -> Self::Output {
        let mut fields_object = Vec::new();

        for (a, b) in fields {
            fields_object.push(StructField {
                name: a,
                type_: b.accept(self).type_,

            })
        }

        self.env.define(
            name.clone(),
            Var {
                value: Object {
                    value: RustValue::Struct(StructType {
                            functions: Vec::new(),
                            fields: fields_object,
                    }),
                    type_: Type::Struct(name.clone()),
                    implementations: Vec::new(),
                },
                mutable: false,
                type_: Type::Struct(name.clone()),
            },
        );

        none()

    }
}
