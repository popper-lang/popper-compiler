mod literal;
mod operator;

use popper_ast::ast::LangNodeId;
use crate::Parser;

impl Parser {
    pub(crate) fn parse_expr(&mut self) -> crate::error::Result<LangNodeId> {
        self.parse_binary_expr()
    }
}