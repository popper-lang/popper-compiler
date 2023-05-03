use crate::interpreter::Interpreter;
use crate::value::{Implementation, Object, RustValue, Type};
use crate::value::callable::Callable;
use crate::value::list::list;
use std::rc::Rc;
use super::panic_if_is_outside_std;



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
    fn call(&self, interpreter: &mut Interpreter, args: &mut Vec<Object>, file: &str) -> Object {
        panic_if_is_outside_std(file, "_map");
        let func = args.first().unwrap();
        let new_func = func.implementations.iter().find_map(|e| {
            if let Implementation::Call(e) = e {
                Some(e.clone())
            } else {
                None
            }
        }).unwrap();
        let obj = args.last().unwrap();
        let mut list_obj: &Vec<Object> = &Vec::new();
        if let RustValue::List(e) = &obj.value {
            list_obj = e;
        }
        let mut new_list = Vec::new();

        for item in list_obj.iter() {
            new_list.push(new_func.call(interpreter, &mut vec![item.clone()], file));
        }

        list(new_list)




    }
}
