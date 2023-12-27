use std::collections::HashMap;
use crate::object::pop_object::PopObject;
use crate::object::pop_pointer::PopPointer;

#[derive(Debug, Clone)]
pub struct LLVMEnv<'ctx> {
    var: HashMap<String, PopObject<'ctx>>
}

impl<'ctx> LLVMEnv<'ctx> {
    pub fn new() -> LLVMEnv<'ctx> {
        LLVMEnv {
            var: HashMap::new()
        }
    }

    pub fn get(&self, name: String) -> Option<&PopObject> {
        self.var.get(&name)
    }

    pub fn set(&mut self, name: String, obj: PopObject<'ctx>) {
        self.var.insert(name, obj);
    }
}
