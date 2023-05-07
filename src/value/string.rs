use crate::value::stdlib::StdLibString;
use crate::value::{Implementation, Object, Value, Type};
use crate::ast::expr::{Expr, ExprType};
use crate::interpreter::Interpreter;
use crate::errors::{Error, ErrorType};
use crate::value::operation::{Add, PartialEq};
use crate::value::int::number;
use std::rc::Rc;
use crate::register_stdlib;
use crate::error;
use crate::get_impl_if_exist;
use crate::value::get::Getter;
use crate::value::function::BuiltinFunction;

pub fn string(s: &str) -> Object {
    Object {
        type_: Type::String,
        implementations: vec![
            Implementation::Add(Rc::new(s.to_string())),
            Implementation::PartialEq(Rc::new(s.to_string())),
            Implementation::Get(Rc::new(s.to_string()))
        ],
        value: Value::String(s.to_string()),
        tags: std::default::Default::default()
    }
}


impl Add for String {
    fn add(&self, other: Object) -> Object {
        if let Value::String(s) = other.value {
            string((self.as_str().to_owned() + s.as_str()).as_str())
        } else {
            panic!("Cannot add {} to {}", self, other)
        }
    }
}

impl PartialEq for String {
    fn eq(&self, other: Object) -> bool {
        if let Value::String(s) = other.value {
            self == &s
        } else {
            panic!("Cannot compare {} to {}", self, other)
        }
    }
}

impl StdLibString for String {
    fn len(_interpreteur: &mut Interpreter, this: &mut Object, args: &mut Vec<Object>) -> Object {
        if args.len() != 0 {
            panic!("expected 0, got {} argument", args.len())
        }

        if let Value::String(s) = dbg!(this.clone().value) {
            return number(s.len() as i32);
        } else {
            unreachable!()
        }
    }
}



/*
register_stdlib!(String, StdLibString, {
<<<<<<< HEAD
    "len" => len_i32(this: Object) {
        number(0)
    }
});*/

/*impl TryInto<String> for Object {
    type Error = ();

    fn try_into(self) -> Result<String, Self::Error> {
        self.value.try_into()
    }
}*/

impl TryInto<String> for Value {
    type Error = ();

    fn try_into(self) -> Result<String, Self::Error> {
        if let Value::String(s) = self {
            Ok(s)
        } else {
            Err(())
        }
    }
}

impl Into<String> for Object {
    fn into(self) -> String {
        if let Value::String(s) = self.value {
            s
        } else {
            panic!("Cannot convert {:?} into String", self)
        }
    }
}


register_stdlib!(String, StdLibString, {
    "len" => len
    }

);


