use crate::Expression;
use crate::While;
use crate::Block;

#[cfg_attr(feature = "extra-trait", derive(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
pub enum Statement {
    Expression(Expression),
    While(While),
    Block(Block),
}
