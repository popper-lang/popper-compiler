use popper_ast::{
    ast::{Ident, LangNode, LangNodeId, LangNodeKind},
    layer::Ast,
    token::TokenKind,
};

use crate::{error::Result, Parser};

impl Parser {
    pub(crate) fn parse_import_decl(&mut self) -> Result<LangNodeId> {
        let token = self.expect(TokenKind::KeywordImport)?;

        // Parse the module path
        let path = if self.cursor.peek_token()?.kind == TokenKind::String {
            self.expect(TokenKind::String)?.value.clone()
        } else {
            self.expect(TokenKind::Identifier)?.value.clone()
        };

        let mut context_mappings = Vec::new();

        // Check if there are context mappings
        if self.match_token(TokenKind::BraceL) {
            while self.cursor.peek_token()?.kind != TokenKind::BraceR {
                let ctx_name_token = self.expect(TokenKind::Identifier)?;
                let ctx_name = Ident(self.ast.add_symbol(&ctx_name_token.value));

                self.expect(TokenKind::Assign)?;

                let ctx_impl_value = if self.match_token(TokenKind::KeywordInherit) {
                    "inherit".to_string()
                } else {
                    self.expect(TokenKind::Identifier)?.value.clone()
                };
                let ctx_impl = Ident(self.ast.add_symbol(&ctx_impl_value));

                context_mappings.push((ctx_name, ctx_impl));

                if !self.match_token(TokenKind::Comma) {
                    break;
                }
            }
            self.expect(TokenKind::BraceR)?;
        }

        let end = self.expect(TokenKind::Semicolon)?;

        let lang_node = LangNode {
            kind: LangNodeKind::ImportDecl {
                path,
                context_mappings,
            },
            span: token.span.merge(end.span),
        };

        Ok(self.ast.add(lang_node))
    }
}
