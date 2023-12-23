use crate::object::{FromPopObject, PopObject};

#[derive(Debug, PartialEq, Clone)]
pub struct PopBoolean {
    pub value: bool,
}

impl PopBoolean {
    pub fn new(value: bool) -> Self {
        Self { value }
    }
}

impl FromPopObject for PopBoolean {
    fn from_pop_object(pop_object: &PopObject) -> Option<&Self> {
        match pop_object {
            PopObject::Boolean(e) => Some(e),
            _ => None,
        }
    }
}