use popper_parser::parser::parse;
use popper_ast::Statement;

use popper_semantic_analyzer::analyze;

use popper_error::generate_color;

use popper_sbc::compile_to_bytecode
    as compile_to_bytecode_sbc;
use popper_sbc::instr::Instruction;

use popper_sac::bytecode_compiler::Compiler;
use popper_asm::builder::{Builder, Program};
use popper_asm::x86_builder::X86Builder;


///
/// get_ast is used to get ast from input
/// # Arguments
/// * `input` - input string
/// * `file` - file name
///
/// return: `Option<Vec<Statement>>`
pub fn get_ast(input: &str, file: &str) -> Option<Vec<Statement>> {
   parse(input, file)
}


///
/// check_program is used to check program
/// # Arguments
/// * `ast` - ast
/// * `source` - source code
/// * `file_name` - file name
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

///
/// compile_to_bytecode is used to compile ast to bytecode
/// # Arguments
/// * `ast` - ast
///
/// return: `Vec<Instruction>`
///
pub fn compile_to_bytecode(ast: Vec<Statement>) -> Vec<Instruction> {
    compile_to_bytecode_sbc(ast)
}

/// compile_to_asm is used to compile bytecode to asm
///
/// # Arguments
/// * `instructions` - Vec<Instruction>
///
/// return: `(Program<'a>, Vec<(String, Program<'a>)>)`
///
/// `Program<'a>` is used to store asm
/// `Vec<(String, Program<'a>)>` are all the labels of asm: `Vec<(<name of label>, <content of label>)>`
pub fn compile_to_asm<'a>(instructions: Vec<Instruction>) -> (Program, Vec<(String, Program)>) {
    let mut compiler = Compiler::new(instructions).clone();

    compiler.compile();
    compiler.build()

}

/// compile_to_binary is used to compile asm to binary
///
/// # Arguments
///
/// * `program` - Program
/// * `labels` - Vec<(String, Program<'a>)>
///
/// return: asm string
pub fn compile_to_binary(program: Program, labels: Vec<(String, Program)>) -> String {
    let mut builder = Builder::new();
    builder.program = program;
    builder.labels = labels;
    let mut x86builder = X86Builder::new(builder);

    x86builder.compile();

    let asm = x86builder.build();

    asm
}


/// popper_compile is used to compile popper code to asm
///
/// # Arguments
///
/// * `input` - input string
/// * `file_name` - file name
///
/// return: asm string
///
pub fn popper_compile(input: &str, file_name: &str) -> String {
    let ast = get_ast(input, file_name);
    let ast = match ast {
        Some(ast) => ast,
        None => {
            println!("Error parsing file");
            return String::new();
        }
    };
    if check_program(ast.clone(), input, file_name) {
        let ir =  compile_to_bytecode(ast);

        let program = compile_to_asm(ir);

        compile_to_binary(program.0, program.1)
    } else {
        String::new()
    }
}
