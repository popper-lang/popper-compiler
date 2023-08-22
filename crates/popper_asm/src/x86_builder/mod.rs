
#[cfg(test)]
mod tests;

use crate::register::Register;
use crate::asm_value::{AsmValue, Immediate, Memory};
use crate::builder::{Assembly, Builder};


pub struct  X86Builder {
    builder: Builder,
    pub x86_asm: String
}



impl X86Builder {
    pub fn new(builder: Builder) -> Self {
        Self {
            builder,
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
        for assembly in self.builder.program.iter() {
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
                Assembly::IAdd(register, value) => {
                    let register = self.register_to_str(register.clone());
                    let value = self.asm_value_to_str(*value.clone());
                    format!("add {}, {}",
                            register,
                            value
                    )
                },
                Assembly::ISub(register, value) => {
                    let register = self.register_to_str(register.clone());
                    let value = self.asm_value_to_str(*value.clone());
                    format!("sub {}, {}",
                            register,
                            value
                    )
                },
                Assembly::Mul(register) => {
                    let register = self.register_to_str(register.clone());
                    format!("mul {}",
                            register
                    )
                },
                Assembly::Call(label) => {
                    format!("call {}", label)
                },
                Assembly::Cmp(lhs, rhs) => {
                    let lhs = self.asm_value_to_str(*lhs.clone());
                    let rhs = self.asm_value_to_str(*rhs.clone());
                    format!("cmp {}, {}",
                            lhs,
                            rhs
                    )
                },
                Assembly::Je(label) => {
                    format!("je {}", label)
                },
                Assembly::Jmp(label) => {
                    format!("jmp {}", label)
                },
                Assembly::Jne(label) => {
                    format!("jne {}", label)
                },
                Assembly::Ret => {
                    "ret".to_string()
                },
                Assembly::Nop => {
                    "nop".to_string()
                },


                e => {
                    todo!("compile assembly: {:?}", e)
                }
            }.to_string() + "\n").as_str();
        }

        for label in self.builder.labels.iter() {

            let mut builder = Builder::new();
            builder.program = label.1.clone();
            let mut x86builder = X86Builder::new(builder);
            x86builder.compile();

            let mut str = x86builder.build();

            str = str.replace("\n", "\n\t");
            self.x86_asm = self.x86_asm.replace("\t", "");
            self.x86_asm += (format!("{}:\n\t{}", label.0, str)).as_str();
        }

    }
    pub fn build(&self) -> String {
        self.x86_asm.clone()
    }

}