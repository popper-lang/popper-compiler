use crate::interpreter::Interpreter;
use super::Object;

pub trait Callable {
    fn call(&self, interpreter: &mut Interpreter, args: Vec<Object>) -> Object;
}