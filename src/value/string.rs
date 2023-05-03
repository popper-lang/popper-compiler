use crate::value::stdlib::StdLibString;
use crate::value::{Implementation, Object, RustValue, Type};
use crate::register_stdlib;
use crate::error;
use crate::get_impl_if_exist;
use crate::ast::expr::{Expr, ExprType};
use crate::interpreter::Interpreter;
use crate::errors::{Error, ErrorType};
use crate::value::get::Getter;
use crate::value::function::BuiltinFunction;
use crate::value::operation::{Add, PartialEq};
use crate::value::int::number;
use std::rc::Rc;

pub fn string(s: &str) -> Object {
    Object {
        type_: Type::String,
        implementations: vec![
            Implementation::Add(Rc::new(s.to_string())),
            Implementation::PartialEq(Rc::new(s.to_string())),
            Implementation::Get(Rc::new(s.to_string()))
        ],
        value: RustValue::String(s.to_string())
    }
}


impl Add for String {
    fn add(&self, other: Object) -> Object {
        if let RustValue::String(s) = other.value {
            string((self.as_str().to_owned() + s.as_str()).as_str())
        } else {
            panic!("Cannot add {} to {}", self, other)
        }
    }
}

impl PartialEq for String {
    fn eq(&self, other: Object) -> bool {
        if let RustValue::String(s) = other.value {
            self == &s
        } else {
            panic!("Cannot compare {} to {}", self, other)
        }
    }
}

impl StdLibString for String {
    fn len(_interpreteur: &mut Interpreter, this: &mut Object, args: &mut Vec<Object>, file: &str) -> Object {
        if args.len() != 0 {
            panic!("expected 0, got {} argument", args.len())
        }

        if let RustValue::String(s) = dbg!(this.clone().value) {
            return number(s.len() as i32);
        } else {
            unreachable!()
        }
    }
}

register_stdlib!(String, StdLibString, {
    "len" => len
});