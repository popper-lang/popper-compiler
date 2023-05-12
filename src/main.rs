use popper::bytecodes::util::{read_bytecode_from_file, decompile};
use popper::bytecodes::debug::debug_bytecode;
use popper::bytecodes::vm::Vm;
use popper::parser::Parser;
use popper::lexer::Lexer;

fn main() {

    let body = r#"
    if false {
    2 + 3
    } else {
    1 + 48
    }
    "#.to_string();
    let mut lexer = Lexer::new(body.clone());
    let mut parser = Parser::new(lexer.scan_token(), body);
    let ast = parser.parse();
    let mut vm = Vm::new();
    for stmt in ast {
        let bytecode = stmt.to_bytecode();
        debug_bytecode(&bytecode);

        dbg!(vm.run(&bytecode));
        println!("Result: {:?}", vm.stack);
    }


}
