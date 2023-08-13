use popper_asm::builder::Program;
use crate::bytecode_compiler::Compiler;
use crate::stack::Stack;

/// reprensation of a asm label (
/// ```asm
///hello:
///     mov eax, 1
/// ```
/// it is a examble ) in a bytecode fmt (Instruction is a Bytecode instruction)
#[derive(Clone, Debug)]
pub struct Label<'a> {
    pub label: String,
    pub program: Program<'a>
}

impl<'a> Label<'a> {
    pub fn new(label: String, program: Program<'a>) -> Self {
        Self {
            label,
            program
        }
    }
}