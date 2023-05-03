use crate::interpreter::Interpreter;
use super::Object;

pub trait Callable {
    fn call(&self, interpreter: &mut Interpreter, args: &mut Vec<Object>, file: &str) -> Object;

    fn method(&self, interpreteur: &mut Interpreter, this: &mut Object, args: &mut Vec<Object>, file: &str) -> Object {
        self.call(interpreteur, args, file)
    }
}