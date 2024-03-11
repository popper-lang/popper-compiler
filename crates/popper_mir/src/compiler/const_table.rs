use std::collections::HashMap;
use crate::consts::{ConstKind, Ident};

#[derive(Debug, Clone)]
pub struct ConstTable {
    consts: Vec<(Ident, ConstKind)>
}

impl ConstTable {
    pub fn new() -> Self {
        Self {
            consts: Vec::new()
        }
    }

    pub fn insert(&mut self, id: Ident, c: ConstKind) {
        self.consts.push((id, c));
    }

    pub fn get(&self, id: Ident) -> Option<&ConstKind> {
        for (i, c) in self.consts.iter() {
            if i == &id {
                return Some(c);
            }
        }
        None
    }

    pub fn search(&self, c: &ConstKind) -> Option<&Ident> {
        for (id, const_kind) in self.consts.iter() {
            if const_kind == c {
                return Some(id);
            }
        }
        None
    }


}