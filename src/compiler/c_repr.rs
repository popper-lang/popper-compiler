
use crate::ast::expr::{Expr, ExprType, LiteralType};
use crate::ast::stmt::{Stmt, StmtType};
use crate::interpreter::Interpreter;

pub fn interpret(stmts: Vec<Stmt>) -> Interpreter {
    let mut interpreter = Interpreter::new();
    for stmt in stmts {
        stmt.accept(&mut interpreter);
    }
    return interpreter;
}

