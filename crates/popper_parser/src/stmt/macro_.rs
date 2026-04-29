use crate::error::Result;
use crate::Parser;
use popper_ast::ast::{Ident, LangNode, LangNodeId, LangNodeKind, MacroDef, ParamDef};
use popper_ast::layer::Ast;
use popper_ast::token::TokenKind;

impl Parser {
    pub(crate) fn parse_macro_stmt(&mut self) -> Result<LangNodeId> {
        let start = self.expect(TokenKind::KeywordMcro)?;
        let identifier = self.expect(TokenKind::Identifier)?;

        self.expect(TokenKind::ParenL)?;
        let mut params = Vec::new();
        while !self.match_token(TokenKind::ParenR) {
            let param_name = self.expect(TokenKind::Identifier)?;
            self.expect(TokenKind::Colon)?;
            let param_type = self.parse_ty()?;
            params.push(ParamDef {
                name: Ident(self.ast.add_symbol(&param_name.value)),
                ty: param_type,
            });
            if !self.match_token(TokenKind::Comma) {
                if self.match_token(TokenKind::ParenR) {
                    break;
                }
                self.expect(TokenKind::ParenR)?;
                break;
            }
        }

        let body = self.parse_block()?;
        let end_span = self.ast.get(body).span;

        let span = start.span.merge(end_span);

        let symbol = self.ast.add_symbol(&identifier.value);

        let node = LangNode {
            kind: LangNodeKind::MacroDef(MacroDef {
                name: Ident(symbol),
                params,
                body,
            }),
            span,
        };
        let id = self.ast.add(node);
        Ok(id)
    }
}
