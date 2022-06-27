
mod api;
mod ast;
pub mod errors;
mod expr;
mod std_t;
mod value;
mod vm;
use std::fs;
use ast::Expr;
use errors::Error;
use lalrpop_util::lalrpop_mod;
use value::Value;
use vm::{Vm, Evaluateur};



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

