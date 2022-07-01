use std::io::Write;
use std::{collections::HashMap, rc::Rc};

use crate::value::{Value, Type, Var};
use crate::vm::Vm;
use crate::errors::*;
use super::Builtin;

pub struct BuiltinFunction;
impl Builtin for BuiltinFunction {
    type BuiltinValue = (Rc<dyn Fn(HashMap<String, Var>, Vm) -> Result<Value, Error>>, Vec<(String, Type)>);
    fn build() -> HashMap<String, Self::BuiltinValue> {
        let mut map = HashMap::<String, Self::BuiltinValue>::new();
        map.insert(
            "print".to_string(),
            (
                Rc::new(BuiltinFunction::print),
                vec![("msg".to_string(), Type::Any)],
            ),
        );
        map.insert(
            "println".to_string(),
            (
                Rc::new(BuiltinFunction::println),
                vec![("msg".to_string(), Type::Any)],
            ),
        );

        map.insert(
            "len".to_string(),
            (
                Rc::new(BuiltinFunction::len),
                vec![("list".to_string(), Type::List)],
            ),
        );
        map.insert(
            "read".to_string(),
            (
                Rc::new(BuiltinFunction::read),
                vec![("msg".to_string(), Type::String)],
            ),
        );
        
        map
    }
}

impl BuiltinFunction {
    pub fn print(args: HashMap<String, Var>, _vm: Vm) -> Result<Value, Error> {
        for i in args {
            print!("{}", i.1.value.display_value());
        }
        println!();
        Ok(Value::None)
    }

    pub fn println(args: HashMap<String, Var>, _vm: Vm) -> Result<Value, Error> {
        for i in args {
            println!("{}", i.1.value.display_value());
        }
        println!();
        Ok(Value::None)
    }

    pub fn len(args: HashMap<String, Var>, _vm: Vm) -> Result<Value, Error> {
        if args.len() != 1 {
            return Ok(Value::None);
        } else {
            let value = args.get("list").unwrap();
            Ok(match value {
                Var {
                    value: Value::String(s),
                    ..
                } => Value::Number(s.len() as f64),
                Var {
                    value: Value::List(l),
                    ..
                } => Value::Number(l.len() as f64),
                _ => Value::None,
            })
        }
    }

    pub fn read(args: HashMap<String, Var>, _vm: Vm) -> Result<Value, Error> {
        if args.len() != 1 {
            return Ok(Value::None);
        } else {
            let value = args.get("msg").unwrap();
            Ok(match value {
                Var {
                    value: Value::String(s),
                    ..
                } => {
                    let mut input = String::new();
                    print!("{}", s);
                    std::io::stdout().flush().unwrap();
                    std::io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line");
                    if input.ends_with("\n") {
                        input.pop();
                    }
                    Value::String(input.to_string())
                }
                _ => Value::None,
            })
        }
    }
}