mod literal;
mod operator;
mod ptr;
mod postfix;
mod struct_instance;

use crate::Parser;
use popper_ast::ast::LangNodeId;

impl Parser {
    pub(crate) fn parse_expr(&mut self) -> crate::error::Result<LangNodeId> {
        self.parse_binary_expr()
    }
}
