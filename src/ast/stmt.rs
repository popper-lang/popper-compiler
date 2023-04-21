use super::expr::Expr;
use crate::lexer::Token;
use std::ops::Range;
use crate::bytecodes::bytecode::Bytecode;
use crate::bytecodes::bytecode::Opcode;
use crate::bytecodes::bytecode::Operand;
use crate::bytecodes::vm::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum StmtType {
    Let {
        name: Token,
        value: Option<Expr>,
        mutable: bool,
        type_: Option<Expr>,
    },
    Block {
        body: Vec<Stmt>,
    },
    Function {
        name: Token,
        args: Vec<String>,
        body: Stmt,
    },
    If {
        cond: Expr,
        then: Stmt,
    },
    IfElse {
        cond: Expr,
        then: Stmt,
        else_: Stmt,
    },
    For {
        name: Token,
        iter: Expr,
        body: Stmt,
    },
    While {
        cond: Expr,
        body: Stmt,
    },
    Match {
        cond: Expr,
        cases: Vec<(Expr, Stmt)>,
    },
    Class {
        name: String,
        methods: Vec<Stmt>,
    },
    Expression {
        expr: Expr,
    },
    Use {
        path: String,
        as_: String,
    },
    Import {
        name: String,
        imports: Vec<String>,
    },
    Impl {
        struct_name: String,
        methods: Vec<Stmt>
    },
    Struct {
        name: String,
        fields: Vec<(String, Expr)>
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Stmt {
    pub(crate) stmt_type: Box<StmtType>,
    pub(crate) extract: Range<usize>,
    pub(crate) body: String,
}

impl Stmt {
    pub fn new(stmt_type: StmtType, extract: Range<usize>, body: String) -> Stmt {
        Stmt {
            stmt_type: Box::new(stmt_type),
            extract,
            body,
        }
    }

    pub fn to_bytecode(&self) -> Bytecode {
        let mut bytecode = Bytecode::new();

        match &*self.stmt_type {
            StmtType::Expression { expr } => {
                let expr = expr.to_bytecode();
                bytecode.instructions.extend(expr.instructions);
            },
            StmtType::If { cond, then } => {
                let cond = cond.to_bytecode();
                let then = then.to_bytecode();
                bytecode.instructions.extend(cond.instructions);
                bytecode.add_instruction(Opcode::If, Some(Operand::Int(then.instructions.len() as i32)));
                bytecode.instructions.extend(then.instructions);
            },
            StmtType::IfElse { cond, then, else_ } => {
                let cond = cond.to_bytecode();
                let then = then.to_bytecode();
                let else_ = else_.to_bytecode();
                bytecode.instructions.extend(cond.instructions);
                bytecode.add_instruction(Opcode::If, Some(Operand::Int((bytecode.instructions.len() + 1) as i32)));
                bytecode.add_instruction(Opcode::Jump, Some(Operand::Int((bytecode.instructions.len() + then.instructions.len()) as i32)));
                bytecode.instructions.extend(then.instructions);
                bytecode.instructions.extend(else_.instructions);
            },
            StmtType::Block { body } => {
                for stmt in body {
                    let stmt = stmt.to_bytecode();
                    bytecode.instructions.extend(stmt.instructions);
                }
            },
            _ => todo!()
        }

        bytecode
    }
}
