use std::collections::HashMap;
use popper_asm::register::Register;

pub(crate) type AsmAddr = (usize, Register);

pub(crate) struct Env<'a> {
    pub(crate) env: HashMap<&'a str, AsmAddr>,
}

impl<'a> Env<'a> {
    pub(crate) fn new() -> Self {
        Self {
            env: HashMap::new(),
        }
    }

    pub(crate) fn insert(&mut self, key: &'a str, value: AsmAddr) {
        self.env.insert(key, value);
    }

    pub(crate) fn get(&self, key: &'a str) -> Option<&AsmAddr> {
        self.env.get(key)
    }
}