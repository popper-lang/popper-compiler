/// 
/// This module contains the implementation of the IR for the SBC(Simple Bytecode Compiler).
/// 


use std::collections::HashMap;
use popper_ast::Span;
use crate::instr::Instruction;
use crate::value::StrPtr;
use crate::value::Literal;

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

    pub fn emit_string(&mut self, value: StrPtr) {
        self.add_instruction(Instruction::PushLiteral(Literal::String(value)));
    }

    pub fn emit_bool(&mut self, value: bool) {
        self.add_instruction(Instruction::PushLiteral(Literal::Boolean(value)));
    }

    pub fn emit_null(&mut self) {
        self.add_instruction(Instruction::PushLiteral(Literal::Null));
    }

    pub fn emit_jump_if_false(&mut self, jump: usize) {
        self.add_instruction(Instruction::JumpIfFalse(jump));
    }

    pub fn emit_jump(&mut self, jump: usize) {
        self.add_instruction(Instruction::Jump(jump));
    }

    pub fn emit_call(&mut self, name: StrPtr) {
        self.add_instruction(Instruction::Call(name));
    }

    pub fn emit_store(&mut self, name: StrPtr) {
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

    pub fn emit_variable(&mut self, name: StrPtr) {
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

    pub fn replace_instruction(&mut self, index: usize, instruction: Instruction) {
        self.instructions[index] = instruction;
    }

}
