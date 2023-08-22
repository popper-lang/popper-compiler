use popper_asm::builder::Program;
use popper_sbc::value::ByteType;
use crate::bytecode_compiler::Compiler;
use crate::stack::Stack;

/// reprensation of a asm label (
/// ```asm
///hello:
///     mov eax, 1
/// ```
/// it is a examble ) in a bytecode fmt (Instruction is a Bytecode instruction)
#[derive(Clone, Debug)]
pub struct Label {
    pub label: String,
    pub program: Program
}

impl Label {
    pub fn new(label: String, program: Program) -> Self {
        Self {
            label,
            program
        }
    }
}

#[derive(Clone, Debug)]
pub struct LabelFn {
    pub label: Label,
    pub fn_type: (Vec<ByteType>, ByteType)
}

impl LabelFn {
    pub fn new(label: Label, fn_type: (Vec<ByteType>, ByteType)) -> Self {
        Self {
            label,
            fn_type
        }
    }
}