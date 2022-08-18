use crate::errors::*;
use crate::interpreter::Interpreter;
use crate::types::int::BuiltinInt;
use crate::types::string::BuiltinString;
use crate::types::bool::BuiltinBool;
use crate::types::Builtin;
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

pub struct Function(pub Rc<dyn Fn(HashMap<String, Var>, &mut Interpreter) -> Result<Value, Error>>);

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
        fields: Vec<(String, Type)>,
        function: HashMap<String, Value>,
    },
    CallStruct {
        name: String,
        fields: HashMap<String, Value>,
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
        context: HashMap<String, Var>,
        name: String,
    },
    None,
}

pub struct Object {
    pub name: String,
    pub attr: HashMap<String, Var>,
    pub type_: Type,
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
            Value::CallStruct { name, .. } => Type::Struct(name.clone()),
            Value::DefStruct { name, .. } => Type::Struct(name.clone()),
            Value::None => Type::None,
            Value::Enum { .. } => Type::Enum,
            Value::EnumCall { name, .. } => Type::FieldEnum(name.clone()),
            Value::Type(_) => Type::Type("unknow".to_string()),
            Value::Module { name, .. } => Type::Module(name.to_string()),
        }
    }

    pub fn get_object(&self) -> Object {
        match self {
            Value::Number(_) => {
                Object {
                    name: "int".to_string(),
                    attr: BuiltinInt::build(),
                    type_: Type::Int,
                }
            },
            Value::String(_) => {
                Object {
                    name: "string".to_string(),
                    attr: BuiltinString::build(),
                    type_: Type::String,
                }
            },
            Value::Bool(_) => {
                Object {
                    name: "bool".to_string(),
                    attr: BuiltinBool::build(),
                    type_: Type::Bool,
                }
            },
            Value::Function { .. } => {
                Object {
                    name: "function".to_string(),
                    attr: HashMap::new(),
                    type_: Type::Func,
                }
            },
            Value::List(_) => {
                Object {
                    name: "list".to_string(),
                    attr: HashMap::new(), // TODO: create a list type
                    type_: Type::List,
                }
            },
            Value::Range(_) => {
                Object {
                    name: "range".to_string(),
                    attr: HashMap::new(), // TODO: create a range type
                    type_: Type::Range,
                }
            },
            Value::CallStruct { name, fields, .. } => {
                let mut map = HashMap::new();
                for (k, v) in fields.iter() {
                    map.insert(k.clone(), Var {
                        value: v.clone(),
                        type_: v.get_type(),
                        mutable: false
                    });
                }
                Object {
                    name: name.clone(),
                    attr: map,
                    type_: Type::FieldStruct(name.clone()),
                }
            },
            Value::DefStruct { name, fields: _, function } => {
                let mut f = HashMap::new();
                for (k, v) in function {
                    f.insert(k.clone(), Var { value: v.clone(), type_: v.get_type(), mutable: false });
                }
                
                Object {
                    name: name.clone(),
                    attr: f,
                    type_: Type::Struct(name.clone()),
                }
            },
            Value::None => {
                Object {
                    name: "None".to_string(),
                    attr: HashMap::new(),
                    type_: Type::None,
                }
            },
            Value::Enum { .. } => {
                Object {
                    name: "enum".to_string(),
                    attr: HashMap::new(),
                    type_: Type::Enum,
                }
            },
            Value::EnumCall { name, .. } => {
                Object {
                    name: name.clone(),
                    attr: HashMap::new(),
                    type_: Type::FieldEnum(name.clone()),
                }
            },
            Value::Type(_) => todo!(),
            Value::Module { context, name } => {
                let mut map = HashMap::new();
                for (k, v) in context.iter() {
                    map.insert(k.clone(), v.clone());
                }
                Object {
                    name: name.clone(),
                    attr: map,
                    type_: Type::Module(name.clone()),
                }
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
