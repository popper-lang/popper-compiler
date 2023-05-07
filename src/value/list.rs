use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use crate::value::get::Getter;
use crate::value::Value;
use crate::value::operation::{PartialEq, Add};
use crate::value::int::{none, number};
use super::{Object, Type, Implementation};
use crate::ast::expr::{Expr, ExprType, LiteralType};
use crate::interpreter::Interpreter;
use crate::{impl_into, register_stdlib};
use crate::error;
use crate::get_impl_if_exist;
use crate::errors::{Error, ErrorType};
use crate::value::function::BuiltinFunction;
use crate::value::stdlib::StdLibList;
use crate::define_method;
use crate::call_function_with_vec;
use crate::create;
use crate::builtin_function::panic_if_is_outside_std;
use crate::value::callable::Callable;

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
            new.extend(l);
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
    fn push(interpreteur: &mut Interpreter, this: &mut Object, args: &mut Vec<Object>, file: &str) -> Object {
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

    fn extend(_interpreteur: &mut Interpreter, this: &mut Object, args: &mut Vec<Object>, _file: &str) -> Object {
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
}
impl_into!(List, List);

impl From<&mut List> for Object {
    fn from(value: &mut List) -> Self {
        value.into()
    }
}

impl Deref for Object {
    type Target = List;
    fn deref(&self) -> &Self::Target {
        if let Value::List(ref l) = self.value {
            l
        } else {
            panic!("Cannot deref {:?} to List", self)
        }
    }
}

impl DerefMut for Object {

    fn deref_mut(&mut self) -> &mut Self::Target {
        if let Value::List(ref mut l) = self.value {
            l
        } else {
            panic!("Cannot deref {:?} to List", self)
        }
    }
}

impl Object {
    pub fn into_mut(&mut self) -> Option<&mut List> {
        if let Value::List(ref mut l) = self.value {
            Some(l)
        } else {
            None
        }
    }
}

register_stdlib!(List, StdLibList, {
    "push" => push(this: Object, value: Object) {
        if let Value::List(ref mut l) = this.value {
            l.push(value);
        } else {
            unreachable!()
        }
        none()
    },
    "extend" => extend() {
        none()
    }
}
);
