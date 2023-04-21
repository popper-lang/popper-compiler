
use std::fs::File;
use std::io::prelude::*;
use crate::bytecodes::bytecode::Bytecode;
use crate::bytecodes::bytecode::Instruction;
use crate::bytecodes::bytecode::Opcode;
use crate::bytecodes::bytecode::Operand;


pub fn write_bytecode_to_file(bytecode: &Bytecode, filename: &str) -> std::io::Result<()> {
    // Opening the file in write mode
    let mut file = File::create(filename)?;

    // Writing the bytecode in the file
    for instr in bytecode.instructions.iter() {
        file.write_all(&instr.to_bytes())?;
    }

    Ok(())
}


/// Reads a bytecode from a file and returns it as a Bytecode object.

pub fn read_bytecode_from_file(filename: &str) -> std::io::Result<Bytecode> {
    let mut file = File::open(filename)?;

    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;

    // Conversion of bytes into a list of instructions
    let mut instructions = Vec::new();
    let mut index = 0;
    while index < bytes.len() {
        let opcode_byte = bytes[index];
        let opcode = match opcode_byte {
            0 => Opcode::LoadConst,
            1 => Opcode::Add,
            2 => Opcode::Subtract,
            3 => Opcode::Multiply,
            4 => Opcode::Divide,
            5 => Opcode::Negate,
            6 => Opcode::If,
            _ => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid opcode")),
        };
        let operand = match opcode {
            Opcode::LoadConst => {
                let mut operand_bytes = [0u8; 4];
                operand_bytes.copy_from_slice(&bytes[index+1..index+5]);
                let int_operand = i32::from_le_bytes(operand_bytes);
                Some(Operand::Int(int_operand))
            },
            Opcode::Add | Opcode::Subtract | Opcode::Multiply | Opcode::Divide => None,
            Opcode::Negate => None,
            Opcode::If => None,
            Opcode::Jump => None,
        };
        let instr = Instruction { opcode, operand };
        instructions.push(instr);
        index += 1 + operand.map(|_| 4).unwrap_or(0);
    }

    // Creation of the Bytecode object from the instructions
    let bytecode = Bytecode { instructions };
    Ok(bytecode)
}


pub fn decompile(bytecode: &Bytecode) -> String {
    let mut output = String::new();
    for instr in bytecode.instructions.iter() {
        match instr.opcode {
            Opcode::LoadConst => {
                if let Some(Operand::Int(value)) = instr.operand {
                    output.push_str(&format!("{} ", value));
                } else if let Some(Operand::Float(value)) = instr.operand {
                    output.push_str(&format!("{} ", value));
                }
            }
            Opcode::Add => output.push_str("+ "),
            Opcode::Subtract => output.push_str("- "),
            Opcode::Multiply => output.push_str("* "),
            Opcode::Divide => output.push_str("/ "),
            Opcode::Negate => output.push_str("- "),
            Opcode::If => output.push_str("if "),
            Opcode::Jump => output.push_str("jump "),
        }
    }
    output
}