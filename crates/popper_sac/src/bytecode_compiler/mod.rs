use popper_asm::builder::{Builder, Program};
use popper_sbc::instr::Instruction;
use popper_sbc::value::Literal;
use popper_asm::register::Register;
use popper_asm::asm_value::{AsmValue, Immediate};
use crate::stack::Stack;

type BytecodeProgram = Vec<Instruction>;


#[derive(Clone)]
pub struct Compiler<'a> {
    builder: Builder<'a>,
    bytecode: BytecodeProgram,
    stack: Stack,
}


impl<'a> Compiler<'a> {
    pub fn new(bytecode: BytecodeProgram) -> Self {
        Self {
            builder: Builder::new(),
            bytecode,
            stack: Stack::new(),
        }
    }

    pub fn compile(&mut self)  {
        for instr in self.bytecode.iter() {
            match instr {
                Instruction::PushLiteral(literal) => {
                    match literal {
                        Literal::Integer(value) => {
                            let asm_value = AsmValue::Immediate(
                                Immediate::U32(value.clone() as u32)
                            );
                            let register = match self.stack.give_register() {
                                Some(register) => register,
                                None => panic!("No more registers available")
                            };

                            self.stack.push(register.clone(),
                                            asm_value.clone()
                            );

                            self.builder.build_mov(register, asm_value);
                        }
                        Literal::Float(_value) => {
                            todo!("Floats not implemented yet")
                        }
                        Literal::String(_value) => {
                            todo!("Strings not implemented yet")
                        }
                        Literal::Boolean(_value) => {
                            todo!("Booleans not implemented yet")
                        }
                        Literal::Null => {
                            todo!("Null not implemented yet")
                        }
                    }
                },
                Instruction::Add => {
                    let registers = self.stack.take_lasts_reg_used(2);

                    self.builder.build_iadd(registers[0].clone(), AsmValue::Register(registers[1].clone()));
                },
                Instruction::Pop => {
                    let register = &self.stack.take_lasts_reg_used(1)[0];

                    if register != &Register::R1 {
                        self.builder.build_mov(Register::R1, AsmValue::Register(register.clone()));
                    }
                },
                _ => todo!("Instruction not implemented yet")
            }
        }

    }

    pub fn build(self) -> Program<'a> {
        self.builder.build()
    }

}
