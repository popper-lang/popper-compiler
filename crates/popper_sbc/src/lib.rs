#![feature(vec_into_raw_parts)]

pub mod ir_sb;
pub mod compiler;
pub mod instr;
pub mod value;
pub mod debug;

#[cfg(test)]
mod tests;

