use crate::bytecode::BytecodeSerializable;
use crate::bytecode::constant::BytecodeConstant;

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum Instruction {
    LOAD_CONST,
    LOAD_VAR,
    STORE_VAR,
    LOAD_FUNC,
    CALL_FUNC,
    OP_ADD,
    OP_SUB,
    OP_MUL,
    OP_DIV,
    OP_MOD,
    OP_POW,
    CMP_EQ,
    CMP_NEQ,
    CMP_GT,
    CMP_LT,
    CMP_GTE,
    CMP_LTE,
    OP_AND,
    OP_OR,
    OP_NOT,
    OP_NEG,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Bytecode {
    pub instruction_type: Instruction,
    pub operand: Vec<BytecodeConstant>,
}

impl Bytecode {
    pub fn new(instruction_type: Instruction, operand: Vec<BytecodeConstant>) -> Self {
        Self {
            instruction_type,
            operand,
        }
    }
}

impl BytecodeSerializable for Bytecode {
    fn serialize(&self) -> Vec<u8> {
        let mut vec = Vec::new();
        vec.push(self.instruction_type as u8);
        for operand in &self.operand {
            vec.extend_from_slice(&operand.serialize());
            vec.push(u8::MAX);
        }
        vec
    }

    fn deserialize(vec: Vec<u8>) -> Self {
        let mut operand = Vec::new();
        let mut i = 1;
        while i < vec.len() {
            let mut j = i;
            while j < vec.len() && vec[j] != u8::MAX {
                j += 1;
            }
            operand.push(BytecodeConstant::deserialize(vec[i..j].to_vec()));
            i = j + 1;
        }
        Self {
            instruction_type: unsafe { std::mem::transmute(vec[0]) },
            operand,
        }
    }
}