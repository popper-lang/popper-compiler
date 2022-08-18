use std::{collections::HashMap, rc::Rc};

use crate::value::{Value, Type, Function, Var};
use crate::interpreter::Interpreter;
use crate::errors::*;
use super::Builtin;


pub struct BuiltinInt;

impl Builtin for BuiltinInt {
    type BuiltinValue = Var;
    fn build() -> HashMap<String, Self::BuiltinValue> {
        let mut map = HashMap::new();

        map.insert("add".to_string(), Var {
            value: Value::Function { 
                name: "add".to_string(),
                func: Function(Rc::new(BuiltinInt::add)),
                args: vec![("right".to_string(), Type::Int)] },
                mutable: false,
            type_: Type::Function,
        });

        map.insert("sub".to_string(), Var {
            value: Value::Function { 
                name: "sub".to_string(),
                func: Function(Rc::new(BuiltinInt::sub)),
                args: vec![("right".to_string(), Type::Int)] },
            mutable: false,
            type_: Type::Function,
        });

        map.insert("mul".to_string(), Var {
            value: Value::Function { 
                name: "mul".to_string(),
                func: Function(Rc::new(BuiltinInt::mul)),
                args: vec![("right".to_string(), Type::Int)] },
            mutable: false,
            type_: Type::Function,
        });

        map.insert("div".to_string(), Var {
            value: Value::Function { 
                name: "div".to_string(),
                func: Function(Rc::new(BuiltinInt::div)),
                args: vec![("right".to_string(), Type::Int)] },
            mutable: false,
            type_: Type::Function,
        });

        map.insert("modulo".to_string(), Var {
            value: Value::Function { 
                name: "modulo".to_string(),
                func: Function(Rc::new(BuiltinInt::modulo)),
                args: vec![("right".to_string(), Type::Int)] },
            mutable: false,
            type_: Type::Function,
        });

        map.insert("pow".to_string(), Var {
            value: Value::Function { 
                name: "pow".to_string(),
                func: Function(Rc::new(BuiltinInt::pow)),
                args: vec![("right".to_string(), Type::Int)] },
            mutable: false,
            type_: Type::Function,
        });

        map.insert("eq".to_string(), Var {
            value: Value::Function { 
                name: "eq".to_string(),
                func: Function(Rc::new(BuiltinInt::eq)),
                args: vec![("right".to_string(), Type::Int)] },
            mutable: false,
            type_: Type::Function,
        });

        map.insert("neq".to_string(), Var {
            value: Value::Function { 
                name: "neq".to_string(),
                func: Function(Rc::new(BuiltinInt::neq)),
                args: vec![("right".to_string(), Type::Int)] },
            mutable: false,
            type_: Type::Function,
        });

        map.insert("lt".to_string(), Var {
            value: Value::Function { 
                name: "lt".to_string(),
                func: Function(Rc::new(BuiltinInt::lt)),
                args: vec![("right".to_string(), Type::Int)] },
            mutable: false,
            type_: Type::Function,
        });

        map.insert("gt".to_string(), Var {
            value: Value::Function { 
                name: "gt".to_string(),
                func: Function(Rc::new(BuiltinInt::gt)),
                args: vec![("right".to_string(), Type::Int)] },
            mutable: false,
            type_: Type::Function,
        });

        map.insert("le".to_string(), Var {
            value: Value::Function { 
                name: "le".to_string(),
                func: Function(Rc::new(BuiltinInt::le)),
                args: vec![("right".to_string(), Type::Int)] },
            mutable: false,
            type_: Type::Function,
        });

        map.insert("ge".to_string(), Var {
            value: Value::Function { 
                name: "ge".to_string(),
                func: Function(Rc::new(BuiltinInt::ge)),
                args: vec![("right".to_string(), Type::Int)] },
            mutable: false,
            type_: Type::Function,
        });

        map.insert("and".to_string(), Var {
            value: Value::Function { 
                name: "and".to_string(),
                func: Function(Rc::new(BuiltinInt::and)),
                args: vec![("right".to_string(), Type::Int)] },
            mutable: false,
            type_: Type::Function,
        });

        map.insert("or".to_string(), Var {
            value: Value::Function { 
                name: "or".to_string(),
                func: Function(Rc::new(BuiltinInt::or)),
                args: vec![("right".to_string(), Type::Int)] },
            mutable: false,
            type_: Type::Function,
        });

        map.insert("opposante".to_string(), Var {
            value: Value::Function { 
                name: "opposante".to_string(),
                func: Function(Rc::new(BuiltinInt::opposante)),
                args: vec![] },
            mutable: false,
            type_: Type::Function,
        });



        map
    }
}

impl BuiltinInt {
    pub fn add(args: HashMap<String, Var>, interpret: &mut Interpreter) -> Result<Value, Error> {

        let left = match interpret.env.fetch("self".to_string()) {
            Some(v) => v,
            None => return Err(Error::AttrNotFound(AttrNotFoundError {
                attr_name: "self".to_string(),
            }))
        };
        let right = args.get("right").unwrap();
        match (left.clone().value, right.clone().value) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
            _ => Err(Error::CannotAdd(CannotAddError {
                left: left.value.to_string(),
                right: right.value.to_string(),
            })),
        }
    }

    pub fn sub(args: HashMap<String, Var>, interpret: &mut Interpreter) -> Result<Value, Error> {
        let left = match interpret.env.fetch("self".to_string()) {
            Some(v) => v,
            None => return Err(Error::AttrNotFound(AttrNotFoundError {
                attr_name: "self".to_string()
            }))
        };
        let right = args.get("right").unwrap();
        match (left.clone().value, right.clone().value) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l - r)),
            _ => Err(Error::CannotSub(CannotSubError {
                left: left.value.to_string(),
                right: right.value.to_string(),
            })),
        }
        
    }

    pub fn mul(args: HashMap<String, Var>, interpret: &mut Interpreter) -> Result<Value, Error> {
        let left = match interpret.env.fetch("self".to_string()) {
            Some(v) => v,
            None => return Err(Error::AttrNotFound(AttrNotFoundError {
                attr_name: "self".to_string()
            }))
        };
        let right = args.get("right").unwrap();
        match (left.clone().value, right.clone().value) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l * r)),
            _ => Err(Error::CannotMul(CannotMulError {
                left: left.value.to_string(),
                right: right.value.to_string(),
            })),
        }
    }

    pub fn div(args: HashMap<String, Var>, interpret: &mut Interpreter) -> Result<Value, Error> {
        let left = match interpret.env.fetch("self".to_string()) {
            Some(v) => v,
            None => return Err(Error::AttrNotFound(AttrNotFoundError {
                attr_name: "self".to_string()
            }))
        };
        let right = args.get("right").unwrap();
        match (left.clone().value, right.clone().value) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l / r)),
            _ => Err(Error::CannotDiv(CannotDivError {
                left: left.value.to_string(),
                right: right.value.to_string(),
            })),
        }
    }

    pub fn modulo(args: HashMap<String, Var>, interpret: &mut Interpreter) -> Result<Value, Error> {
        let left = match interpret.env.fetch("self".to_string()) {
            Some(v) => v,
            None => return Err(Error::AttrNotFound(AttrNotFoundError {
                attr_name: "self".to_string()
            }))
        };
        let right = args.get("right").unwrap();
        match (left.clone().value, right.clone().value) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l % r)),
            _ => Err(Error::CannotMod(CannotModError {
                left: left.value.to_string(),
                right: right.value.to_string(),
            })),
        }
    }

    pub fn pow(args: HashMap<String, Var>, interpret: &mut Interpreter) -> Result<Value, Error> {
        let left = match interpret.env.fetch("self".to_string()) {
            Some(v) => v,
            None => return Err(Error::AttrNotFound(AttrNotFoundError {
                attr_name: "self".to_string()
            }))
        };
        let right = args.get("right").unwrap();
        match (left.clone().value, right.clone().value) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l.powf(r))),
            _ => Err(Error::CannotPow(CannotPowError {
                left: left.value.to_string(),
                right: right.value.to_string(),
            })),
        }
    }

    pub fn eq(args: HashMap<String, Var>, interpret: &mut Interpreter) -> Result<Value, Error> {
        let left = match interpret.env.fetch("self".to_string()) {
            Some(v) => v,
            None => return Err(Error::AttrNotFound(AttrNotFoundError {
                attr_name: "self".to_string()
            }))
        };
        let right = args.get("right").unwrap();
        match (left.clone().value, right.clone().value) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l == r)),
            _ => Err(Error::CannotCompare(CannotCompareError {
                left: left.value.to_string(),
                right: right.value.to_string(),
            })),
        }
        
    }

    pub fn neq(args: HashMap<String, Var>, interpret: &mut Interpreter) -> Result<Value, Error> {
        let left = match interpret.env.fetch("self".to_string()) {
            Some(v) => v,
            None => return Err(Error::AttrNotFound(AttrNotFoundError {
                attr_name: "self".to_string()
            }))
        };
        let right = args.get("right").unwrap();
        match (left.clone().value, right.clone().value) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l != r)),
            (Value::String(l), Value::String(r)) => Ok(Value::Bool(l != r)),
            (Value::Bool(l), Value::Bool(r)) => Ok(Value::Bool(l != r)),
            (Value::List(l), Value::List(r)) => Ok(Value::Bool(l != r)),
            _ => Err(Error::CannotCompare(CannotCompareError {
                left: left.clone().value.to_string(),
                right: right.clone().value.to_string(),
            })),
        }
    }

    pub fn lt(args: HashMap<String, Var>, interpret: &mut Interpreter) -> Result<Value, Error> {
        let left = match interpret.env.fetch("self".to_string()) {
            Some(v) => v,
            None => return Err(Error::AttrNotFound(AttrNotFoundError {
                attr_name: "self".to_string()
            }))
        };
        let right = args.get("right").unwrap();
        match (left.clone().value, right.clone().value) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l < r)),
            _ => Err(Error::CannotCompare(CannotCompareError {
                left: left.clone().value.to_string(),
                right: right.clone().value.to_string(),
            })),
        }
    }

    pub fn gt(args: HashMap<String, Var>, interpret: &mut Interpreter) -> Result<Value, Error> {
        let left = match interpret.env.fetch("self".to_string()) {
            Some(v) => v,
            None => return Err(Error::AttrNotFound(AttrNotFoundError {
                attr_name: "self".to_string()
            }))
        };
        let right = args.get("right").unwrap();
        match (left.clone().value, right.clone().value) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l > r)),
            _ => Err(Error::CannotCompare(CannotCompareError {
                left: left.clone().value.to_string(),
                right: right.clone().value.to_string(),
            })),
        }
    }

    pub fn le(args: HashMap<String, Var>, interpret: &mut Interpreter) -> Result<Value, Error> {
        let left = match interpret.env.fetch("self".to_string()) {
            Some(v) => v,
            None => return Err(Error::AttrNotFound(AttrNotFoundError {
                attr_name: "self".to_string()
            }))
        };
        let right = args.get("right").unwrap();
        match (left.clone().value, right.clone().value) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l <= r)),
            _ => Err(Error::CannotCompare(CannotCompareError {
                left: left.clone().value.to_string(),
                right: right.clone().value.to_string(),
            })),
        }
    }

    pub fn ge(args: HashMap<String, Var>, interpret: &mut Interpreter) -> Result<Value, Error> {
        let left = match interpret.env.fetch("self".to_string()) {
            Some(v) => v,
            None => return Err(Error::AttrNotFound(AttrNotFoundError {
                attr_name: "self".to_string()
            }))
        };
        let right = args.get("right").unwrap();
        match (left.clone().value, right.clone().value) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l >= r)),
            _ => Err(Error::CannotCompare(CannotCompareError {
                left: left.clone().value.to_string(),
                right: right.clone().value.to_string(),
            })),
        }
    }

    pub fn and(args: HashMap<String, Var>, interpret: &mut Interpreter) -> Result<Value, Error> {
        let left = match interpret.env.fetch("self".to_string()) {
            Some(v) => v,
            None => return Err(Error::AttrNotFound(AttrNotFoundError {
                attr_name: "self".to_string()
            }))
        };
        let right = args.get("right").unwrap();
        match (left.clone().value, right.clone().value) {
            (Value::Bool(l), Value::Bool(r)) => Ok(Value::Bool(l && r)),
            _ => Err(Error::CannotCompare(CannotCompareError {
                left: left.clone().value.to_string(),
                right: right.clone().value.to_string(),
            })),
        }
    }

    pub fn or(args: HashMap<String, Var>, interpret: &mut Interpreter) -> Result<Value, Error> {
        let left = match interpret.env.fetch("self".to_string()) {
            Some(v) => v,
            None => return Err(Error::AttrNotFound(AttrNotFoundError {
                attr_name: "self".to_string()
            }))
        };
        let right = args.get("right").unwrap();
        match (left.clone().value, right.clone().value) {
            (Value::Bool(l), Value::Bool(r)) => Ok(Value::Bool(l || r)),
            _ => Err(Error::CannotCompare(CannotCompareError {
                left: left.clone().value.to_string(),
                right: right.clone().value.to_string(),
            })),
        }
    }

    pub fn opposante(args: HashMap<String, Var>, interpret: &mut Interpreter) -> Result<Value, Error> {
        let left = match interpret.env.fetch("self".to_string()) {
            Some(v) => v,
            None => return Err(Error::AttrNotFound(AttrNotFoundError {
                attr_name: "self".to_string()
            })) 
        };
        Ok(Value::Number(match left.value {
            Value::Number(n) => -n,
            _ => return Err(Error::CannotUnaryOp(CannotUnaryOpError {
                operand: left.value.to_string()
        }))}))
    }

}