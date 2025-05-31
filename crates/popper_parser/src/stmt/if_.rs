use crate::Parser;
use popper_ast::ast::{LangNode, LangNodeId, LangNodeKind, Span};
use popper_ast::layer::Ast;
use popper_ast::token::TokenKind;

impl Parser {
    pub(crate) fn parse_if_stmt(&mut self) -> crate::error::Result<LangNodeId> {
        let start = self.expect(TokenKind::KeywordIf)?;
        let condition = self.parse_expr()?;
        let then_block = self.parse_block()?;

        let else_block = if self.cursor.peek_token()?.kind == TokenKind::KeywordElse {
            self.cursor.next_token()?;
            let res = self.parse_block()?;
            Some(res)
        } else {
            None
        };
        
        let end = self.cursor.pos();

        let span = Span::new(
            start.span.lo,
            end
        );

        let node = LangNode {
            kind: LangNodeKind::If {
                condition,
                then_branch: then_block,
                else_branch: else_block,
            },
            span,
        };
        Ok(self.ast.add(node))
    }
}