use crate::Expression;
use crate::While;
use crate::Block;
use crate::LetStmt;
use crate::If;
use crate::IfElse;


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
}
