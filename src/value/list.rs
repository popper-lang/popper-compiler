use std::rc::Rc;
use crate::value::get::Getter;
use crate::value::RustValue;
use crate::value::operation::{PartialEq, Add};
use crate::value::litteral::number;
use super::{Object, Type, Implementation};
use crate::ast::expr::{Expr, ExprType, LiteralType};
use crate::interpreter::Interpreter;

type List = Vec<Object>;

pub fn list(l: Vec<Object>) -> Object {
    let p = Rc::new(l.clone());
    Object {
        type_: Type::List,
        implementations: vec![
            Implementation::Add(p.clone()),
            Implementation::PartialEq(p.clone()),
            Implementation::Get(p)
        ],
        value: RustValue::List(l.clone())
    }
}

impl Add for List {
    fn add(&self, other: Object) -> Object {
        if let RustValue::List(l) = other.value {
            let mut new = self.clone();
            new.extend(l);
            list(new)
        } else {
            panic!("Cannot add {:?} to {}", self, other)
        }
    }
}

impl PartialEq for List {
    fn eq(&self, other: Object) -> bool {
        if let RustValue::List(ref l) = other.value {
            self == l
        } else {
            panic!("Cannot compare {:?} to {}", self, other)
        }
    }
}

impl Getter for List {
    fn fetch(&self, _interpreteur: &mut Interpreter, key: Expr) -> Option<Object> {
        match *key.expr_type {
            ExprType::Call { name, args: _ } => {
                if let ExprType::Ident { ident } = *name.expr_type {
                    if ident.lexeme == "len" {
                        Some(number(self.len() as i32))
                    } else {
                        panic!("Cannot call  on a list")
                    }
                } else {
                    panic!("Cannot call  on a list")
                }
            },
            ExprType::Literal { literal } => {
                if let LiteralType::Number(n) = literal {
                    Some(self[n as usize].clone())
                } else {
                    panic!("Cannot index a list ")
                }
            },
            _ => None
        }
    }
}