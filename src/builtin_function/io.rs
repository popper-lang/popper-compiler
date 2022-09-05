use crate::value::callable::Callable;
use crate::value::{Object, Type};
use crate::interpreter::Interpreter;


pub struct Print;
pub struct Println;


impl Callable for Print {
    fn call(&self, _interpreter: &mut Interpreter, args: Vec<Box<dyn Object>>) -> Box<dyn Object> {
        for i in args {
            print!("{}", i.display_value())
        }
        Box::new(())
    }
}

impl Object for Print {
    fn display_value(&self) -> String {
        "Function".to_string()
    }

    fn get_type(&self) -> Type {
        Type::Function
    }
}

impl Callable for Println {
    fn call(&self, _interpreter: &mut Interpreter, args: Vec<Box<dyn Object>>) -> Box<dyn Object> {
        for i in args {
            println!("{}", i.display_value())
        }
        Box::new(())
    }
}

impl Object for Println {
    fn display_value(&self) -> String {
        "Builtin Function".to_string()
    }

    fn get_type(&self) -> Type {
        Type::Function
    }
}