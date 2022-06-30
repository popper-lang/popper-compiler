use std::collections::HashMap;


pub mod function;
pub mod int;
pub mod string;
pub mod bool;



pub trait Builtin {
    type BuiltinValue;
    fn build() -> HashMap<String, Self::BuiltinValue>;
}




