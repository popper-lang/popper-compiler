use std::fmt::Display;
use super::{Object, Type, Implementation};
use std::rc::Rc;
use crate::value::RustValue;
use crate::value::operation::{Add, PartialEq};


#[derive(Clone, Debug, PartialEq)]
pub struct PopperString {
    pub(crate) value: String
}

impl From<String> for PopperString {
    fn from(value: String) -> Self {
        Self {
            value
        }
    }
}

impl From<PopperString> for String {
    fn from(value: PopperString) -> Self {
        value.value
    }
}

impl Display for PopperString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value.as_str())
    }
}

pub fn string(s: PopperString) -> Object {
    Object {
        type_: Type::String,
        implementations: vec![
            Implementation::Add(Rc::new(s.clone())),
            Implementation::PartialEq(Rc::new(s.clone())),
        ],
        value: RustValue::String(s)
    }
}

impl Add for PopperString {

    fn add(&self, other: Object) -> Object {
        if let RustValue::String(s) = other.value {
            string((self.value.as_str().to_owned() + s.value.as_str()).into())
        } else {
            panic!("Cannot add {} to {}", self.clone().value, other)
        }
    }
}

impl PartialEq for PopperString {
    fn eq(&self, other: Object) -> bool {
        if let RustValue::String(s) = &other.value {
            self.value == s.value
        } else {
            panic!("Cannot compare {} to {}", self.value, other)
        }
    }
}

