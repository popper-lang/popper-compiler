use std::collections::HashMap;
use crate::object::pop_object::PopObject;


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
    pub fn extend(&mut self, env: LLVMEnv<'ctx>) {
        self.var.extend(env.var);
    }
}
