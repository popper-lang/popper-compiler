use std::{collections::HashMap, rc::Rc};

use crate::value::{Value, Type, Function, Var};
use crate::interpreter::Interpreter;
use crate::errors::*;
use super::Builtin;

pub struct BuiltinString;

impl Builtin for BuiltinString {
    type BuiltinValue = Var;
    fn build() -> HashMap<String, Self::BuiltinValue> {
        let mut map = HashMap::new();

        map.insert("add".to_string(), Var {
            value: Value::Function { 
                name: "add".to_string(),
                func: Function(Rc::new(BuiltinString::add)),
                args: vec![("right".to_string(), Type::String)] },
                mutable: false,
            type_: Type::Function,
        });

        map.insert("mul".to_string(), Var {
            value: Value::Function { 
                name: "mul".to_string(),
                func: Function(Rc::new(BuiltinString::mul)),
                args: vec![("right".to_string(), Type::Int)] },
            mutable: false,
            type_: Type::Function,
        });


        map.insert("eq".to_string(), Var {
            value: Value::Function { 
                name: "eq".to_string(),
                func: Function(Rc::new(BuiltinString::eq)),
                args: vec![("right".to_string(), Type::String)] },
            mutable: false,
            type_: Type::Function,
        });

        map.insert("neq".to_string(), Var {
            value: Value::Function { 
                name: "neq".to_string(),
                func: Function(Rc::new(BuiltinString::neq)),
                args: vec![("right".to_string(), Type::String)] },
            mutable: false,
            type_: Type::Function,
        });

        map.insert("lt".to_string(), Var {
            value: Value::Function { 
                name: "lt".to_string(),
                func: Function(Rc::new(BuiltinString::lt)),
                args: vec![("right".to_string(), Type::String)] },
            mutable: false,
            type_: Type::Function,
        });

        map.insert("gt".to_string(), Var {
            value: Value::Function { 
                name: "gt".to_string(),
                func: Function(Rc::new(BuiltinString::gt)),
                args: vec![("right".to_string(), Type::String)] },
            mutable: false,
            type_: Type::Function,
        });

        map.insert("le".to_string(), Var {
            value: Value::Function { 
                name: "le".to_string(),
                func: Function(Rc::new(BuiltinString::le)),
                args: vec![("right".to_string(), Type::String)] },
            mutable: false,
            type_: Type::Function,
        });

        map.insert("ge".to_string(), Var {
            value: Value::Function { 
                name: "ge".to_string(),
                func: Function(Rc::new(BuiltinString::ge)),
                args: vec![("right".to_string(), Type::String)] },
            mutable: false,
            type_: Type::Function,
        });



        map
    }
}

impl BuiltinString {
    pub fn add(args: HashMap<String, Var>, interpreter: &mut Interpreter) -> Result<Value, Error> {
        let left = match interpreter.env.fetch("self".to_string()) {
            Some(v) => v,
            None => return Err(Error::AttrNotFound(AttrNotFoundError {
                attr_name: "self".to_string()
            }))
        };
        let right = args.get("right").unwrap();
        match (left.clone().value, right.clone().value) {
            (Value::String(l), Value::String(r)) => Ok(Value::String(l + r.as_str())),
            _ => Err(Error::CannotAdd(CannotAddError {
                left: left.value.to_string(),
                right: right.value.to_string(),
            })),
        }
    }

    pub fn mul(args: HashMap<String, Var>, interpreter: &mut Interpreter) -> Result<Value, Error> {
        let left = match interpreter.env.fetch("self".to_string()) {
            Some(v) => v,
            None => return Err(Error::AttrNotFound(AttrNotFoundError {
                attr_name: "self".to_string()
            }))
        };
        let right = args.get("right").unwrap();
        match (left.clone().value, right.clone().value) {
            (Value::String(l), Value::Number(r)) => {
                let mut result = String::new();
                for _ in 0..r as usize {
                    result.push_str(l.as_str());
                }
                Ok(Value::String(result))
            },
            _ => Err(Error::CannotMul(CannotMulError {
                left: left.value.to_string(),
                right: right.value.to_string(),
            })),
        }
    }


    pub fn eq(args: HashMap<String, Var>, interpreter: &mut Interpreter) -> Result<Value, Error> {
        let left = match interpreter.env.fetch("self".to_string()) {
            Some(v) => v,
            None => return Err(Error::AttrNotFound(AttrNotFoundError {
                attr_name: "self".to_string()
            }))
        };
        let right = args.get("right").unwrap();
        match (left.clone().value, right.clone().value) {
            (Value::String(l), Value::String(r)) => Ok(Value::Bool(l == r)),
            _ => Err(Error::CannotCompare(CannotCompareError {
                left: left.value.to_string(),
                right: right.value.to_string(),
            })),
        }
        
    }

    pub fn neq(args: HashMap<String, Var>, interpreter: &mut Interpreter) -> Result<Value, Error> {
        let left = match interpreter.env.fetch("self".to_string()) {
            Some(v) => v,
            None => return Err(Error::AttrNotFound(AttrNotFoundError {
                attr_name: "self".to_string()
            }))
        };
        let right = args.get("right").unwrap();
        match (left.clone().value, right.clone().value) {
            (Value::String(l), Value::String(r)) => Ok(Value::Bool(l != r)),
            _ => Err(Error::CannotCompare(CannotCompareError {
                left: left.clone().value.to_string(),
                right: right.clone().value.to_string(),
            })),
        }
    }

    pub fn lt(args: HashMap<String, Var>, interpreter: &mut Interpreter) -> Result<Value, Error> {
        let left = match interpreter.env.fetch("self".to_string()) {
            Some(v) => v,
            None => return Err(Error::AttrNotFound(AttrNotFoundError {
                attr_name: "self".to_string()
            }))
        };
        let right = args.get("right").unwrap();
        match (left.clone().value, right.clone().value) {
            (Value::String(l), Value::String(r)) => Ok(Value::Bool(l < r)),
            _ => Err(Error::CannotCompare(CannotCompareError {
                left: left.clone().value.to_string(),
                right: right.clone().value.to_string(),
            })),
        }
    }

    pub fn gt(args: HashMap<String, Var>, interpreter: &mut Interpreter) -> Result<Value, Error> {
        let left = match interpreter.env.fetch("self".to_string()) {
            Some(v) => v,
            None => return Err(Error::AttrNotFound(AttrNotFoundError {
                attr_name: "self".to_string()
            }))
        };
        let right = args.get("right").unwrap();
        match (left.clone().value, right.clone().value) {
            (Value::String(l), Value::String(r)) => Ok(Value::Bool(l > r)),
            _ => Err(Error::CannotCompare(CannotCompareError {
                left: left.clone().value.to_string(),
                right: right.clone().value.to_string(),
            })),
        }
    }

    pub fn le(args: HashMap<String, Var>, interpreter: &mut Interpreter) -> Result<Value, Error> {
        let left = match interpreter.env.fetch("self".to_string()) {
            Some(v) => v,
            None => return Err(Error::AttrNotFound(AttrNotFoundError {
                attr_name: "self".to_string()
            }))
        };
        let right = args.get("right").unwrap();
        match (left.clone().value, right.clone().value) {
            (Value::String(l), Value::String(r)) => Ok(Value::Bool(l <= r)),
            _ => Err(Error::CannotCompare(CannotCompareError {
                left: left.clone().value.to_string(),
                right: right.clone().value.to_string(),
            })),
        }
    }

    pub fn ge(args: HashMap<String, Var>, interpreter: &mut Interpreter) -> Result<Value, Error> {
        let left = match interpreter.env.fetch("self".to_string()) {
            Some(v) => v,
            None => return Err(Error::AttrNotFound(AttrNotFoundError {
                attr_name: "self".to_string()
            }))
        };
        let right = args.get("right").unwrap();
        match (left.clone().value, right.clone().value) {
            (Value::String(l), Value::String(r)) => Ok(Value::Bool(l >= r)),
            _ => Err(Error::CannotCompare(CannotCompareError {
                left: left.clone().value.to_string(),
                right: right.clone().value.to_string(),
            })),
        }
    }

}