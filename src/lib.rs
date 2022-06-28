
extern crate pest;
#[macro_use]
extern crate pest_derive;
pub mod parser;
pub mod ast;
pub mod expr;
pub mod value;
pub mod vm;
pub mod errors;
pub mod std_t;
pub mod api;


/*

lalrpop_mod!(pub popper);

pub fn get_ast_from_string(string: &str) -> Result<Expr, String> {
    match popper::ExprsParser::new().parse(string) {
        Ok(e) => Ok(e),
        Err(d) => Err(format!("invalid syntax: {:#?}", d))
    }
}

pub fn get_ast_from_file(path: &str) -> Result<Expr, String> {
    let content = fs::read_to_string(path).expect("a error when reading the file");
    
    get_ast_from_string(content.as_str())
}

pub fn eval_expr(expr: Expr) -> Result<Value, Error> {
    let mut vm = Vm::new();
    expr.eval(&mut vm)
}

pub fn execute_string(string: &str) -> Result<Value, Error> {
    eval_expr(match get_ast_from_string(string) {
        Ok(e) => e,
        Err(d) => return Err(Error::SyntaxError(d))
    })
}

pub fn execute_file(path: &str) -> Result<Value, Error> {
    eval_expr(match get_ast_from_file(path) {
        Ok(e) => e,
        Err(e) => return Err(Error::SyntaxError(e))
    })
}
*/
