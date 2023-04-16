use crate::interpreter::Interpreter;
use crate::value::{Implementation, Object, RustValue, Type};
use crate::value::callable::Callable;
use crate::value::list::list;
use crate::get_impl_if_exist;
use std::rc::Rc;
use crate::value::litteral::none;



#[derive(Clone, Debug)]
pub struct Map;

impl Map {
    pub fn create() -> Object {
        Object {
            type_: Type::Function,
            implementations: vec![Implementation::Call(Rc::new(Map))],
            value: RustValue::Function
        }
    }
}

impl Callable for Map {
    fn call(&self, interpreter: &mut Interpreter, args: Vec<Object>) -> Object {
        let mut func = args.first().unwrap();
        let mut new_func = func.implementations.iter().find_map(|e| {
            if let Implementation::Call(e) = e {
                Some(e.clone())
            } else {
                None
            }
        }).unwrap();
        let mut obj = args.last().unwrap();
        let mut list_obj: &Vec<Object> = &Vec::new();
        if let RustValue::List(e) = &obj.value {
            list_obj = e;
        }
        let mut new_list = Vec::new();
        for item in list_obj.iter() {
            new_list.push(dbg!(new_func.call(interpreter, vec![item.clone()])));
        }

        list(new_list)
    }
}
