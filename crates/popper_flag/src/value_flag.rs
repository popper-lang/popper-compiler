use std::fmt::Display;
use crate::type_flag::TypeFlag;


#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ValueFlag {
    Integer,
    Float,
    String,
    Boolean,
    None,
    Array(TypeFlag)
}

impl Display for ValueFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueFlag::Integer => write!(f, "integer"),
            ValueFlag::Float => write!(f, "float"),
            ValueFlag::String => write!(f, "string"),
            ValueFlag::Boolean => write!(f, "boolean"),
            ValueFlag::None => write!(f, "none"),
            ValueFlag::Array(t) => write!(f, "array of {}", t.to_string()),
        }
    }
}