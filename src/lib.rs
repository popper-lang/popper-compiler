#![feature(type_alias_impl_trait)]

pub mod lexer;
pub mod ast;
pub mod parser;
pub mod errors;
pub mod value;
pub mod interpreter;
pub mod builtin_function;

use std::fs;
use lexer::{Token, Lexer};
use ast::stmt::Stmt;
use parser::Parser;
use interpreter::Interpreter;
use interpreter::resolver::Resolver;

fn read_file(filename: &str) -> String {
    let content = fs::read_to_string(filename).expect("file not found");
    content
}

pub fn tokenize(string: String) -> Vec<Token> {
    let mut lexer = Lexer::new(string);
    lexer.scan_token()
}

pub fn parse(tokens: Vec<Token>) -> Stmt{
    let mut parser = Parser::new(tokens);
    parser.parse()
}

pub fn interpret(stmts: Stmt) {
    let mut interpreter = Interpreter::new();
    resolve(stmts.clone(), interpreter.clone());
    stmts.accept(&mut interpreter);
}

pub fn resolve(stmt: Stmt, interpreter: Interpreter) {
    let mut resolve = Resolver::new(interpreter);
    stmt.accept(&mut resolve);
}

pub fn execute(string: &str) {
    let tokens = tokenize(string.to_string());
    let stmts = parse(tokens);

    interpret(stmts);
}

pub fn execute_file(filename: &str) {
    let content = read_file(filename);
    execute(content.as_str());
}

