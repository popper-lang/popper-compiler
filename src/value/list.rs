use std::fmt::Display;
use std::rc::Rc;
use crate::value::get::Getter;
use crate::value::RustValue;
use crate::value::int::number;
use super::{Object, Type, Implementation};
use crate::ast::expr::{Expr, ExprType, LiteralType};
use crate::interpreter::Interpreter;
use crate::value::operation::{Add, PartialEq};

#[derive(Debug, Clone, PartialEq)]
pub struct PopperList {
    pub(crate) value: Vec<Object>
}

impl Display for PopperList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::from("[");
        for (i, v) in self.value.iter().enumerate() {
            s.push_str(&format!("{}", v));
            if i != self.value.len() - 1 {
                s.push_str(", ");
            }
        }
        s.push_str("]");
        write!(f, "{}", s)
    }
}

impl From<PopperList> for Vec<Object> {
    fn from(value: PopperList) -> Self {
        value.value
    }
}

impl From<Vec<Object>> for PopperList {
    fn from(value: Vec<Object>) -> Self {
        Self { value }
    }
}

impl PopperList {
    pub fn new() -> Self {
        Self {
            value: vec![]
        }
    }

    pub fn push(&mut self, value: Object) {
        self.value.push(value)
    }

    pub fn len(&self) -> usize {
        self.value.len()
    }
}

pub fn list(l: PopperList) -> Object {
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

impl Add for PopperList {
    fn add(&self, other: Object) -> Object {
        if let RustValue::List(l) = other.value {
            let mut new = self.value.clone();
            new.extend(l);
            list(new.into())
        } else {
            panic!("Cannot add {:?} to {}", self, other)
        }
    }
}


impl Getter for PopperList {
    fn fetch(&self, _interpreteur: &mut Interpreter, key: Expr) -> Option<Object> {
        match *key.expr_type {
            ExprType::Call { name, args: _ } => {
                if let ExprType::Ident { ident } = *name.expr_type {
                    if ident.lexeme == "len" {
                        Some(number((self.len() as i32).into()))
                    } else {
                        panic!("Cannot call  on a list")
                    }
                } else {
                    panic!("Cannot call  on a list")
                }
            },
            ExprType::Literal { literal } => {
                if let LiteralType::Number(n) = literal {
                    Some(self.value[n as usize].clone())
                } else {
                    panic!("Cannot index a list ")
                }
            },
            _ => None
        }
    }
}

impl Iterator for PopperList {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        self.clone().value.into_iter().next()
    }
}

impl PartialEq for PopperList {

    fn eq(&self, other: Object) -> bool {
        if let RustValue::List(ref l) = other.value {
            self == l
        } else {
            false
        }
    }
}