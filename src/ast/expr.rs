use crate::lexer::{Token, TokenType};
use std::ops::Range;
use crate::bytecodes::bytecode::{Bytecode, StrPtr};
use crate::bytecodes::bytecode::Opcode;
use crate::bytecodes::bytecode::Operand;


#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum ExprType {
    BinOp { left: Expr, op: Token, right: Expr },
    Call { name: Expr, args: Vec<Expr> },
    Get { name: Expr, attr: Expr },
    Grouping { group: Expr },
    Index { name: Expr, index: Expr },
    IOp { name: Token, op: Token, value: Expr },
    List { elems: Vec<Expr> },
    Literal { literal: LiteralType },
    Range { start: Expr, end: Expr },
    Assign { name: Token, value: Expr },
    To { value: Expr, type_: Expr },
    UnaryOp { op: Token, operand: Expr },
    Ident { ident: Token },
    Type { type_: Token },
    CmpOp { left: Expr, op: Token, right: Expr },
    NsGet { name: Expr, attr: Expr },
    InitStruct { name: Expr, fields: Vec<(Expr, Expr)> },
    Eof,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Expr {
    pub(crate) expr_type: Box<ExprType>,
    pub(crate) extract: Range<usize>,
    pub(crate) body: String,
}

impl Expr {
    pub fn new(expr_type: Box<ExprType>, extract: Range<usize>, body: String) -> Expr {
        Expr {
            expr_type,
            extract,
            body,
        }
    }

    pub fn to_bytecode(&self) -> Bytecode {
        let mut bytecode = Bytecode::new();
        match &*self.expr_type {
            ExprType::BinOp { left, op, right } => {
                let left = left.to_bytecode();
                let right = right.to_bytecode();
                bytecode.instructions.extend(left.instructions);
                bytecode.instructions.extend(right.instructions);
                match op.token_type {
                    TokenType::ADD => bytecode.add_instruction(Opcode::Add, None),
                    TokenType::SUB => bytecode.add_instruction(Opcode::Subtract, None),
                    TokenType::MUL => bytecode.add_instruction(Opcode::Multiply, None),
                    TokenType::DIV => bytecode.add_instruction(Opcode::Divide, None),

                    _ => todo!()
                }

            },
            ExprType::Literal { literal } => {
                match literal {
                    LiteralType::Number(i) => bytecode.add_instruction(Opcode::LoadConst, Some(Operand::Int(*i))),
                    LiteralType::Bool(b) => bytecode.add_instruction(Opcode::LoadConst, Some(Operand::Bool(*b))),
                    LiteralType::String(s) => {
                        let str_ptr = StrPtr {
                            ptr: s.as_ptr(),
                            len: s.len(),
                        };

                        bytecode.add_instruction(Opcode::LoadConst, Some(Operand::Str(str_ptr)))
                    },
                    _ => todo!()
                }
            },
            ExprType::Ident { ident } => {
                bytecode.add_instruction(Opcode::LoadVar, Some(Operand::Str(StrPtr {
                    ptr: ident.lexeme.as_ptr(),
                    len: ident.lexeme.len(),
                })))
            },
            ExprType::Call { name, args} => {
                let name = name.to_bytecode();

                bytecode.instructions.extend(name.instructions);

                for arg in args {
                    let arg = arg.to_bytecode();
                    bytecode.instructions.extend(arg.instructions);
                }

                bytecode.add_instruction(Opcode::Call, Some(Operand::Int(args.len() as i32)));
            }
            ExprType::Eof => bytecode.add_instruction(Opcode::EndOfProgram, None),

            _ => todo!()
        }
        bytecode
    }


}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LiteralType {
    Number(i32),
    Bool(bool),
    String(String),
}
