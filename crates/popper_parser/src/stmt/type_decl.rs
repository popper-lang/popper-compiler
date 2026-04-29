use crate::error::Result;
use crate::{parse_error, Parser};
use popper_ast::ast::LangNode;
use popper_ast::ast::LangNodeId;
use popper_ast::ast::LangNodeKind;
use popper_ast::ast::{Ident, ParamDef, Span, TypeDeclKind};
use popper_ast::layer::Ast;
use popper_ast::token::TokenKind;

impl Parser {
    pub(crate) fn parse_type_decl(&mut self) -> Result<LangNodeId> {
        let start = self.cursor.pos();
        let type_decl_kind = if self.match_token(TokenKind::KeywordStruct) {
            TypeDeclKind::Struct
        } else if self.match_token(TokenKind::KeywordUnion) {
            TypeDeclKind::Union
        } else {
            return Err(
                parse_error!(expect tokens [KeywordStruct, KeywordUnion] but got (self.cursor.peek_token()?)),
            );
        };
        let name_token = self.expect(TokenKind::Identifier)?;
        let name = name_token.value.clone();
        self.expect(TokenKind::BraceL)?;

        let mut fields = Vec::new();
        while !self.match_token(TokenKind::BraceR) {
            let field_name_token = self.expect(TokenKind::Identifier)?;
            let field_name = field_name_token.value.clone();
            self.expect(TokenKind::Colon)?;
            let field_type = self.parse_ty()?;
            fields.push(ParamDef {
                name: Ident(self.ast.add_symbol(&field_name)),
                ty: field_type,
            });
            if !self.match_token(TokenKind::Comma) {
                break;
            }
            self.cursor.next_token()?; // consume the comma
        }

        self.expect(TokenKind::BraceR)?;
        let end = self.cursor.pos();

        let node = LangNode {
            kind: LangNodeKind::TypeDecl {
                name: Ident(self.ast.add_symbol(&*name)),
                kind: type_decl_kind,
                fields,
            },
            span: Span { lo: start, hi: end },
        };

        Ok(self.ast.add(node))
    }
}
