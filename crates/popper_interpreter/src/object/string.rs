use crate::object::FromPopObject;

#[derive(Debug, PartialEq, Clone)]
pub struct PopString {
    pub value: String,
}

impl PopString {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

impl FromPopObject for PopString {
    fn from_pop_object(pop_object: &crate::object::PopObject) -> Option<&Self> {
        match pop_object {
            crate::object::PopObject::String(e) => Some(e),
            _ => None,
        }
    }
}