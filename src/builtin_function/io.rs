use crate::value::{Object, Type};
use crate::interpreter::Interpreter;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Print;

#[derive(Clone, Debug)]
pub struct Println;




impl Object for Print {
    fn display_value(&self) -> String {
        "Function".to_string()
    }

    fn get_type(&self) -> Type {
        Type::Function
    }
    
    fn call(&self, _interpreter: &mut Interpreter, args: Vec<Rc<dyn Object>>) -> Rc<dyn Object> {
        for arg in args {
            print!("{}", arg.display_value());
        }
        Rc::new(())
    }
}


impl Object for Println {
    fn display_value(&self) -> String {
        "Builtin Function".to_string()
    }

    fn get_type(&self) -> Type {
        Type::Function
    }

    fn call(&self, _interpreter: &mut Interpreter, args: Vec<Rc<dyn Object>>) -> Rc<dyn Object> {
        for arg in args {
            print!("{}", arg.display_value());
        }
        println!();
        Rc::new(())
    }
}