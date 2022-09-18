pub mod function;
pub mod litteral;
pub mod list;
pub mod instance;
pub mod get;
pub mod class;

use std::{hash::Hash, fmt::{Debug, Display}, rc::Rc};
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

    fn call(&self, _interpreter: &mut Interpreter, _args: Vec<Rc<dyn Object>>) -> Rc<dyn Object> {
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

pub trait BinaryOperation {

    fn add(&self, rhs: &Self) -> Option<Self> 
    where Self: Sized {
        None
    }

    fn subtract(&self, rhs: &Self) -> Option<Self> 
    where Self: Sized {
        None 
    }

    fn multiply(&self, rhs: &Self) -> Option<Self> 
    where Self: Sized {
        None     
    }

    fn divide(&self, rhs: &Self) -> Option<Self>
    where Self: Sized  {
        None   
    }

    fn pow(&self, rhs: &Self) -> Option<Self>
    where Self: Sized  {
        None
    }

}


impl BinaryOperation for Rc<dyn Object> {}

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











impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Type::Int => "int",
            Type::String => "string",
            Type::Bool => "bool",
            Type::Func => "func",
            Type::List => "list",
            Type::Range => "range",
            Type::None => "None",
            Type::Type(_) => "type",
            Type::Any => "any",
            Type::Function => "function",
            Type::Instance(i) => "instance",
            Type::Class(e) => "class",
        })

    }
}

