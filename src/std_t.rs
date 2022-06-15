
use std::collections::HashMap;
use std::rc::Rc;
use crate::executer::Vm;
use crate::executer::value::Value;
use crate::errors::Error;



pub trait Builtin {
    type BuiltinValue;
    fn build() -> HashMap<String, (Self::BuiltinValue, Vec<String>)>;
}

pub struct BuiltinFunction;

impl Builtin for BuiltinFunction  {
    type BuiltinValue = Rc<dyn Fn(HashMap<String, Value>, Vm) -> Result<Value, Error>>;
    fn build() -> HashMap<String, (Self::BuiltinValue, Vec<String>)> {
        let mut map = HashMap::<String, (Self::BuiltinValue, Vec<String>)>::new();
        map.insert("print".to_string(), (Rc::new(BuiltinFunction::print), vec!["msg".to_string()]));
        map.insert("println".to_string(), (Rc::new(BuiltinFunction::println), vec!["msg".to_string()]));
        map.insert("len".to_string(), (Rc::new(BuiltinFunction::len), vec!["list".to_string()]));
        map
    }
    
}

impl BuiltinFunction {
    pub fn print(args: HashMap<String, Value>, vm: Vm) -> Result<Value, Error> {
        for i in args {
            print!("{}", i.1.display_value());
        }
        Ok(Value::None)
    }

    pub fn println(args: HashMap<String, Value>, vm: Vm) -> Result<Value, Error> {
        for i in args {
            print!("{}", i.1.display_value());
        }
        println!();
        Ok(Value::None)
    }

    pub fn len(args: HashMap<String, Value>, vm: Vm) -> Result<Value, Error> {
        if args.len() != 1 {
            return Ok(Value::None);
        } else {
            let value = args.get("0").unwrap();
            Ok(match value {
                Value::String(s) => Value::Number(s.len() as f64),
                Value::List(l) => Value::Number(l.len() as f64),
                _ => Value::None,
            })
        }
    }
}