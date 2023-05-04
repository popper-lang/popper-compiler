use crate::value::{Object, Value, Type, Implementation};
use crate::value::operation::PartialEq;
use std::rc::Rc;
use std::convert::TryInto;
use crate::errors::Error;
pub fn boolean(b: bool) -> Object {
    Object {
        type_: Type::Bool,
        implementations: vec![
            Implementation::PartialEq(Rc::new(b)),
        ],
        value: Value::Bool(b)
    }
}

impl PartialEq for bool {
    fn eq(&self, other: Object) -> bool {
        if let Value::Bool(b) = other.value {
            self == &b
        } else {
            panic!("Cannot compare {} to {}", self, other)
        }
    }
}



impl TryInto<bool> for Object {
    type Error = ();

    fn try_into(self) -> Result<bool, Self::Error> {
        self.value.try_into()
    }
}

impl TryInto<bool> for Value {
    type Error = ();

    fn try_into(self) -> Result<bool, Self::Error> {
        if let Value::Bool(n) = self {
            Ok(n)
        } else {
            Err(())
        }
    }
}