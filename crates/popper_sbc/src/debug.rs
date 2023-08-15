
/// Debugger for the IR of the SBC(Simple Bytecode Compiler).

use crate::instr::Instruction;

pub fn debug_instruction(instruction: &Instruction) -> String {
    match instruction {
        Instruction::PushLiteral(literal) => debug_literal(literal),
        Instruction::JIF(is_include, instrs) => format!("JumpIfFalse({:?}, is_included={})", instrs, is_include),
        Instruction::Jmp(is_include, instrs) => format!("Jump({:?}, is_included={})", instrs,is_include),
        Instruction::Call(name) => format!("Call({})", name.str),
        Instruction::Add => "Add".to_string(),
        Instruction::Sub => "Sub".to_string(),
        Instruction::Mul => "Mul".to_string(),
        Instruction::Div => "Div".to_string(),
        Instruction::Neg => "Neg".to_string(),
        Instruction::Return => "Return".to_string(),
        Instruction::PushVariable(name) => format!("PushVariable({})", name.str),
        Instruction::Store(name) => format!("Store({})", name.str),
        Instruction::Nop => "Nop".to_string(),
        Instruction::Pop => "Pop".to_string(),
        Instruction::JIT(is_include, instrs) => format!("JumpIfTrue({:?}, is_include={})", instrs, is_include),
        Instruction::StoreFn(s, args, ret, body) => format!("StoreFn {}({:?}, {:?}, {:?})", s.str, args, ret, body)
    }
}

pub fn debug_literal(literal: &crate::value::Literal) -> String {
    match literal {
        crate::value::Literal::Integer(value) => format!("Integer({})", value),
        crate::value::Literal::Float(value) => format!("Float({})", value),
        crate::value::Literal::String(value) => format!("String({})", value.str),
        crate::value::Literal::Boolean(value) => format!("Boolean({})", value),
        crate::value::Literal::Null => "Null".to_string(),
    }
}

pub fn debug_instructions(instructions: &[Instruction]) -> String {
    let mut result = String::new();
    for instruction in instructions.iter().enumerate() {
        result.push_str(&format!("({}) {}\n", instruction.0, debug_instruction(instruction.1)));
    }
    result
}

pub fn debug_ir(ir: &crate::ir_sb::SbcIr) -> String {
    debug_instructions(&ir.instructions.as_slice())
}