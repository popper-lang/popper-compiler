pub mod callable;
pub mod function;

use crate::interpreter::class::Class;
use crate::interpreter::environement::Environment;
use crate::{errors::*, interpreter::instance::Instance};
use crate::interpreter::Interpreter;

use std::{fmt::{self, Debug}, hash::Hash, ops::Range, rc::Rc};

use self::callable::Callable;

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

pub struct Func(pub  String, pub Rc<dyn Callable>);

impl Debug for Func {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Func").field(&self.0).finish()
    }
}

impl Clone for Func {
    fn clone_from(&mut self, source: &Self)
    {
        *self = source.clone()
    }

    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone())
    }
}

impl PartialEq for Func {


    fn eq(&self, other: &Self) -> bool {
        &self.0 as *const _ == &other.0 as *const _
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Function(Func),
    List(Vec<Value>),
    Range(Range<isize>),
    Type(Type),
    Instance(Instance),
    Class(Class),
    None,
}



pub struct Object {
    pub name: String,
    pub attr: Environment<String, Var>,
    pub type_: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Var {
    pub value: Value,
    pub mutable: bool,
    pub type_: Type,
}



impl Value {
    pub fn add(&self, other: &Value) -> Result<Value, Error> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
            (Value::String(a), Value::String(b)) => Ok(Value::String(a.to_owned() + b.as_str())),
            _ => Err(Error::CannotAdd(CannotAddError {
                left: self.to_string(),
                right: other.to_string(),
            })),
        }
    }
    pub fn sub(&self, other: &Value) -> Result<Value, Error> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
            _ => Err(Error::CannotSub(CannotSubError {
                left: self.to_string(),
                right: other.to_string(),
            })),
        }
    }

    pub fn mul(&self, other: &Value) -> Result<Value, Error> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
            _ => Err(Error::CannotMul(CannotMulError {
                left: self.to_string(),
                right: other.to_string(),
            })),
        }
    }

    pub fn div(&self, other: &Value) -> Result<Value, Error> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a / b)),
            _ => Err(Error::CannotDiv(CannotDivError {
                left: self.to_string(),
                right: other.to_string(),
            })),
        }
    }

    pub fn modulo(&self, other: &Value) -> Result<Value, Error> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a % b)),
            _ => Err(Error::CannotMod(CannotModError {
                left: self.to_string(),
                right: other.to_string(),
            })),
        }
    }

    pub fn pow(&self, other: &Value) -> Result<Value, Error> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a.powf(*b))),
            _ => Err(Error::CannotPow(CannotPowError {
                left: self.to_string(),
                right: other.to_string(),
            })),
        }
    }

    pub fn eq(&self, other: &Value) -> Result<Value, Error> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a == b)),
            (Value::String(a), Value::String(b)) => Ok(Value::Bool(a == b)),
            (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a == b)),
            _ => Err(Error::CannotCompare(CannotCompareError {
                left: self.to_string(),
                right: other.to_string(),
            })),
        }
    }

    pub fn neq(&self, other: &Value) -> Result<Value, Error> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a != b)),
            (Value::String(a), Value::String(b)) => Ok(Value::Bool(a != b)),
            (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a != b)),
            _ => Err(Error::CannotCompare(CannotCompareError {
                left: self.to_string(),
                right: other.to_string(),
            })),
        }
    }

    pub fn gt(&self, other: &Value) -> Result<Value, Error> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(*a > *b)),
            _ => Err(Error::CannotCompare(CannotCompareError {
                left: self.to_string(),
                right: other.to_string(),
            })),
        }
    }

    pub fn lt(&self, other: &Value) -> Result<Value, Error> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(*a < *b)),
            _ => Err(Error::CannotCompare(CannotCompareError {
                left: self.to_string(),
                right: other.to_string(),
            })),
        }
    }

    pub fn ge(&self, other: &Value) -> Result<Value, Error> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(*a >= *b)),
            _ => Err(Error::CannotCompare(CannotCompareError {
                left: self.to_string(),
                right: other.to_string(),
            })),
        }
    }

    pub fn le(&self, other: &Value) -> Result<Value, Error> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(*a <= *b)),
            _ => Err(Error::CannotCompare(CannotCompareError {
                left: self.to_string(),
                right: other.to_string(),
            })),
        }
    }

    pub fn and(&self, other: &Value) -> Result<Value, Error> {
        match (self, other) {
            (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(*a && *b)),
            _ => Err(Error::CannotCompare(CannotCompareError {
                left: self.to_string(),
                right: other.to_string(),
            })),
        }
    }

    pub fn or(&self, other: &Value) -> Result<Value, Error> {
        match (self, other) {
            (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(*a || *b)),
            _ => Err(Error::CannotCompare(CannotCompareError {
                left: self.to_string(),
                right: other.to_string(),
            })),
        }
    }

    pub fn opposante(&self) -> Result<Value, Error> {
        match self {
            Value::Number(n) => Ok(Value::Number(-n)),
            _ => Err(Error::CannotUnaryOp(CannotUnaryOpError {
                operand: self.to_string(),
                }))
        }
    }

    pub fn not(&self) -> Result<Value, Error> {
        match self {
            Value::Bool(n) => Ok(Value::Bool(!n)),
            _ => Err(Error::CannotUnaryOp(CannotUnaryOpError {
                operand: self.to_string(),
                }))
        }
    }
    

    pub fn display_value(&self) -> String {
        match self {
            Value::Number(n) => n.to_string(),
            Value::String(s) => s.clone(),
            Value::Bool(b) => b.to_string(),
            Value::Function(_) => "function".to_string(),
            Value::List(list) => {
                let mut s = String::new();
                s.push_str("[");
                for (i, item) in list.iter().enumerate() {
                    if i > 0 {
                        s.push_str(", ");
                    }
                    s.push_str(&item.display_value());
                }
                s.push_str("]");
                s
            }
            Value::Range(_) => "range".to_string(),
            Value::None => "None".to_string(),
            Value::Type(n) => n.to_string(),
            Value::Instance( ins ) => format!("instance of class {}", ins.class.name),
            Value::Class(c) => format!("class {}", c.name),
        }
    }

    pub fn get_type(&self) -> Type {
        match self {
            Value::Number(_) => Type::Int,
            Value::String(_) => Type::String,
            Value::Bool(_) => Type::Bool,
            Value::Function (_) => Type::Func,
            Value::List(_) => Type::List,
            Value::Range(_) => Type::Range,
            Value::None => Type::None,
            Value::Type(_) => Type::Type("unknow".to_string()),
            Value::Instance(e) => Type::Instance(e.class.name.clone()),
            Value::Class(c) => Type::Class(c.name.clone()),
        }
    }

    pub fn get_object(&self) -> Object {
        match self {
            Value::Number(_) => {
                Object {
                    name: "int".to_string(),
                    attr: Environment::new(None), //BuiltinInt::build(),
                    type_: Type::Int,
                }
            },
            Value::String(_) => {
                Object {
                    name: "string".to_string(),
                    attr: Environment::new(None),//BuiltinString::build(),
                    type_: Type::String,
                }
            },
            Value::Bool(_) => {
                Object {
                    name: "bool".to_string(),
                    attr: Environment::new(None),//BuiltinBool::build(),
                    type_: Type::Bool,
                }
            },
            Value::Function (_) => {
                Object {
                    name: "function".to_string(),
                    attr: Environment::new(None),
                    type_: Type::Func,
                }
            },
            Value::List(_) => {
                Object {
                    name: "list".to_string(),
                    attr: Environment::new(None), // TODO: create a list type
                    type_: Type::List,
                }
            },
            Value::Range(_) => {
                Object {
                    name: "range".to_string(),
                    attr: Environment::new(None), // TODO: create a range type
                    type_: Type::Range,
                }
            },
            Value::None => {
                Object {
                    name: "None".to_string(),
                    attr: Environment::new(None),
                    type_: Type::None,
                }
            },
            Value::Type(_) => todo!(),
            Value::Instance(i) => {
                let map = &i.class.methods;
                Object {
                    name: i.class.name.clone(),
                    attr: map.clone(),
                    type_: Type::Instance(i.class.name.clone()),
                }
            },
            Value::Class(e) => Object {
                name: e.name.clone(),
                attr: Environment::new(None),
                type_: Type::Class(e.name.clone()),
            },
        

        }
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

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_value())
    }
}
