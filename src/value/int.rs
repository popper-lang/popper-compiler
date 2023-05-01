use super::{Object, Type, Implementation};
use std::rc::Rc;
use crate::value::operation::{Add, Sub, Mul, Div, Pow, Mod, PartialEq, PartialOrd};
use crate::value::RustValue;
use crate::value::stdlib::{StdLibString, StdLibInt};
use crate::register_stdlib;
use crate::error;
use crate::get_impl_if_exist;
use crate::ast::expr::{Expr, ExprType};
use crate::interpreter::Interpreter;
use crate::errors::{Error, ErrorType};
use crate::value::get::Getter;
use crate::value::function::BuiltinFunction;

pub fn number(n: i32) -> Object {
    Object {
        type_: Type::Int,
        implementations: vec![
            Implementation::Add(Rc::new(n)),
            Implementation::Sub(Rc::new(n)),
            Implementation::Mul(Rc::new(n)),
            Implementation::Div(Rc::new(n)),
            Implementation::Pow(Rc::new(n)),
            Implementation::Mod(Rc::new(n)),
            Implementation::PartialEq(Rc::new(n)),
            Implementation::PartialOrd(Rc::new(n)),
            Implementation::Get(Rc::new(n))
        ],
        value: RustValue::Int(n)
    }
}

impl Add for i32 {
    fn add(&self, other: Object) -> Object {
        if let RustValue::Int(ref n) = other.value {
            number(self + n)
        } else {
            panic!("Cannot add {} to {}", self, other)
        }
    }
}

impl Sub for i32 {
    fn sub(&self, other: Object) -> Object {
        if let RustValue::Int(n) = other.value {
            number(self - n)
        } else {
            panic!("Cannot substract {} to {}", self, other)
        }
    }
}

impl Mul for i32 {
    fn mul(&self, other: Object) -> Object {
        if let RustValue::Int(n) = other.value {
            number(self * n)
        } else {
            panic!("Cannot multiply {} to {}", self, other)
        }
    }
}



impl Div for i32 {
    fn div(&self, other: Object) -> Object {
        if let RustValue::Int(n) = other.value {
            number(self / n)
        } else {
            panic!("Cannot divide {} to {}", self, other)
        }
    }
}

impl Pow for i32 {
    fn pow(&self, other: Object) -> Object {
        if let RustValue::Int(n) = other.value {
            let mut i = *self;
            for _ in 0..n {
                i *= self;
            }
            number(i)
        } else {
            panic!("Cannot power {} to {}", self, other)
        }
    }
}

impl Mod for i32 {
    fn modulo(&self, other: Object) -> Object {
        if let RustValue::Int(n) = other.value {
            number(self % n)
        } else {
            panic!("Cannot modulo {} to {}", self, other)
        }
    }
}

impl PartialEq for i32 {
    fn eq(&self, other: Object) -> bool {
        if let RustValue::Int(n) = other.value {
            self == &n
        } else {
            panic!("Cannot compare {} to {}", self, other)
        }
    }
}

impl PartialOrd for i32 {
    fn partial_cmp(&self, other: Object) -> Option<std::cmp::Ordering> {
        if let RustValue::Int(n) = other.value {
            if self < &n {
                Some(std::cmp::Ordering::Less)
            } else if self > &n {
                Some(std::cmp::Ordering::Greater)
            } else {
                Some(std::cmp::Ordering::Equal)
            }
        } else {
            panic!("Cannot compare {} to {}", self, other)
        }
    }
}

impl StdLibInt for i32 {
    fn sqrt(interpreteur: &mut Interpreter, args: Vec<Object>, file: &str) -> Object {
        if args.len() != 1 {
            panic!("expected 1, got {} argument", args.len())
        }

        let first_element = args.last().unwrap();
        if let RustValue::Int(i) = first_element.value {
            return number(f64::sqrt(i as f64) as i32);
        } else {
            unreachable!()
        }
    }
}

pub fn none() -> Object {
    Object {
        type_: Type::None,
        implementations: vec![],
        value: RustValue::None
    }
}

register_stdlib!(i32, StdLibInt, "sqrt" => sqrt);
