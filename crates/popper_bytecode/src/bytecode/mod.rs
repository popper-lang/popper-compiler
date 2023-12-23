
#![allow(non_camel_case_types)]
#![allow(dead_code)]

mod instruction;
mod constant;

pub use instruction::*;
pub use constant::*;


#[derive(Clone, Debug, PartialEq)]
pub struct Bytecodes {
    pub instructions: Vec<Bytecode>,
}

impl Bytecodes {
    pub fn new() -> Bytecodes {
        Bytecodes {
            instructions: Vec::new(),
        }
    }

    pub fn add_instruction(&mut self, instruction: Bytecode) {
        self.instructions.push(instruction);
    }

}

impl BytecodeSerializable for Bytecodes {
    fn serialize(&self) -> Vec<u8> {
        let mut vec = Vec::new();
        for instruction in &self.instructions {
            vec.extend_from_slice(&instruction.serialize());
        }
        vec
    }

    fn deserialize(vec: Vec<u8>) -> Self {
        let mut instructions = Vec::new();
        let mut i = 0;
        while i < vec.len() {
            let mut j = i;
            while j < vec.len() && vec[j] != 0 {
                j += 1;
            }
            let instruction = Bytecode::deserialize(vec[i..j].to_vec());
            instructions.push(instruction);
            i = j + 1;
        }
        Self {
            instructions,
        }
    }
}

pub trait BytecodeSerializable {
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(vec: Vec<u8>) -> Self;

}