use crate::interpreter::Interpreter;
use crate::value::{Implementation, Object, RustValue, Type};
use std::rc::Rc;

use crate::value::callable::Callable;
use crate::value::boolean::boolean;
use crate::get_impl_if_exist;

#[derive(Clone, Debug)]
/// The `==` function.
pub struct IsEqual;

impl IsEqual {
    pub fn create() -> Object {
        Object {
            type_: Type::Function,
            implementations: vec![
                Implementation::Call(Rc::new(Self))
            ],
            value: RustValue::Function
        }
    }
}

impl Callable for IsEqual {

    // TODO: This is a temporary implementation.
    fn call(&self, _interpreter: &mut Interpreter, args: Vec<Object>, _file: &str) -> Object {
        if args.len() != 2 {
            panic!("Expected 2 arguments, got {}", args.len());
        }

        let left: Object = args[0].clone();
        let right: Object = args[1].clone();

        if left.type_ != right.type_ {
            return boolean(false.into());
        }

        return boolean((left.value == right.value).into());
    }
}

#[derive(Clone, Debug)]
/// The `!=` function.
pub struct IsNotEqual;

impl IsNotEqual {
    pub fn create() -> Object {
        Object {
            type_: Type::Function,
            implementations: vec![
                Implementation::Call(Rc::new(IsNotEqual))
            ],
            value: RustValue::Function
        }
    }
}

impl Callable for IsNotEqual {

    fn call(&self, _interpreter: &mut Interpreter, args: Vec<Object>, _file: &str) -> Object {
        if args.len() != 2 {
            panic!("Expected 2 arguments, got {}", args.len());
        }

        let left = args[0].clone();
        let right = args[1].clone();

        let impl_left = get_impl_if_exist!(PartialEq, left);

        if let Some(impl_left) = impl_left {
            return boolean(impl_left.ne(right).into());
        } else {
            boolean(true.into())
        }
    }
}