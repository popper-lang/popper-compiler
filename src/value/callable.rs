use crate::interpreter::Interpreter;
use super::Value;

pub trait Callable {
    fn call(&self, interpreter: &mut Interpreter, args: Vec<Value>) -> Value;
}