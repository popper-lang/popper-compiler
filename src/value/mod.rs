pub mod callable;
pub mod function;
pub mod litteral;
pub mod list;
pub mod instance;
pub mod get;
pub mod class;

use std::{hash::Hash, fmt::Debug};

use self::callable::Callable;
use self::get::{Getter, Setter};

pub trait Object {
    fn display_value(&self) -> String;
    fn get_type(&self) -> Type;
    fn called(self) -> Option<Box<dyn Callable>> {
        None
    }
 
    fn getter(&self) -> Option<Box<dyn Getter>> {
        None
    }

    fn setter(&mut self) -> Option<Box<dyn Setter>> {
        None
    }



} 

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum Type {
    Int,
    String,
    Bool,
    List,
    Func,
    Range,
    Type(String),
    Any,
    None,
    Function,
    Instance(String),
    Class(String)
}




#[derive(Debug, Clone, PartialEq)]
pub struct Var {
    pub value: Box<dyn Object>,
    pub mutable: bool,
    pub type_: Type,
}

impl Clone for Box<dyn Object> {
    fn clone(&self) -> Self {
        self.clone()
    }
}

impl PartialEq for Box<dyn Object> {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}


impl Debug for Box<dyn Object> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Box").finish()
    }
}




impl ToString for Type {
    fn to_string(&self) -> String {
        match self {
            Type::Int => "int".to_string(),
            Type::String => "string".to_string(),
            Type::Bool => "bool".to_string(),
            Type::Func => "func".to_string(),
            Type::List => "list".to_string(),
            Type::Range => "range".to_string(),
            Type::None => "None".to_string(),
            Type::Type(_) => "type".to_string(),
            Type::Any => "any".to_string(),
            Type::Function => "function".to_string(),
            Type::Instance(i) => format!("instance of class {}", i),
            Type::Class(e) => format!("class {}", e),
        }
    }
}

