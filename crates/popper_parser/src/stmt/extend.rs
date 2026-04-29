use popper_ast::{
    ast::{Ident, LangNode, LangNodeId, LangNodeKind},
    layer::Ast,
    token::TokenKind,
};

use crate::{error::Result, Parser};

impl Parser {
    pub(crate) fn parse_extend_decl(&mut self) -> Result<LangNodeId> {
        let token = self.expect(TokenKind::KeywordExtend)?;
        let ty = self.parse_ty()?;
        self.expect(TokenKind::BraceL)?;
        let mut methodes = Vec::new();
        while !self.match_token(TokenKind::BraceR) {
            methodes.push(self.parse_function_stmt(true)?);
        }

        let other = self.cursor.peek_token()?;

        let lang_node = LangNode {
            kind: LangNodeKind::ExtendDecl {
                target_type: ty,
                methodes: methodes,
            },
            span: token.span.merge(other.span),
        };

        Ok(self.ast.add(lang_node))
    }

    pub(crate) fn parse_context_decl(&mut self) -> Result<LangNodeId> {
        let token = self.expect(TokenKind::KeywordContext)?;
        let name = self.expect(TokenKind::Identifier)?;
        let symbol_id = self.ast.add_symbol(&name.value);
        let mut extends = Vec::new();
        self.expect(TokenKind::BraceL)?;
        while !self.match_token(TokenKind::BraceR) {
            extends.push(self.parse_extend_decl()?);
        }

        let other = self.cursor.peek_token()?;

        let lang_node = LangNode {
            kind: LangNodeKind::ContextDecl {
                name: Ident(symbol_id),
                extensions: extends,
            },
            span: token.span.merge(other.span),
        };

        Ok(self.ast.add(lang_node))
    }

    pub(crate) fn parse_require_context_decl(&mut self) -> Result<LangNodeId> {
        let token = self.expect(TokenKind::KeywordRequire)?;
        self.expect(TokenKind::KeywordContext)?;
        let name = self.expect(TokenKind::Identifier)?;
        let symbol_id = self.ast.add_symbol(&name.value);
        self.expect(TokenKind::BraceL)?;
        let mut items = Vec::new();
        while !self.match_token(TokenKind::BraceR) {
            if self.cursor.peek_token()?.kind == TokenKind::KeywordExtend {
                items.push(self.parse_extend_decl()?);
            } else {
                items.push(self.parse_function_stmt(false)?);
            }
        }

        let other = self.cursor.peek_token()?;

        let lang_node = LangNode {
            kind: LangNodeKind::RequireContextDecl {
                name: Ident(symbol_id),
                items,
            },
            span: token.span.merge(other.span),
        };

        Ok(self.ast.add(lang_node))
    }

    pub(crate) fn parse_in_context(&mut self) -> Result<LangNodeId> {
        let token = self.expect(TokenKind::KeywordIn)?;
        let id = self.expect(TokenKind::Identifier)?;
        let block = self.parse_block()?;
        let other = self.cursor.peek_token()?;

        let node = LangNode {
            kind: LangNodeKind::InContext {
                context_name: Ident(self.ast.add_symbol(&id.value)),
                body: block,
            },
            span: token.span.merge(other.span),
        };

        Ok(self.ast.add(node))
    }
}
