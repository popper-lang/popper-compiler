pub mod ast;
pub mod builtin_function;
pub mod errors;
pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod value;
pub mod c_compiler;
pub mod arduino_rs;


use crate::ast::stmt::Stmt;

use interpreter::resolver::Resolver;
use interpreter::Interpreter;
use lexer::{Lexer, Token};
use parser::Parser;
use std::fs;

use value::Object;
use crate::value::litteral::none;

fn read_file(filename: &str) -> String {
    let content = fs::read_to_string(filename).expect("file not found");
    content
}

pub fn tokenize(string: String) -> Vec<Token> {
    let mut lexer = Lexer::new(string);
    lexer.scan_token()
}

pub fn parse(tokens: Vec<Token>, extract: String, file: String) -> Vec<Stmt> {
    let mut parser = Parser::new(tokens, extract, file);
    parser.parse()
}

pub fn interpret(stmts: Vec<Stmt>) {
    println!("{:#?}", stmts);
    let mut interpreter = Interpreter::new();
    //resolve(stmts.clone(), interpreter.clone());
    let mut value: Object = none();
    for stmt in stmts {
        //println!("{:?}", stmt.clone());
        /*println!(
            "expr: {}",
            Box::new(stmt.clone().body[stmt.clone().extract].to_string())
        );*/
        value = stmt.accept(&mut interpreter);
    }
    println!("{:?}", value)
}

pub fn resolve(stmts: Vec<Stmt>, interpreter: Interpreter) {
    let mut resolve = Resolver::new(interpreter);
    resolve.resolve(stmts);
}

pub fn execute(string: &str, file: Option<&str>) {
    let tokens = tokenize(string.to_string());
    let stmts = if let Some(file) = file {
        parse(tokens, string.to_string(), file.to_string())
    } else {
        parse(tokens, string.to_string(), "__main__".to_string())
    };

    interpret(stmts);
}

pub fn execute_file(filename: &str) {
    let content = read_file(filename);
    execute(content.as_str(), Some(filename));
}
