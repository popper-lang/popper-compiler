use crate::builder::Assembly;
use crate::builder::Program;
use crate::x86_builder::X86Builder;
use crate::register::Register;
use crate::asm_value::{AsmValue, Immediate, Memory};


#[test]
fn x86_builder_compiler_add() {

    let mut program: Program = vec![
        Assembly::Mov(
            Register::R1,
            Box::new(AsmValue::Immediate(Immediate::U32(2)))
        ),
        Assembly::Mov(
            Register::R2,
            Box::new(AsmValue::Immediate(Immediate::U32(2)))
        ),
        Assembly::Add(
            Register::R1,
            Box::new(AsmValue::Register(Register::R2)),
            Register::R1
        ),
        Assembly::Ret
    ];

    let mut x86_builder = X86Builder::new(program);
    x86_builder.compile();
    assert_eq!(x86_builder.x86_asm, "mov eax, 2\nmov ebx, 2\nadd eax, ebx\nret\n");

}