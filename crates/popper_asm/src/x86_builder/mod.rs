
#[cfg(test)]
mod tests;

use crate::register::Register;
use crate::asm_value::{AsmValue, Immediate, Memory};
use crate::builder::Assembly;
use crate::builder::Program;


pub struct  X86Builder<'a> {
    program: Program<'a>,
    x86_asm: String
}



impl<'a> X86Builder<'a> {
    pub fn new(program: Program<'a>) -> Self {
        Self {
            program,
            x86_asm: "".to_string()
        }
    }

    pub fn register_to_str(&self, reg: Register) -> String {
        match reg {
            Register::R1 => "eax".to_string(),
            Register::R2 => "ebx".to_string(),
            Register::R3 => "ecx".to_string(),
            Register::R4 => "edx".to_string(),
            Register::R5 => "edi".to_string(),
            Register::R6 => "esi".to_string(),
            Register::R7 => "ecx".to_string(),
            Register::R8 => "r8".to_string(),
            Register::R9 => "r9".to_string(),
            Register::R10 => "r10".to_string(),
            Register::R11 => "r11".to_string(),
            Register::R12 => "r12".to_string(),
            Register::R13 => "r13".to_string(),
            Register::R14 => "r14".to_string(),
            Register::R15 => "r15".to_string(),
            Register::RBP => "rbp".to_string(),
            Register::RNP => "rnp".to_string(),
            Register::Inc(inc, reg) => {
                format!("[{} + {}]", self.register_to_str(*reg), inc)
            },
        }
    }

    pub fn mem_to_str(&self, mem: Memory) -> String {
        match mem {
            Memory::RegisterOffset(reg, offset) => {
                format!("[{} + {}]", self.register_to_str(reg), self.immediate_to_str(offset))
            },
            Memory::Label(label) => {
                format!("[{}]", label)
            },
        }
    }

    pub fn immediate_to_str(&self, imm: Immediate) -> String {
        match imm {
            Immediate::U32(value) => {
                format!("{}", value)
            }
            Immediate::U64(value) => {
                format!("{}", value)
            }
            _ => {
                panic!("Immediate type not supported")
            }
        }
    }

    pub fn asm_value_to_str(&self, value: AsmValue) -> String {
        match value {
            AsmValue::Immediate(imm) => {
                self.immediate_to_str(imm)
            }
            AsmValue::Register(reg) => {
                self.register_to_str(reg)
            }
            AsmValue::Memory(mem) => {
                self.mem_to_str(mem)
            }
        }
    }

    pub fn compile(&mut self) {
        for assembly in self.program.iter() {
            self.x86_asm += (match assembly {
                Assembly::Mov(register, value) => {
                    let register = self.register_to_str(register.clone());
                    let value = self.asm_value_to_str(*value.clone());
                    format!("mov {}, {}",
                            register,
                            value
                    )

                },
                Assembly::Add(register, value, _dest) => {

                    let register = self.register_to_str(register.clone());
                    let value = self.asm_value_to_str(*value.clone());
                    format!("add {}, {}",
                            register,
                            value
                    )
                },
                Assembly::Sub(register, value, _dest) => {
                    let register = self.register_to_str(register.clone());
                    let value = self.asm_value_to_str(*value.clone());
                    format!("sub {}, {}",
                            register,
                            value
                    )
                },
                Assembly::Call(label) => {
                    format!("call {}", label)
                },
                Assembly::Ret => {
                    "ret".to_string()
                },
                Assembly::Label(label, asm) => {
                    let mut compiler = X86Builder::new(asm.clone());
                    compiler.compile();
                    compiler.x86_asm = compiler.x86_asm.replace("\n", "\n\t");
                    format!("{}:\n{}", label, compiler.x86_asm)

                },

                _ => {
                    todo!("compile assembly")
                }
            }.to_string() + "\n").as_str();
        }
    }
}