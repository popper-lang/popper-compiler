use crate::Block;
use crate::Expression;
use crate::Extern;
use crate::External;
use crate::ForStmt;
use crate::Function;
use crate::If;
use crate::IfElse;
use crate::ImportStmt;
use crate::LetStmt;
use crate::Return;
use crate::Span;
use crate::StructStmt;
use crate::While;

#[cfg_attr(feature = "extra-trait", derive(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
pub enum Statement {
    Expression(Expression),
    While(While),
    Block(Block),
    Let(LetStmt),
    If(If),
    IfElse(IfElse),
    Function(Function),
    Return(Return),
    Import(ImportStmt),
    External(External),
    Extern(Extern),
    For(ForStmt),
    Struct(StructStmt),
}

impl Statement {
    pub fn span(&self) -> Span {
        match self {
            Statement::Expression(expr) => expr.span(),
            Statement::While(while_stmt) => while_stmt.span(),
            Statement::Block(block) => block.span(),
            Statement::Let(let_stmt) => let_stmt.span(),
            Statement::If(if_stmt) => if_stmt.span(),
            Statement::IfElse(if_else_stmt) => if_else_stmt.span(),
            Statement::Function(fn_stmt) => fn_stmt.span(),
            Statement::Return(ret_stmt) => ret_stmt.span(),
            Statement::Import(import_stmt) => import_stmt.span(),
            Statement::External(external_stmt) => external_stmt.span(),
            Statement::For(for_stmt) => for_stmt.span(),
            Statement::Struct(struct_stmt) => struct_stmt.span,
            Statement::Extern(extern_stmt) => extern_stmt.span(),
        }
    }
}
