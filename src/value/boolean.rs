use std::fmt::Display;
use super::{Object, Type, Implementation};
use std::rc::Rc;
use crate::value::RustValue;
use crate::value::operation::PartialEq;


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PopperBoolean {
    pub(crate) value: bool,
}

impl PopperBoolean {
    pub fn new(value: bool) -> Self {
        Self { value }
    }
}

impl From<bool> for PopperBoolean {
    fn from(value: bool) -> Self {
        Self { value }
    }
}

impl From<PopperBoolean> for bool {
    fn from(value: PopperBoolean) -> Self {
        value.value
    }
}

pub fn boolean(b: PopperBoolean) -> Object {
    Object {
        type_: Type::Bool,
        implementations: vec![
            Implementation::PartialEq(Rc::new(b)),
        ],
        value: RustValue::Bool(b)
    }
}

impl PartialEq for PopperBoolean {
    fn eq(&self, other: Object) -> bool {
        if let RustValue::Bool(ref b) = other.value {
            self == b
        } else {
            panic!("Cannot compare {:?} to {:?}", self, other)
        }
    }
}

impl Display for PopperBoolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if self.value { "true" } else { "false" })
    }
}