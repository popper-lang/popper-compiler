use crate::interpreter::Interpreter;
use crate::value::{Implementation, Object, RustValue, Type};
use crate::value::callable::Callable;
use crate::value::list::{list, PopperList};
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
    fn call(&self, interpreter: &mut Interpreter, args: Vec<Object>, file: &str) -> Object {
        println!("BREAK 8");
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
        let mut list_obj: PopperList = PopperList::new();
        if let RustValue::List(e) = &obj.value {
            list_obj = e.clone();
        }
        let mut new_list = PopperList::new();

        for item in list_obj.value {
            new_list.push(new_func.call(interpreter, vec![item.clone()], file));
        }

        list(new_list.into())




    }
}
