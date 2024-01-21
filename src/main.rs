use popper_compiler::compile_to_mir;
use popper_compiler::get_ast;
use popper_compiler::check_program;
use std::env::args;

fn main() {
    let arg = args().nth(1).expect("No file provided");
    let file = arg.as_str();

    let out = std::fs::read_to_string(file).expect("Cannot read file");

    let ast = get_ast(out.as_str(), file).unwrap();
    let is_ok = check_program(ast.clone(), out.as_str(), file);
    if is_ok {
        compile_to_mir(ast, file);

    } else {
        println!("Program is not ok");
    }


}
