#![allow(clippy::new_without_default)]
#![allow(clippy::filter_next)]

use std::collections::HashMap;
use popper_asm::register::Register;
use popper_asm::asm_value::AsmValue;

/// stack save register, and give register who is useless
#[derive(Clone, Debug)]
pub struct Stack {
    stack: HashMap<Register, AsmValue>,
}

pub static AVAILABLE_REGISTERS: [Register; 9] = [
    Register::R7,
    Register::R8,
    Register::R9,
    Register::R10,
    Register::R11,
    Register::R12,
    Register::R13,
    Register::R14,
    Register::R15,
];

pub static AVAILABLE_ARG_REGISTER: [Register; 6] = [
    Register::R1,
    Register::R2,
    Register::R3,
    Register::R4,
    Register::R5,
    Register::R6,
];


impl Stack {
    pub fn new() -> Self {
        Self {
            stack: HashMap::new(),
        }
    }

    pub fn push(&mut self, register: Register, value: AsmValue) {
        self.stack.insert(register, value);
    }

    pub fn give_register(&mut self, list: &[Register]) -> Option<Register> {
        list
            .iter()
            .filter(|reg| !self.stack.contains_key(reg))
            .next()
            .cloned()
            .map(move |ref e| {
                self.stack.remove(e);
                e.clone()
            } )
    }
    pub fn give_normal_register(&mut self) -> Option<Register> {
        self.give_register(&AVAILABLE_REGISTERS)
    }

    pub fn give_arg_register(&mut self) -> Option<Register> {
        self.give_register(&AVAILABLE_ARG_REGISTER)
    }
    pub fn take_lasts_reg_used(&mut self, n: usize) -> Vec<Register> {
        let mut keys = self.stack.iter().map(|(k, _v)| k).cloned().collect::<Vec<Register>>();
        keys.reverse();
        keys.into_iter().take(n).collect()
    }


    pub fn register_uses(&self) -> Vec<Register> {
        self.stack.keys().cloned().collect()
    }

    pub fn free_register(&mut self, register: Register) {
        self.stack.remove(&register);
    }



    pub fn free_all_registers(&mut self) {
        self.stack.clear();
    }
}

#[derive(Clone)]
pub struct StackEnv {
    env: HashMap<String, Register>
}

impl StackEnv {
    pub fn new() -> Self {
        Self {
            env: HashMap::new()
        }
    }

    pub fn push(&mut self, name: String, register: Register) {
        self.env.insert(name, register);
    }

    pub fn get(&self, name: &str) -> Option<Register> {
        self.env.get(name).cloned()
    }
}