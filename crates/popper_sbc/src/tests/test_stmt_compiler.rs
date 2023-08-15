use popper_ast::get_ast_from_json_file;
use popper_ast::visitor::StmtVisitor;
use crate::compiler::SbCompiler;
use crate::instr::Instruction;
use crate::value::{ByteArg, ByteType, Literal, ByteStr};


#[test]
pub fn test_while_stmt() {
    let ast = get_ast_from_json_file("src/tests/assets/test_while_stmt.json");
    let mut compiler = SbCompiler::new();
    for stmt in ast {
        let _ = compiler.visit_stmt(stmt); // ignore errors
    }
    let instructions_expected = vec![
        Instruction::PushLiteral(Literal::Boolean(true)),
        Instruction::JIT(true, vec![
            Instruction::PushLiteral(Literal::Integer(1)),
            Instruction::PushLiteral(Literal::Integer(2)),
            Instruction::Add,
            Instruction::Pop,
            Instruction::PushLiteral(Literal::Boolean(true))
        ]),
    ];

    assert_eq!(compiler.ir.instructions, instructions_expected);

}

#[test]
pub fn test_function() {
    let ast = get_ast_from_json_file("src/tests/assets/test_function.json");

    let mut compiler = SbCompiler::new();
    for stmt in ast {
        let _ = compiler.visit_stmt(stmt); // ignore errors
    }

    let instructions_expected = vec![
        Instruction::StoreFn(
            vec![
                ByteArg::new(
                    ByteStr::from_str("name"),
                    ByteType::Str
                )
            ],
            Box::new(
                ByteType::Int
            ),
            vec![
                Instruction::PushLiteral(
                    Literal::Integer(4)
                ),
                Instruction::PushLiteral(
                    Literal::Integer(5)
                ),
                Instruction::Add,
                Instruction::Pop
            ]
        )
    ];

    assert_eq!(compiler.ir.instructions, instructions_expected);
}

