use popper_asm::builder::Program;
use popper_sbc::instr::Instruction;
use crate::bytecode_compiler::Compiler;
use crate::stack::Stack;

#[derive(Clone, Debug)]
pub struct Label {
    pub label: String,
    pub program: Vec<Instruction>
}

impl Label {
    pub fn new(label: String, program: Vec<Instruction>) -> Self {
        Self {
            label,
            program
        }
    }

    pub fn assembly<'a>(self, stack: Stack, labels: Vec<Self>) -> (Program<'a>, Vec<(String, Program<'a>)>) {
        let mut compiler = Compiler::new(self.program.clone());
        compiler.set_stack(stack);
        compiler.set_labels(labels);
        compiler.compile();
        compiler.build()


    }
}