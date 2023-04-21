use std::mem;

#[derive(Copy, Clone, Debug)]
pub enum Opcode {
    LoadConst,
    Add,
    Subtract,
    Multiply,
    Divide,
    Negate,
    If,
    Jump,
}

#[derive(Copy, Clone, Debug)]
pub enum Operand {
    Int(i32),
    Float(f32),
    Bool(bool),
}

impl Opcode {
    pub fn to_bytes(&self) -> Vec<u8> {
        let opcode_bytes = *self as u8;
        vec![opcode_bytes]
    }

    pub fn priority(&self) -> u8 {
        match self {
            Opcode::Add | Opcode::Subtract => 1,
            Opcode::Multiply | Opcode::Divide => 2,
            Opcode::Negate => 3,
            _ => 0,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Instruction {
    pub opcode: Opcode,
    pub operand: Option<Operand>,
}

impl Instruction {
    pub fn to_bytes(&self) -> Vec<u8> {
        let opcode_bytes = self.opcode as u8;
        let operand_bytes = match &self.operand {
            Some(Operand::Int(i)) => i.to_le_bytes().to_vec(),
            Some(Operand::Float(f)) => {
                let bytes: [u8; 4] = unsafe { mem::transmute(*f) };
                bytes.to_vec()
            }
            Some(Operand::Bool(b)) => vec![*b as u8],
            None => vec![],
        };
        let mut bytes = vec![opcode_bytes];
        bytes.extend_from_slice(&operand_bytes);
        bytes
    }

    pub fn len(&self) -> usize {
        (match &self.operand {
            Some(Operand::Int(_)) => 4,
            Some(Operand::Float(f)) => 4,
            Some(Operand::Bool(_)) => 1,
            None => 0,
        }) + 1
    }
}



#[derive(Debug, Clone)]
pub struct Bytecode {
    pub instructions: Vec<Instruction>,
}

impl Bytecode {
    pub fn new() -> Self {
        Bytecode { instructions: vec![] }
    }

    pub fn add_instruction(&mut self, opcode: Opcode, operand: Option<Operand>) {
        self.instructions.push(Instruction { opcode, operand});
    }

    pub fn ip(&self) -> usize {
        self.instructions.len()
    }
}