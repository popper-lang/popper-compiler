#![feature(type_alias_impl_trait)]

pub mod lexer;
pub mod ast;
pub mod parser;
pub mod errors;
pub mod value;
pub mod interpreter;
pub mod builtin_function;

use std::fs;
use std::rc::Rc;
use lexer::{Token, Lexer};
use ast::stmt::Stmt;
use parser::Parser;
use interpreter::Interpreter;
use interpreter::resolver::Resolver;
use value::Object;

fn read_file(filename: &str) -> String {
    let content = fs::read_to_string(filename).expect("file not found");
    content
}

pub fn tokenize(string: String) -> Vec<Token> {
    let mut lexer = Lexer::new(string);
    lexer.scan_token()
}

pub fn parse(tokens: Vec<Token>) -> Vec<Stmt> {
    let mut parser = Parser::new(tokens);
    parser.parse()
}

pub fn interpret(stmts: Vec<Stmt>) {
    let mut interpreter = Interpreter::new();
    resolve(stmts.clone(), interpreter.clone());
    let mut value: Rc<dyn Object> = Rc::new(());
    for stmt in stmts {
        value = stmt.accept(&mut interpreter);
    }
    println!("value {:?}", value)
}

pub fn resolve(stmts: Vec<Stmt>, interpreter: Interpreter) {
    let mut resolve = Resolver::new(interpreter);
    resolve.resolve(stmts);

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

