use popper_ast::ast::{Ident, Let, LangNode, LangNodeId, LangNodeKind};
use popper_ast::layer::Ast;
use popper_ast::token::TokenKind;
use crate::Parser;
use crate::error::Result;
impl Parser {
    pub(crate) fn parse_let_stmt(&mut self) -> Result<LangNodeId> {
        let start = self.expect(TokenKind::KeywordLet)?;
        let identifier = self.expect(TokenKind::Identifier)?;
        self.expect(TokenKind::Eq)?;
        let expr = self.parse_expr()?;
        let end = self.expect(TokenKind::Semicolon)?;

        let span = start.span.merge(end.span);

        let symbol = self.ast.add_symbol(&identifier.value);

        let node = LangNode {
            kind: LangNodeKind::Let(
                Let {
                    name: Ident(symbol),
                    value: expr,
                }
            ),
            span,
        };
        let id = self.ast.add(node);
        Ok(id)
    }
}