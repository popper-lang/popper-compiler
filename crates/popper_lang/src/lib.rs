use popper_parser::parser::popper::FileParser;
use popper_ast::Statement;
use popper_semantic_analyzer::analyze;
use popper_common::error::generate_color;

use popper_sbc::compile_to_bytecode as compile_to_bytecode_sbc;
use popper_sbc::ir_sb::SbcIr;
use popper_asm::builder::{Program};
use popper_asm::x86_builder::X86Builder;
use popper_sac::bytecode_compiler::Compiler;



pub fn get_ast(input: &str) -> Vec<Statement> {
    FileParser::new().parse(input).unwrap()
}

pub fn check_program(ast: Vec<Statement>, source: &str, file_name: &str) -> bool {
    let res = analyze(ast);

    let mut errs = Vec::new();
    for r in res  {
        match r {
            Ok(_) => {},
            Err(err) => {
                errs.push(err)
            }
        }
    }

    if errs.len() == 0 {
        true
    } else {
        errs
            .into_iter()
            .for_each(|x| {
                x.report(
                    generate_color(),
                    source,
                    file_name,
                )
            })
        ;

        false
    }

}

pub fn compile_to_bytecode(ast: Vec<Statement>) -> SbcIr {
    compile_to_bytecode_sbc(ast)
}


pub fn compile_to_asm<'a>(ir: SbcIr) -> Program<'a> {
    let mut compiler = Compiler::new(ir.instructions).clone();

    compiler.compile();
    compiler.build()

}

pub fn compile_to_binary<'a>(program: Program<'a>) -> String {
    let mut builder = X86Builder::new(program);

    builder.compile();

    let asm = builder.build();

    asm
}

pub fn popper_compile(input: &str, file_name: &str) -> String {
    let ast = get_ast(input);

    if check_program(ast.clone(), input, file_name) {
        let ir = compile_to_bytecode(ast);

        let program = compile_to_asm(ir);

        compile_to_binary(program)
    } else {
        String::new()
    }
}
