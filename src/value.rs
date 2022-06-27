use crate::errors::*;
use crate::expr::ident::Ident;
use crate::vm::Vm;
use std::{collections::HashMap, fmt, hash::Hash, ops::Range, rc::Rc};

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum Type {
    Int,
    String,
    Bool,
    List,
    Func,
    Range,
    Enum,
    FieldEnum(String),
    Struct(String),
    FieldStruct(String),
    Type(String),
    Any,
    None,
    Module(String),
    Function
}

pub struct Function(pub Rc<dyn Fn(HashMap<String, Var>, Vm) -> Result<Value, Error>>);

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Function {
        name: String,
        func: Function,
        args: Vec<(String, Type)>,
    },
    DefStruct {
        name: String,
        fields: Vec<(Ident, Type)>,
        function: HashMap<String, Value>,
    },
    CallStruct {
        name: String,
        fields: HashMap<Ident, Value>,
    },
    List(Vec<Value>),
    Range(Range<isize>),
    Enum {
        variants: Vec<String>,
    },
    EnumCall {
        name: String,
        field: String,
    },
    Type(Type),
    Module {
        context: HashMap<Ident, Var>,
        name: String,
    },
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Var {
    pub value: Value,
    pub mutable: bool,
    pub type_: Type,
}

impl Clone for Function {
    fn clone(&self) -> Self {
        Function(self.0.clone())
    }
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Function")
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        &self.0 as *const _ == &other.0 as *const _
    }
}
impl Hash for Function {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (&self.0 as *const _ as usize).hash(state);
    }
}

impl Eq for Function {}

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

    pub fn display_value(&self) -> String {
        match self {
            Value::Number(n) => n.to_string(),
            Value::String(s) => s.clone(),
            Value::Bool(b) => b.to_string(),
            Value::Function { .. } => "function".to_string(),
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
            Value::DefStruct { .. } => todo!(),
            Value::CallStruct { .. } => todo!(),
            Value::Enum { .. } => todo!(),
            Value::EnumCall { .. } => todo!(),
            Value::Type(n) => n.to_string(),
            Value::Module { name, .. } => format!("module {}", name),
        }
    }

    pub fn get_type(&self) -> Type {
        match self {
            Value::Number(_) => Type::Int,
            Value::String(_) => Type::String,
            Value::Bool(_) => Type::Bool,
            Value::Function { .. } => Type::Func,
            Value::List(_) => Type::List,
            Value::Range(_) => Type::Range,
            Value::CallStruct { name, .. } => Type::FieldStruct(name.clone()),
            Value::DefStruct { name, .. } => Type::Struct(name.clone()),
            Value::None => Type::None,
            Value::Enum { .. } => Type::Enum,
            Value::EnumCall { name, .. } => Type::FieldEnum(name.clone()),
            Value::Type(_) => Type::Type("unknow".to_string()),
            Value::Module { name, .. } => Type::Module(name.to_string()),
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
            Type::FieldStruct(name) => format!("struct {}", name),
            Type::Struct(name) => format!("struct {}", name),
            Type::None => "None".to_string(),
            Type::Enum => "enum".to_string(),
            Type::FieldEnum(name) => format!("enum {}", name),
            Type::Type(_) => "type".to_string(),
            Type::Module(name) => format!("module {}", name),
            Type::Any => "any".to_string(),
            Type::Function => "function".to_string(),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_value())
    }
}
