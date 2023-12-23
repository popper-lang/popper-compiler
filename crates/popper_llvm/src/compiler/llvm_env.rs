use std::collections::HashMap;
use crate::object::pop_pointer::PopPointer;

pub struct LLVMEnv<'ctx> {
    var: HashMap<String, PopPointer<'ctx>>
}

impl<'ctx> LLVMEnv<'ctx> {
    pub fn new() -> LLVMEnv<'ctx> {
        LLVMEnv {
            var: HashMap::new()
        }
    }

    pub fn get(&self, name: String) -> Option<&PopPointer> {
        self.var.get(&name)
    }

    pub fn set(&mut self, name: String, ptr: PopPointer<'ctx>) {
        self.var.insert(name, ptr);
    }
}
