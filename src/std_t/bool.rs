use std::{collections::HashMap, rc::Rc};

use crate::value::{Value, Type, Function, Var};
use crate::vm::Vm;
use crate::errors::*;
use super::Builtin;

pub struct BuiltinBool;

impl Builtin for BuiltinBool {
    type BuiltinValue = Var;
    fn build() -> HashMap<String, Self::BuiltinValue> {
        let mut map = HashMap::new();

        map.insert("and".to_string(), Var {
            value: Value::Function { 
                name: "and".to_string(),
                func: Function(Rc::new(BuiltinBool::and)),
                args: vec![("left".to_string(), Type::Bool),("right".to_string(), Type::Bool)] },
            mutable: false,
            type_: Type::Function,
        });

        map.insert("or".to_string(), Var {
            value: Value::Function { 
                name: "or".to_string(),
                func: Function(Rc::new(BuiltinBool::or)),
                args: vec![("left".to_string(), Type::Bool),("right".to_string(), Type::Bool)] },
            mutable: false,
            type_: Type::Function,
        });



        map
    }
}

impl BuiltinBool {
    
    pub fn and(args: HashMap<String, Var>, _vm: Vm) -> Result<Value, Error> {
        let left = args.get("left").unwrap();
        let right = args.get("right").unwrap();
        match (left.clone().value, right.clone().value) {
            (Value::Bool(l), Value::Bool(r)) => Ok(Value::Bool(l && r)),
            _ => Err(Error::CannotCompare(CannotCompareError {
                left: left.clone().value.to_string(),
                right: right.clone().value.to_string(),
            })),
        }
    }

    pub fn or(args: HashMap<String, Var>, _vm: Vm) -> Result<Value, Error> {
        let left = args.get("left").unwrap();
        let right = args.get("right").unwrap();
        match (left.clone().value, right.clone().value) {
            (Value::Bool(l), Value::Bool(r)) => Ok(Value::Bool(l || r)),
            _ => Err(Error::CannotCompare(CannotCompareError {
                left: left.clone().value.to_string(),
                right: right.clone().value.to_string(),
            })),
        }
    }

}