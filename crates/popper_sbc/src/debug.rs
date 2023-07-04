
/// Debugger for the IR of the SBC(Simple Bytecode Compiler).

use crate::instr::Instruction;

pub fn debug_instruction(instruction: &Instruction) -> String {
    match instruction {
        Instruction::PushLiteral(literal) => debug_literal(literal),
        Instruction::JIFIncluded(jump) => format!("JumpIfFalse({})", jump),
        Instruction::JmpIncluded(jump) => format!("Jump({})", jump),
        Instruction::Call(name) => format!("Call({})", unsafe { name.as_str()  }),
        Instruction::Add => "Add".to_string(),
        Instruction::Sub => "Sub".to_string(),
        Instruction::Mul => "Mul".to_string(),
        Instruction::Div => "Div".to_string(),
        Instruction::Neg => "Neg".to_string(),
        Instruction::Return => "Return".to_string(),
        Instruction::PushVariable(name) => format!("PushVariable({})", unsafe { name.as_str() }),
        Instruction::Store(name) => format!("Store({})", unsafe { name.as_str() }),
        Instruction::Nop => "Nop".to_string(),
        Instruction::Pop => "Pop".to_string(),

    }
}

pub fn debug_literal(literal: &crate::value::Literal) -> String {
    match literal {
        crate::value::Literal::Integer(value) => format!("Integer({})", value),
        crate::value::Literal::Float(value) => format!("Float({})", value),
        crate::value::Literal::String(value) => format!("String({})", unsafe { value.as_str() }),
        crate::value::Literal::Boolean(value) => format!("Boolean({})", value),
        crate::value::Literal::Null => "Null".to_string(),
    }
}

pub fn debug_instructions(instructions: &[Instruction]) -> String {
    let mut result = String::new();
    for instruction in instructions {
        result.push_str(&format!("{}\n", debug_instruction(instruction)));
    }
    result
}

pub fn debug_ir(ir: &crate::ir_sb::SbcIr) -> String {
    debug_instructions(&ir.instructions.as_slice())
}