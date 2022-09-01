use std::collections::HashMap;
use crate::value::callable::Callable;

pub mod function;
pub mod int;
pub mod string;
pub mod bool;



pub trait Builtin {
    type BuiltinValue;
    fn build() -> HashMap<String, Self::BuiltinValue>;
}






