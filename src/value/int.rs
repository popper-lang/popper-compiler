use std::fmt::Display;
use super::{Object, Type, Implementation};
use std::rc::Rc;
use crate::value::RustValue;
use std::ops::{Add, Sub, Mul, Div};
use crate::value::operation::{Pow, Mod};
use std::cmp::{PartialEq, PartialOrd};

#[derive(Clone, Debug, PartialEq, Ord, PartialOrd, Eq)]
pub struct PopperInt {
    value: i32
}

impl Display for PopperInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<PopperInt> for i32 {
    fn from(value: PopperInt) -> Self {
        value.value
    }
}

impl From<i32> for PopperInt {
    fn from(value: i32) -> Self {
        Self {
            value
        }
    }
}


pub fn number(n: PopperInt) -> Object {
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
        ],
        value: RustValue::Int(n)
    }
}

impl Add for PopperInt {
    fn add(&self, other: Object) -> Object {
        if let RustValue::Int(ref n) = other.value {
            number(self.value + n.value)
        } else {
            panic!("Cannot add {} to {}", self, other)
        }
    }
}

impl Sub for PopperInt {
    fn sub(&self, other: Object) -> Object {
        if let RustValue::Int(n) = other.value {
            number(self.value - n.value)
        } else {
            panic!("Cannot substract {} to {}", self, other)
        }
    }
}

impl Mul for PopperInt {
    fn mul(&self, other: Object) -> Object {
        if let RustValue::Int(n) = other.value {
            number(self.value * n.value)
        } else {
            panic!("Cannot multiply {} to {}", self, other)
        }
    }
}

impl Div for PopperInt {
    fn div(&self, other: Object) -> Object {
        if let RustValue::Int(n) = other.value {
            number(self.value / n.value)
        } else {
            panic!("Cannot divide {} to {}", self, other)
        }
    }
}



impl Mod for PopperInt {
    fn modulo(&self, other: Object) -> Object {
        if let RustValue::Int(n) = other.value {
            number(self % n)
        } else {
            panic!("Cannot modulo {} to {}", self, other)
        }
    }
}







pub fn string(s: &str) -> Object {
    Object {
        type_: Type::String,
        implementations: vec![
            Implementation::Add(Rc::new(s.to_string())),
            Implementation::PartialEq(Rc::new(s.to_string())),
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

pub fn none() -> Object {
    Object {
        type_: Type::None,
        implementations: vec![],
        value: RustValue::None
    }
}