pub mod function;
pub mod litteral;
pub mod list;
pub mod instance;
pub mod get;
pub mod class;

use std::{hash::Hash, fmt::Debug, rc::Rc};
use crate::error;
use crate::interpreter::Interpreter;

use self::get::{Getter, Setter};

type Args = Vec<Rc<dyn Object>>;

pub trait Object: Debug {
    fn display_value(&self) -> String;
    fn get_type(&self) -> Type;
    fn is_callable(&self) -> bool {
        false
    }

    fn call(&self, _interpreter: &mut Interpreter, _args: Args) -> Rc<dyn Object> {
        error!("this object cant be call")
    }
 
    fn getter(&self) -> Option<Box<dyn Getter>> {
        None
    }

    fn setter(&mut self) -> Option<Box<dyn Setter>> {
        None
    }

    fn boolean(&self) -> bool {
        false
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




#[derive(Debug, Clone)]
pub struct Var {
    pub value: Rc<dyn Object>,
    pub mutable: bool,
    pub type_: Type,
}


impl PartialEq for Var {
    fn eq(&self, other: &Self) -> bool {
        self.value.display_value() == other.value.display_value() && self.mutable == other.mutable && self.type_ == other.type_
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

