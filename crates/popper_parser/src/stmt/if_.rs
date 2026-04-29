use crate::Parser;
use popper_ast::ast::{LangNode, LangNodeId, LangNodeKind, Span};
use popper_ast::layer::Ast;
use popper_ast::token::TokenKind;

impl Parser {
    pub(crate) fn parse_if_stmt(&mut self) -> crate::error::Result<LangNodeId> {
        let start = self.expect(TokenKind::KeywordIf)?;
        self.disallow_struct_literal = true;
        let condition = self.parse_expr()?;
        self.disallow_struct_literal = false;
        let then_block = self.parse_block()?;

        let else_block = if self.cursor.peek_token()?.kind == TokenKind::KeywordElse {
            self.cursor.next_token()?;
            if self.cursor.peek_token()?.kind == TokenKind::KeywordIf {
                Some(self.parse_if_stmt()?)
            } else {
                let res = self.parse_block()?;
                Some(res)
            }
        } else {
            None
        };

        let end = self.cursor.pos();

        let span = Span::new(start.span.lo, end);

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
