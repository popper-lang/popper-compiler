
use crate::interpreter::Interpreter;
use crate::value::{Implementation, Object, Value, Type};
use crate::value::callable::Callable;
use std::rc::Rc;
use crate::value::int::{none, number};
use crate::value::string::string;
use crate::value::boolean::boolean;
use crate::{create, call_function_with_vec}; // File : src/builtin_function/mod.rs



use super::panic_if_is_outside_std;
use crate::define_function;



define_function!(Print(msg: String) {
    print!("{}", msg);
    none()
},function_name = "_print");

define_function!(Println(msg: String) {
    println!("{}", msg);
    none()
}, function_name = "_println");

define_function!(Input(prompt: String) {
    println!("{}", prompt);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    string(input.as_str())
}, function_name = "_input");





