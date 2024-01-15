use popper_compiler::compile_to_llvm;
use popper_compiler::get_ast;
use popper_compiler::check_program;


fn main() {
    let out = std::path::read_to_string("../examples/helloworld.popper").expect("Cannot read file");

    let file_name = "helloworld.popper";

    let ast = get_ast(out, file_name).unwrap();
    let is_ok = check_program(ast.clone(), out, file_name);
    if is_ok {
        let llvm_code = compile_to_llvm(ast, file_name);
        println!("{}", llvm_code);
    }


}
