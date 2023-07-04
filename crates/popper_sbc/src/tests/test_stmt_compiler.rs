use popper_ast::get_ast_from_json_file;
use popper_ast::visitor::StmtVisitor;
use crate::compiler::SbCompiler;
use crate::instr::Instruction;
use crate::value::Literal;


#[test]
pub fn test_while_stmt() {
    let ast = get_ast_from_json_file("src/tests/assets/test_while_stmt.json");
    let mut compiler = SbCompiler::new();
    for stmt in ast {
        let _ = compiler.visit_stmt(stmt); // ignore errors
    }
    let instructions_expected = vec![
        Instruction::PushLiteral(Literal::Boolean(true)),
        Instruction::JIFIncluded(7),
        Instruction::PushLiteral(Literal::Integer(1)),
        Instruction::PushLiteral(Literal::Integer(2)),
        Instruction::Add,
        Instruction::Pop,
        Instruction::JmpIncluded(2),
        Instruction::Nop
    ];

    assert_eq!(compiler.ir.instructions, instructions_expected);

}

