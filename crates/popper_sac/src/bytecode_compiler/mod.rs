use std::ops::{Range, RangeFrom};
use popper_asm::builder::{Assembly, Builder, Program};
use popper_sbc::instr::{find_used_variable_in_instrs, Instruction};
use popper_sbc::value::{ByteType, Literal};
use popper_asm::register::Register;
use popper_asm::asm_value::{AsmValue, Immediate};
use crate::stack::{Stack, StackEnv};
use crate::label::Label;
use crate::label::LabelFn;



type BytecodeProgram = Vec<Instruction>;

/// compiler who compile bytecode instruction to asm-like instruction
#[derive(Clone)]
pub struct Compiler<'a> {
    builder: Builder<'a>,
    bytecode: BytecodeProgram,
    stack: Stack,
    stack_env: StackEnv,
    labels: Vec<Label<'a>>,
    labels_fn: Vec<LabelFn<'a>>,
    to_free: Vec<Range<usize>>,
    ip: usize,
}


impl<'a> Compiler<'a> {
    pub fn new(bytecode: BytecodeProgram) -> Self {
        Self {
            builder: Builder::new(),
            bytecode,
            stack: Stack::new(),
            stack_env: StackEnv::new(),
            labels: Vec::new(),
            to_free: Vec::new(),
            labels_fn: Vec::new(),
            ip: 0,
        }
    }
    pub fn set_stack_env(&mut self, stack_env: StackEnv) {
        self.stack_env = stack_env;
    }
    pub fn set_stack(&mut self, stack: Stack) {
        self.stack = stack;
    }

    pub fn set_labels(&mut self, labels: Vec<Label<'a>>) {
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
                            let register = match self.stack.give_normal_register() {
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
                Instruction::JIF(is_included, instrs) => {
                    let name = "label".to_string() + self.labels.len().to_string().as_str();
                    let instr = Assembly::Jne(name.clone());
                    let instr_to_add = if is_included {
                        vec![instr.clone()]
                    } else {
                        vec![]
                    };
                    self.build_label(instrs, name.clone(), instr, instr_to_add);

                },
                Instruction::Jmp(is_included, instrs) => {
                    let name = "label".to_string() + self.labels.len().to_string().as_str();
                    let instr = Assembly::Jmp(name.clone());
                    let instr_to_add = if is_included {
                        vec![instr.clone()]
                    } else {
                        vec![]
                    };
                    self.build_label(instrs, name.clone(), instr, instr_to_add);
                }
                Instruction::JIT(is_included, instrs) => {
                    let name = "label".to_string() + self.labels.len().to_string().as_str();
                    let instr = Assembly::Je(name.clone());
                    let instr_to_add = if is_included {
                        vec![instr.clone()]
                    } else {
                        vec![]
                    };
                    self.build_label(instrs, name.clone(), instr, instr_to_add);
                },

                Instruction::Pop => {
                    let register = &self.stack.take_lasts_reg_used(1)[0];

                    self.builder.build_mov(Register::R1, AsmValue::Register(register.clone()));

                    self.stack.free_all_registers();
                },
                Instruction::StoreFn(name, args, ret, body) => {
                    let name = format!("fn_{}", name.str);
                    let mut stack = Stack::from(self.stack.clone());
                    for arg in args.clone() {
                        self.stack_env.push(arg.name.str, stack.give_arg_register().expect("TODO: handle err")) // TODO: handle err
                    }

                    let body = {
                        let mut compiler = Compiler::new(body);
                        compiler.set_labels(self.labels.clone());
                        compiler.set_stack(stack);
                        compiler.set_stack_env(self.stack_env.clone());
                        compiler.compile();
                        compiler.build().0
                    };

                    let fn_ty : (Vec<ByteType>, ByteType) = (args.iter().map(|x| x.ty.clone()).collect(),  *ret);
                    let label = Label::new(name, body);
                    self.labels_fn.push(LabelFn::new(label, fn_ty));

                }

                Instruction::PushVariable(var) => {
                    let register = self.stack_env.get(var.str.as_str()).expect("TODO: handle err"); // TODO:HANDLE ERR
                    self.stack.push(register.clone(), AsmValue::Register(register.clone()));
                },
                Instruction::Nop => {
                    self.builder.build_nop();
                },
                e => todo!("Instruction not implemented yet: {:?}", e)
            }
        }
    }

    pub fn build(self) -> (Program<'a>, Vec<(String, Program<'a>)>) {
        let mut builder_labels: Vec<(String, Program<'a>)> = self.labels.into_iter().map(|x| (x.label, x.program)).collect();
        builder_labels.extend(self.labels_fn.into_iter().map(|d| (d.label.label, d.label.program)).collect::<Vec<(String, Program<'a>)>>());
        (self.builder.build(), builder_labels)
    }

    fn build_label(&mut self,  body: Vec<Instruction>, name: String, instr: Assembly<'a>, instr_to_add: Vec<Assembly<'a>>) {
        if self.labels.iter().filter(|x| x.label == name ).count()  != 0 {
            self.builder.push(instr);
            return;
        }

        let program = {
            let mut compiler = Compiler::new(body);
            compiler.set_labels(self.labels.clone());
            compiler.set_stack(self.stack.clone());
            compiler.compile();
            for instr in instr_to_add {
                compiler.builder.push(instr);
            }
            compiler.build().0
        };
        self.builder.push(instr);

        self.labels.push(
            Label::new(name, program)
        )
    }

}
