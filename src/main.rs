
use pest::Parser;
use popper::parser::ExprParser;
use popper::parser::build_ast;
use popper::vm::Evaluateur;
use popper::vm::Vm;


fn main() {
   
    let mut result = ExprParser::parse(popper::parser::Rule::statement,"\"a\".* 5");
    
    match result {
        Ok(ref mut e) => {
            for rule in e {
                println!("{:#?}", match build_ast(rule) {
                    Ok(ast) => {
                        println!("{:#?}", ast);
                        ast.eval(&mut Vm::new())
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