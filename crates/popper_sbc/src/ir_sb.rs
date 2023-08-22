/// 
/// This module contains the implementation of the IR for the SBC(Simple Bytecode Compiler).
/// 


use crate::instr::Instruction;
use crate::value::{ByteArg, ByteType};
use crate::value::{Literal, ByteStr};

/// a ir that store all instruction
#[derive(Clone, Debug)]
pub struct SbcIr {
    pub instructions: Vec<Instruction>,
    ip: usize,
}

impl SbcIr {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            ip: 0,
        }
    }

    pub fn ip(self) -> usize {
        self.ip
    }

    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
        self.ip += 1;
    }

    pub fn emit_int(&mut self, value: i64) {
        self.add_instruction(Instruction::PushLiteral(Literal::Integer(value)));
    }

    pub fn emit_float(&mut self, value: f64) {
        self.add_instruction(Instruction::PushLiteral(Literal::Float(value)));
    }

    pub fn emit_string(&mut self, value: ByteStr) {
        self.add_instruction(Instruction::PushLiteral(Literal::String(value)));
    }

    pub fn emit_bool(&mut self, value: bool) {
        self.add_instruction(Instruction::PushLiteral(Literal::Boolean(value)));
    }

    pub fn emit_null(&mut self) {
        self.add_instruction(Instruction::PushLiteral(Literal::Null));
    }

    pub fn emit_store(&mut self, name: ByteStr) {
        self.add_instruction(Instruction::Store(name));
    }

    pub fn emit_return(&mut self) {
        self.add_instruction(Instruction::Return);
    }

    pub fn emit_add(&mut self) {
        self.add_instruction(Instruction::Add);
    }

    pub fn emit_sub(&mut self) {
        self.add_instruction(Instruction::Sub);
    }

    pub fn emit_mul(&mut self) {
        self.add_instruction(Instruction::Mul);
    }

    pub fn emit_div(&mut self) {
        self.add_instruction(Instruction::Div);
    }

    pub fn emit_variable(&mut self, name: ByteStr) {
        self.add_instruction(Instruction::PushVariable(name));
    }

    pub fn emit_neg(&mut self) {
        self.add_instruction(Instruction::Neg);
    }

    pub fn emit_nop(&mut self) {
        self.add_instruction(Instruction::Nop);
    }

    pub fn emit_pop(&mut self) {
        self.add_instruction(Instruction::Pop);
    }

    pub fn emit_jump_if_false(&mut self, is_included: bool, instrs: Vec<Instruction>) {
        self.add_instruction(Instruction::JIF(is_included, instrs));
    }

    pub fn emit_jump(&mut self, is_included: bool, instrs: Vec<Instruction>) {
        self.add_instruction(Instruction::Jmp(is_included, instrs));
    }

    pub fn emit_jump_if_true(&mut self, is_included: bool, instrs: Vec<Instruction>) {
        self.add_instruction(Instruction::JIT(is_included, instrs));
    }

    pub fn emit_function(&mut self, name: ByteStr, args: Vec<ByteArg>, ty: Box<ByteType>, body: Vec<Instruction>) {
        self.add_instruction(Instruction::StoreFn(name, args, ty, body));
    }

    pub fn emit_call_fn(&mut self, name: ByteStr, args: Vec<Instruction>) {
        self.add_instruction(Instruction::Call(name, args));
    }

    pub fn replace_instruction(&mut self, index: usize, instruction: Instruction) {
        self.instructions[index] = instruction;
    }

}
