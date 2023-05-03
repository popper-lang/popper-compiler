use std::rc::Rc;
use crate::value::get::Getter;
use crate::value::RustValue;
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



impl StdLibList for List {
    fn push(interpreteur: &mut Interpreter, mut this: &mut Object, args: &mut Vec<Object>, file: &str) -> Object {
        if args.len() != 1 {
            panic!("expected 1 argument, got {:?}", args);
        }

        let elt = args.get(0).unwrap().clone();
        if let RustValue::List(ref mut  l) = this.value {
            l.push(elt);
            this.value = RustValue::List(l.clone());
        } else {
            unreachable!()
        }



        none()
    }

    fn extend(_interpreteur: &mut Interpreter, this: &mut Object, args: &mut Vec<Object>, _file: &str) -> Object {
        if args.len() != 1 {
            panic!("expected 1 argument, got {:?}", args);
        }

        let mut elt = args.get(0).unwrap().clone();

        if let RustValue::List(ref mut l) = this.value {
            if let RustValue::List(ref mut l2) = elt.value {
                l.append(l2);
            } else {
                panic!("expected list, got {:?}", elt.type_)
            }
        } else {
            unreachable!()
        }
        none()
    }
}

register_stdlib!(List, StdLibList, {
    "push" => push,
    "extend" => extend
});
