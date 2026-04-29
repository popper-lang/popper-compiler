use popper_ast::ast::{LangNode, LangNodeId, LangNodeKind};
use popper_ast::layer::Ast;
use popper_ast::token::TokenKind;
use crate::Parser;
use crate::error::Result;

impl Parser {
    pub(crate) fn parse_comptime_stmt(&mut self) -> Result<LangNodeId> {
        let start = self.expect(TokenKind::KeywordComptime)?;
        let block = self.parse_block()?;
        let end_span = self.ast.get(block).span;

        let span = start.span.merge(end_span);

        let node = LangNode {
            kind: LangNodeKind::Comptime(block),
            span,
        };
        let id = self.ast.add(node);
        Ok(id)
    }
}
