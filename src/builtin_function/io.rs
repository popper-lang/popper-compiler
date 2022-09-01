use crate::value::callable::Callable;
use crate::value::Value;
use crate::interpreter::Interpreter;


pub struct Print;
pub struct Println;


impl Callable for Print {
    fn call(&self, _interpreter: &mut Interpreter, args: Vec<Value>) -> Value {
        for i in args {
            print!("{}", i.display_value())
        }
        Value::None
    }
}

impl Callable for Println {
    fn call(&self, _interpreter: &mut Interpreter, args: Vec<Value>) -> Value {
        for i in args {
            println!("{}", i.display_value())
        }
        Value::None
    }
}