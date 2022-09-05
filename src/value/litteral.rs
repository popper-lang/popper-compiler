use std::fmt::format;

use super::{Object, Type};

impl Object for String {
    fn get_type(&self) -> Type {
        Type::String
    }

    fn display_value(&self) -> String {
        format!("{}", &self)
    }
}

impl Object for i32 {
    fn get_type(&self) -> Type {
        Type::Int
    }
    fn display_value(&self) -> String {
        format!("{}", &self)
    }
}

impl Object for bool {
    fn get_type(&self) -> Type {
        Type::Bool
    }
    fn display_value(&self) -> String {
        format!("{}", &self)
    }
}

impl Object for () {
    fn display_value(&self) -> String {
        "none".to_string()
    }

    fn get_type(&self) -> Type {
        Type::None
    }
}