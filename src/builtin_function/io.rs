use crate::interpreter::Interpreter;
use crate::value::{Implementation, Object, RustValue, Type};
use crate::value::callable::Callable;
use std::rc::Rc;
use crate::value::int::none;


use super::panic_if_is_outside_std;

#[derive(Clone, Debug)]
pub struct Print;

#[derive(Clone, Debug)]
pub struct Println;

impl Print {
    pub fn create() -> Object {
        Object {
            type_: Type::Function,
            implementations: vec![Implementation::Call(Rc::new(Print))],
            value: RustValue::Function
        }
    }
}

impl Println {
    pub fn create() -> Object {
        Object {
            type_: Type::Function,
            implementations: vec![Implementation::Call(Rc::new(Println))],
            value: RustValue::Function
        }
    }
}

impl Callable for Print {


    fn call(&self, _interpreter: &mut Interpreter, args: &mut Vec<Object>, file: &str) -> Object {
        panic_if_is_outside_std(file, "_print");
        for arg in args {
            print!("{}", arg);
        }
        none()
    }
}

impl Callable for Println {


    fn call(&self, _interpreter: &mut Interpreter, args: &mut Vec<Object>, file: &str) -> Object {
        panic_if_is_outside_std(file, "_println");
        for arg in args {
            print!("{}", arg);
        }
        println!();
        none()
    }
}
