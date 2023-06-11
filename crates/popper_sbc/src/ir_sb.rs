/// 
/// This module contains the implementation of the IR for the SBC(Simple Bytecode Compiler).
/// 


use std::collections::HashMap;
use crate::symbol_table::SymbolTable;
use crate::symbol_table::Symbol;

struct IR {
    instructions: Vec<Instruction>,
    symbol_table: SymbolTable,
}

impl IR {
    fn new() -> Self {
        Self {
            instructions: Vec::new(),
            symbol_table: SymbolTable::new(),
        }
    }

    fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    fn add_symbol(&mut self, symbol: Symbol) {
        self.symbol_table.insert(symbol);
    }

    fn get_symbol(&self, name: &str) -> Option<Symbol> {
        self.symbol_table.get(name)
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    PushLiteral(Literal),
    PushVariable(StrPtr),
    JumpIfFalse(usize),
    Jump(usize),
    Call(StrPtr),
    Return,
}

#[derive(Debug, Clone)]
enum Literal {
    Integer(i64),
    Float(f64),
    String(StrPtr),
    Boolean(bool),
}

#[derive(Debug, Clone, Copy)]
struct StrPtr {
    ptr: *const u8,
    len: usize,
}

impl StrPtr {
    fn new(ptr: *const u8, len: usize) -> Self {
        Self { ptr, len }
    }
    
    unsafe fn as_str(&self) -> &str {
        unsafe {
            let slice = std::slice::from_raw_parts(self.ptr, self.clone().len);
            std::str::from_utf8_unchecked(slice)
        }
    }
}