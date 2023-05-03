use crate::value::{Object, RustValue, Type, Implementation};
use crate::value::operation::PartialEq;
use std::rc::Rc;
use crate::register_stdlib;

pub fn boolean(b: bool) -> Object {
    Object {
        type_: Type::Bool,
        implementations: vec![
            Implementation::PartialEq(Rc::new(b)),
        ],
        value: RustValue::Bool(b)
    }
}

impl PartialEq for bool {
    fn eq(&self, other: Object) -> bool {
        if let RustValue::Bool(b) = other.value {
            self == &b
        } else {
            panic!("Cannot compare {} to {}", self, other)
        }
    }
}

