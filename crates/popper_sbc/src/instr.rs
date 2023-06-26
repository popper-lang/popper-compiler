use crate::value::Literal;
use crate::value::StrPtr;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Clone)]
pub enum Instruction {
    PushLiteral(Literal),
    PushVariable(StrPtr),
    JumpIfFalse(usize),
    Jump(usize),
    Call(StrPtr),
    Store(StrPtr),
    Add,
    Sub,
    Mul,
    Div,
    Neg,
    Return,
    Nop,
    Pop
}

pub trait Bytecode: Sized {
    fn to_bytecode(&self) -> Vec<u8>;
    fn from_bytecode(bytecode: Vec<u8>) -> Self;
    fn write_file(&self, file: &str) {
        let bytecode = self.to_bytecode();
        std::fs::write(file, bytecode).expect("Unable to write file");
    }

    fn read_file(file: &str) -> Self {
        let bytecode = std::fs::read(file).expect("Unable to read file");
        Self::from_bytecode(bytecode)
    }
}

impl Bytecode for Instruction {
    fn to_bytecode(&self) -> Vec<u8> {
        match self {
            Instruction::PushLiteral(lit) => {
                let mut bytecode = vec![0x01];
                bytecode.extend(lit.to_bytecode());
                bytecode
            },
            Instruction::PushVariable(name) => {
                let mut bytecode = vec![0x02];
                bytecode.extend(name.to_bytecode());
                bytecode
            },
            Instruction::JumpIfFalse(jump) => {
                let mut bytecode = vec![0x03];
                bytecode.extend(jump.to_bytecode());
                bytecode
            },
            Instruction::Jump(jump) => {
                let mut bytecode = vec![0x04];
                bytecode.extend(jump.to_bytecode());
                bytecode
            },
            Instruction::Call(name) => {
                let mut bytecode = vec![0x05];
                bytecode.extend(name.to_bytecode());
                bytecode
            },
            Instruction::Store(name) => {
                let mut bytecode = vec![0x06];
                bytecode.extend(name.to_bytecode());
                bytecode
            },
            Instruction::Add => vec![0x07],
            Instruction::Sub => vec![0x08],
            Instruction::Mul => vec![0x09],
            Instruction::Div => vec![0x0A],
            Instruction::Neg => vec![0x0B],
            Instruction::Return => vec![0x0C],
            Instruction::Nop => vec![0x0D],
            Instruction::Pop => vec![0x0E],
        }
    }

    fn from_bytecode(bytecode: Vec<u8>) -> Self {
        match bytecode[0] {
            0x01 => Instruction::PushLiteral(Literal::from_bytecode(bytecode[1..].to_vec())),
            0x02 => Instruction::PushVariable(StrPtr::from_bytecode(bytecode[1..].to_vec())),
            0x03 => Instruction::JumpIfFalse(usize::from_bytecode(bytecode[1..].to_vec())),
            0x04 => Instruction::Jump(usize::from_bytecode(bytecode[1..].to_vec())),
            0x05 => Instruction::Call(StrPtr::from_bytecode(bytecode[1..].to_vec())),
            0x06 => Instruction::Store(StrPtr::from_bytecode(bytecode[1..].to_vec())),
            0x07 => Instruction::Add,
            0x08 => Instruction::Sub,
            0x09 => Instruction::Mul,
            0x0A => Instruction::Div,
            0x0B => Instruction::Neg,
            0x0C => Instruction::Return,
            0x0D => Instruction::Nop,
            0x0E => Instruction::Pop,
            _ => panic!("Invalid bytecode")
        }
    }
}



