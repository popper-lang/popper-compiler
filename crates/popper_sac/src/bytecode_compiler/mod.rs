use std::ops::{Range, RangeFrom};
use popper_asm::builder::{Assembly, Builder, Program};
use popper_sbc::instr::{find_used_variable_in_instrs, Instruction};
use popper_sbc::value::{ByteType, Literal};
use popper_asm::register::Register;
use popper_asm::asm_value::{AsmValue, Immediate};
use crate::stack::{AVAILABLE_ARG_REGISTER, Stack, StackEnv};
use crate::label::Label;
use crate::label::LabelFn;

mod jump;
use jump::Jumper;

type BytecodeProgram = Vec<Instruction>;

/// compiler who compile bytecode instruction to asm-like instruction
#[derive(Clone)]
pub struct Compiler {
    builder: Builder,
    bytecode: BytecodeProgram,
    stack: Stack,
    stack_env: StackEnv,
    labels: Vec<Label>,
    labels_fn: Vec<LabelFn>,
    to_free: Vec<Range<usize>>,
    is_fn: bool,
    ip: usize,
}


impl Compiler {
    pub fn new(bytecode: BytecodeProgram) -> Self {
        Self {
            builder: Builder::new(),
            bytecode,
            stack: Stack::new(),
            stack_env: StackEnv::new(),
            labels: Vec::new(),
            to_free: Vec::new(),
            labels_fn: Vec::new(),
            is_fn: false,
            ip: 0,
        }
    }
    pub fn set_stack_env(&mut self, stack_env: StackEnv) {
        self.stack_env = stack_env;
    }
    pub fn set_stack(&mut self, stack: Stack) {
        self.stack = stack;
    }

    pub fn set_labels(&mut self, labels: Vec<Label>) {
        self.labels = labels;
    }

    pub fn set_labels_fn(&mut self, labels_fn: Vec<LabelFn>) {
        self.labels_fn = labels_fn;
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
                    let name = Jumper::create_name(self.clone());
                    let jumper = Jumper::new(
                        name.clone(),
                        Assembly::Jne(name),
                        instrs
                    );

                    jumper.build_assembler(self, is_included);

                },
                Instruction::Jmp(is_included, instrs) => {
                    let name = Jumper::create_name(self.clone());
                    let jumper = Jumper::new(
                        name.clone(),
                        Assembly::Jmp(name),
                        instrs
                    );

                    jumper.build_assembler(self, is_included);
                }
                Instruction::JIT(is_included, instrs) => {
                    let name = Jumper::create_name(self.clone());
                    let jumper = Jumper::new(
                        name.clone(),
                        Assembly::Je(name),
                        instrs
                    );

                    jumper.build_assembler(self, is_included);
                },

                Instruction::Pop => {
                    let registers = &self.stack.take_lasts_reg_used(1);
                    if !registers.is_empty() {
                        self.builder.build_mov(Register::R1, AsmValue::Register(registers[0].clone()));
                    }
                    if !self.is_fn { self.stack.free_all_registers() } else {}
                },
                Instruction::StoreFn(name, args, ret, body) => {
                    let name = format!("fn_{}", name.str);
                    let mut stack = Stack::from(self.stack.clone());
                    for (reg, arg) in AVAILABLE_ARG_REGISTER.iter().zip(args.clone()) {
                        self.stack_env.push(arg.name.str, reg.clone()) // TODO: handle err
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
                Instruction::Call(ref name, args) => {
                    let name = "fn_".to_owned() + &name.str;

                    let fn_ty = self.labels_fn.iter().find(|x| x.label.label == name.clone()).expect("TODO: handle err");

                    let mut compiler = Compiler::new(args.clone());

                    compiler.set_stack(self.stack.clone());
                    compiler.set_stack_env(self.stack_env.clone());
                    compiler.set_labels(self.labels.clone());
                    compiler.set_labels_fn(self.labels_fn.clone());
                    compiler.is_fn = true;
                    compiler.compile();

                    let regs = compiler.stack.take_lasts_reg_used(args.len());

                    for (reg_to_mov, reg) in AVAILABLE_ARG_REGISTER.iter().zip(regs) {
                        compiler.builder.build_mov(reg_to_mov.clone(), AsmValue::Register(reg));
                    }

                    self.builder = compiler.builder;

                    self.builder.build_call(name.clone());
                    self.stack.push(Register::R1, AsmValue::Register(Register::R1));
                },
                Instruction::Return => {
                    let regs = self.stack.take_lasts_reg_used(1);
                    let reg = &regs[0];

                    self.builder.build_mov(
                        Register::R1,
                        AsmValue::Register(reg.clone())
                    );
                    self.builder.build_ret();

                },

                Instruction::PushVariable(var) => {
                    let register = self.stack_env
                        .get(var.str.as_str())
                        .expect("TODO: handle err"); // TODO:HANDLE ERR
                    self.stack.push(
                        register.clone(),
                        AsmValue::Register(register.clone())
                    );
                },
                Instruction::Store(name) => {
                    let reg = self.stack.give_normal_register().expect("Panic: No register available");
                    let regs = self.stack.take_lasts_reg_used(1);

                    let last_reg = regs.last().unwrap();
                    self.stack_env.push(name.str, reg.clone());
                    self.builder.build_mov(reg, AsmValue::Register(last_reg.clone()))
                }
                Instruction::Nop => {
                    self.builder.build_nop();
                },
                e => todo!("Instruction not implemented yet: {:?}", e)
            }
        }
    }

    pub fn build(self) -> (Program, Vec<(String, Program)>) {
        let mut builder_labels: Vec<(String, Program)> = self.labels.into_iter().map(|x| (x.label, x.program)).collect();
        builder_labels.extend(self.labels_fn.into_iter().map(|d| (d.label.label, d.label.program)).collect::<Vec<(String, Program)>>());
        (self.builder.build(), builder_labels)
    }

    fn build_label(&mut self,  body: Vec<Instruction>, name: String, instr: Assembly, instr_to_add: Vec<Assembly>) {
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
