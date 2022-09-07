use std::{collections::HashMap, hash::Hash};

#[derive(Debug, Clone, PartialEq)]
pub struct Environment<K, V> 
where K: PartialEq + Eq + Hash + Clone,
      V: PartialEq + Clone {
    values: HashMap<K, V>,
    enclosing: Box<Option<Environment<K, V>>>
}


impl<K, V> Environment<K, V> 
where K: PartialEq + Eq + Hash + Clone, 
      V: PartialEq + Clone {
    pub fn new(enclosing: Option<Environment<K, V>>) -> Environment<K, V> {
        if let Some(e) = enclosing {
            Environment { 
                values: HashMap::new(),
                enclosing: Box::new(Some(e))
            }
        } else {
            Environment { 
                values: HashMap::new(),
                enclosing: Box::new(None)
            }
        }
    }

    pub fn fetch(&mut self, key: K) -> Option<V> {
        if let Some(mut e) = *self.enclosing.clone() { 
            e.fetch(key)
        } else {
            self.values.get(&key).map(|e| e.clone())
        }
        
    }
    
    pub fn define(&mut self, key: K, value: V) -> bool {

        if self.defined(key.clone()) {
            false 
        } else {
            if let Some(ref mut e) = *self.enclosing { 
                e.define(key, value);
            } else {    
                self.values.insert(key, value);
            }
            true
        }
    }

    pub fn defined(&mut self, key: K) -> bool { 
        self.values.contains_key(&key)
    }

    pub fn modify(&mut self, key: K, value: V) -> Option<V> {
        if self.defined(key.clone()) {
            self.values.insert(key.clone(), value);
            self.fetch(key)
        } else {
            None
        }
    }

    pub fn extend(&mut self, env: Self) {
        self.values.extend(env.values);
    }

    pub fn get_at(&mut self, distance: i32, key: K) -> Option<V> {
        if let Some(mut e) = self.ancestore(distance) { 
            e.fetch(key)
            
        } else { None }
    }

    pub fn define_at(&mut self, distance: i32, key: K, value: V) -> bool {
        if let Some(ref mut e) = self.ancestore(distance) { 
            e.define(key, value) 
        } else { false }
    }

    pub fn ancestore(&mut self, distance: i32) -> Option<Environment<K, V>> {
        let mut env = Some(self.clone());
        for _ in 0..distance as usize {
            env = if let Some(e) = env {
                *e.enclosing
            } else {
                None
            };
        }
        env   
    }



}