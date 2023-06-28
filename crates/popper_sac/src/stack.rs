use std::collections::HashMap;
use popper_asm::register::Register;
use popper_asm::asm_value::AsmValue;

pub struct Stack {
    stack: HashMap<Register, AsmValue>,
}

static AVAILABLE_REGISTERS: [Register; 15] = [
    Register::R1,
    Register::R2,
    Register::R3,
    Register::R4,
    Register::R5,
    Register::R6,
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

impl Stack {
    pub fn new() -> Self {
        Self {
            stack: HashMap::new(),
        }
    }

    pub fn push(&mut self, register: Register, value: AsmValue) {
        self.stack.insert(register, value);
    }

    pub fn give_register(&self) -> Option<Register> {
        let mut iter = AVAILABLE_REGISTERS
            .clone()
            .into_iter();

        let mut register = iter
            .next()
            .unwrap()
            .clone();
        while self.stack.contains_key(&register) {
            register = iter
                .next()
                .unwrap()
                .clone();

            if register == Register::R15 {
                return None;
            }
        }
        return Some(register)
    }

    pub fn take_lasts_reg_used(&mut self, n: usize) -> Vec<Register> {
        let mut keys = self.stack.iter().map(|(k, v)| k).cloned().collect::<Vec<Register>>();
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