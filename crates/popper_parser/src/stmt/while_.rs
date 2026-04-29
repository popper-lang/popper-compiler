use popper_ast::ast::{LangNode, LangNodeKind, Span};
use popper_ast::layer::Ast;
use popper_ast::token::TokenKind;
use crate::Parser;

impl Parser {
    pub(crate) fn parse_while_stmt(&mut self) -> crate::error::Result<popper_ast::ast::LangNodeId> {
        let start = self.cursor.pos();
        self.expect(TokenKind::KeywordWhile)?;
        self.disallow_struct_literal = true;
        let condition = self.parse_expr()?;
        self.disallow_struct_literal = false;
        let body = self.parse_block()?;
        let end = self.cursor.pos();

        let node = LangNode {
            kind: LangNodeKind::While { condition, body },
            span: Span::new(start, end),
        };

        Ok(self.ast.add(node))
    }
}