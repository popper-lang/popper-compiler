
use pest::Parser;
use popper::parser::ExprParser;
use popper::parser::build_ast;
use popper::vm::Evaluateur;
use popper::vm::Vm;
use std::fs;

fn main() {
    let content = fs::read_to_string("src/example/presentation-1.pop").unwrap();
    let mut result = ExprParser::parse(popper::parser::Rule::program, &content);
    let mut vm = Vm::new();
    println!("{:#?}", result);
    match result {
        Ok(ref mut e) => {
            for rule in e {
                println!("break");
                println!("{:#?}", match build_ast(rule) {
                    Ok(ast) => {
                        println!("{:#?}", ast);
                        ast.eval(&mut vm)
                    }
                    Err(e) => {
                        println!("{}", e);
                        return;
                    }
                });
            }

        },
        Err(e) => println!("{}", e)
    }
}