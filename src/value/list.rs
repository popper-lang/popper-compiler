use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use crate::value::get::Getter;
use crate::value::Value;
use crate::value::operation::{PartialEq, Add};
use crate::value::int::{none, number};
use super::{Object, Type, Implementation};
use crate::ast::expr::{Expr, ExprType, LiteralType};
use crate::interpreter::Interpreter;
use crate::register_stdlib;
use crate::error;
use crate::get_impl_if_exist;
use crate::errors::{Error, ErrorType};
use crate::value::function::BuiltinFunction;
use crate::value::stdlib::StdLibList;
use crate::value::string::string;

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
        value: Value::List(l.clone()),
        tags: std::default::Default::default()

    }
}

impl Add for List {
    fn add(&self, other: Object) -> Object {
        if let Value::List(l) = other.value {
            let mut new = self.clone();
            new = new.into_iter().chain(l.into_iter()).collect();
            list(new)
        } else {
            panic!("Cannot add {:?} to {}", self, other)
        }
    }
}

impl PartialEq for List {
    fn eq(&self, other: Object) -> bool {
        if let Value::List(ref l) = other.value {
            self == l
        } else {
            panic!("Cannot compare {:?} to {}", self, other)
        }
    }
}





impl StdLibList for List {
    fn push(_interpreteur: &mut Interpreter, this: &mut Object, args: &mut Vec<Object>) -> Object {
        if args.len() != 1 {
            panic!("expected 1 argument, got {:?}", args);
        }

        let elt = args.get(0).unwrap().clone();
        if let Value::List(ref mut  l) = this.value {
            l.push(elt);
            this.value = Value::List(l.clone());
        } else {
            unreachable!()
        }



        none()
    }

    fn extend(_interpreteur: &mut Interpreter, this: &mut Object, args: &mut Vec<Object>) -> Object {
        if args.len() != 1 {
            panic!("expected 1 argument, got {:?}", args);
        }

        let mut elt = args.get(0).unwrap().clone();

        if let Value::List(ref mut l) = this.value {
            if let Value::List(ref mut l2) = elt.value {
                l.append(l2);
            } else {
                panic!("expected list, got {:?}", elt.type_)
            }
        } else {
            unreachable!()
        }
        none()
    }

    fn to_string(_interpreteur: &mut Interpreter, this: &mut Object, _args: &mut Vec<Object>) -> Object {
        string(format!("{:?}", this).as_str())
    }

    fn nth(_interpreteur: &mut Interpreter, this: &mut Object, args: &mut Vec<Object>) -> Object {
        if args.len() != 1 {
            panic!("expected 1 argument, got {:?}", args);
        }

        let index = args.get(0).unwrap().clone();

        if let Value::List(ref mut l) = this.value {
            if let Value::Int(n) = index.value {
                let index = n.clone() as usize;
                if index >= l.len() {
                    panic!("index out of range")
                }
                l.get_mut(index).unwrap().clone()
            } else {
                panic!("expected number, got {:?}", index.type_)
            }
        } else {
            unreachable!()
        }
    }



}





register_stdlib!(List, StdLibList, {
    "push" => push,
    "extend" => extend,
    "to_string" => to_string,
    "nth" => nth
    }
);
