use popper_ast::ast::{Ident, Const, LangNode, LangNodeId, LangNodeKind};
use popper_ast::layer::Ast;
use popper_ast::token::TokenKind;
use crate::Parser;
use crate::error::Result;

impl Parser {
    pub(crate) fn parse_const_stmt(&mut self) -> Result<LangNodeId> {
        let start = self.expect(TokenKind::KeywordConst)?;
        let identifier = self.expect(TokenKind::Identifier)?;
        
        let mut ty = None;
        if self.match_token(TokenKind::Colon) {
            ty = Some(self.parse_ty()?);
        }

        self.expect(TokenKind::Assign)?;
        let expr = self.parse_expr()?;
        let end = self.expect(TokenKind::Semicolon)?;

        let span = start.span.merge(end.span);

        let symbol = self.ast.add_symbol(&identifier.value);

        let node = LangNode {
            kind: LangNodeKind::Const(
                Const {
                    name: Ident(symbol),
                    value: expr,
                    ty,
                }
            ),
            span,
        };
        let id = self.ast.add(node);
        Ok(id)
    }
}
