use crate::interpreter::Interpreter;
use crate::value::{Implementation, Object, Value, Type};
use crate::value::callable::Callable;
use std::rc::Rc;
use crate::value::int::{none, number};
use crate::value::string::string;
use crate::value::boolean::boolean;
use crate::{create, value_to_rs_value, rs_type_to_type, call_function_with_vec}; // File : src/builtin_function/mod.rs
use crate::define_function;


use super::panic_if_is_outside_std;

#[derive(Clone, Debug)]
pub struct Print;

#[derive(Clone, Debug)]
pub struct Println;


#[derive(Clone, Debug)]
pub struct Input;



create!(Print);
create!(Println);
create!(Input);



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

impl Callable for Input {
    fn call(&self, _interpreter: &mut Interpreter, args: &mut Vec<Object>, file: &str) -> Object {
        panic_if_is_outside_std(file, "_input");
        if args.len() != 1 {
            panic!("expected 1 argument, found {}", args.len());
        }
        let prompt = args.remove(0);
        if let Value::String(s) = prompt.value {
            println!("{}", s);
        } else {
            panic!("expected string, found {}", prompt.type_);
        }
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        string(input.as_str())
    }
}

define_function!(Test(x: i32, y: i32 ) {
    number(x + y)
});
create!(Test);





