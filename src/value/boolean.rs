use crate::value::{Object, Value, Type, Implementation};
use crate::value::operation::PartialEq;
use std::rc::Rc;
use crate::register_stdlib;

pub fn boolean(b: bool) -> Object {
    Object {
        type_: Type::Bool,
        implementations: vec![
            Implementation::PartialEq(Rc::new(b)),
        ],
        value: Value::Bool(b),
        tags: std::default::Default::default()

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



/*impl TryInto<bool> for Object {
    type Error = ();

    fn try_into(self) -> Result<bool, Self::Error> {
        self.value.try_into()
    }
}*/

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

impl Into<bool> for Object {
    fn into(self) -> bool {
        if let Ok(res) = self.value.clone().try_into() {
            res
        } else {
            panic!("cant convert {:?} to i32", self.type_)
        }
    }
}

