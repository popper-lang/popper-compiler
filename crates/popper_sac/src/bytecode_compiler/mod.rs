use popper_asm::builder::{Assembly, Builder, Program};
use popper_sbc::instr::Instruction;
use popper_sbc::value::Literal;
use popper_asm::register::Register;
use popper_asm::asm_value::{AsmValue, Immediate};
use crate::stack::Stack;
use crate::label::Label;

type BytecodeProgram = Vec<Instruction>;


#[derive(Clone)]
pub struct Compiler<'a> {
    builder: Builder<'a>,
    bytecode: BytecodeProgram,
    stack: Stack,
    labels: Vec<Label>,
    ip: usize,
}


impl<'a> Compiler<'a> {
    pub fn new(bytecode: BytecodeProgram) -> Self {
        Self {
            builder: Builder::new(),
            bytecode,
            stack: Stack::new(),
            labels: Vec::new(),
            ip: 0,
        }
    }

    pub fn set_stack(&mut self, stack: Stack) {
        self.stack = stack;
    }

    pub fn set_labels(&mut self, labels: Vec<Label>) {
        self.labels = labels;
    }

    pub fn compile(&mut self)  {
        for instr in self.bytecode.clone() {
            self.ip += 1;
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
                        Literal::Boolean(value) => {
                            if value {
                                let value = AsmValue::Immediate(Immediate::U32(1));
                                self.builder.build_cmp(value.clone(), value);
                            } else {
                                let value = AsmValue::Immediate(Immediate::U32(0));
                                let value1 = AsmValue::Immediate(Immediate::U32(1));
                                self.builder.build_cmp(value, value1);
                            }
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
                Instruction::Sub => {
                    let registers = self.stack.take_lasts_reg_used(2);

                    self.builder.build_isub(registers[0].clone(), AsmValue::Register(registers[1].clone()));
                },
                Instruction::Mul => {
                    let registers = self.stack.take_lasts_reg_used(2);
                    self.builder.build_mov(Register::R1, AsmValue::Register(registers[0].clone()));
                    self.builder.build_mul(registers[1].clone());
                    self.builder.build_mov(registers[1].clone(), AsmValue::Register(Register::R1));
                    self.stack.free_register(registers[0].clone());
                },
                Instruction::JIFIncluded(x) => {
                    let name = "label".to_string() + &x.to_string();
                    if self.labels.iter().filter(|x| x.label == name ).count()  != 0 {
                        self.builder.build_jne(name.clone());
                        continue;
                    }
                    let label = if x < self.ip {
                        let l = Label::new(
                            name.clone(),
                            self.bytecode[x..self.ip-1].to_vec()
                        );
                        self.builder.program.drain(x..self.ip-1);
                        l
                    } else {
                        let l = if x <= self.bytecode.len() {
                            Label::new(
                                name.clone(),
                                self.bytecode[x..].to_vec()
                            )
                        } else {
                            Label::new(
                                name.clone(),
                                self.bytecode[x-1..].to_vec()
                            )
                        };
                        if x <= self.bytecode.len() {
                            self.bytecode.drain(x..);
                        } else {
                            self.bytecode.drain(x - 1..);
                        }
                        l
                    };


                    self.labels.push(label.clone());

                    self.builder.build_jne(name.clone());
                    self.builder.build_label(name, label.assembly(self.stack.clone(), self.labels.clone()).0);

                },
                Instruction::JmpIncluded(x) => {
                    let name = "label".to_string() + &x.to_string();
                    let label = if x < self.ip {
                        let l = Label::new(
                            name.clone(),
                            self.bytecode[x..self.ip-1].to_vec()
                        );
                        self.builder.program.drain(x..self.ip-1);
                        l
                    } else {
                        let l = if x <= self.bytecode.len() {
                            Label::new(
                                name.clone(),
                                self.bytecode[x..].to_vec()
                            )
                        } else {
                            Label::new(
                                name.clone(),
                                self.bytecode[x-1..].to_vec()
                            )
                        };
                        if x <= self.bytecode.len() {
                            self.bytecode.drain(x..);
                        } else {
                            self.bytecode.drain(x - 1..);
                        }
                        l
                    };
                    let mut res = label.clone().assembly(self.stack.clone(), self.labels.clone()).0;

                    res.push(Assembly::Jmp(name.clone()));

                    self.labels.push(label.clone());
                    self.builder.build_jmp(name.clone());

                    self.builder.build_label(name.clone(), res);


                },
                Instruction::Pop => {
                    let register = &self.stack.take_lasts_reg_used(1)[0];

                    self.builder.build_mov(Register::R1, AsmValue::Register(register.clone()));

                    self.stack.free_all_registers();
                },
                Instruction::Nop => {
                    self.builder.build_nop();
                },
                e => todo!("Instruction not implemented yet: {:?}", e)
            }
        }

    }

    pub fn build(self) -> (Program<'a>, Vec<(String, Program<'a>)>) {
        (self.builder.build(), self.builder.labels)
    }

}
