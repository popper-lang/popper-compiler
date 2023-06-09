use crate::Expression;
use crate::While;
use crate::Block;

#[derive(Debug, Clone)]
pub enum Statement {
    Expression(Expression),
    While(While),
    Block(Block),
}
