use popper_parser::parser::parse;
use popper_ast::Statement;
use popper_semantic_analyzer::analyze;
use popper_error::generate_color;
use popper_llvm::Context;
use popper_llvm::compiler::llvm_env::LLVMEnv;

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

pub fn compile_to_llvm(ast: Vec<Statement>, file_name: &str) -> String {
    let context = Context::create();
    let mut compiler = popper_llvm::compiler::LLVMCompiler::new(&context, LLVMEnv::new(), file_name);
    compiler.register_builtin();
    let res = compiler.compile(ast);

    return res

}