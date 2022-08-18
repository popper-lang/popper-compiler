use std::collections::HashMap;
use crate::value::Var;

#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, Var>
}


impl Environment {
    pub fn new() -> Environment {
        Environment { 
            values: HashMap::new()
        }
    }

    pub fn fetch(&mut self, key: String) -> Option<&Var> {
        self.values.get(&key)
    }
    
    pub fn define(&mut self, key: String, value: Var) -> bool {
        if self.defined(key.clone()) {
            false 
        } else {
            self.values.insert(key, value);
            true
        }
    }

    pub fn defined(&mut self, key: String) -> bool { 
        self.values.contains_key(&key)
    }

    pub fn modify(&mut self, key: String, value: Var) -> Option<&Var> {
        if self.defined(key.clone()) {
            self.values.insert(key.clone(), value);
            self.fetch(key)
        } else {
            None
        }
    }



}