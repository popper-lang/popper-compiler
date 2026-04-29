use popper_ast::ast::{Expr, Ident, LangNode, LangNodeId, LangNodeKind, Span};
use popper_ast::layer::Ast;
use popper_ast::token::TokenKind;
use crate::Parser;

impl Parser {
    pub(crate) fn parse_struct_instance(&mut self) -> crate::error::Result<LangNodeId> {
        let pos = self.cursor.pos();
        let base = self.parse_literal()?;

        let node = self.ast.get(base).clone();
        if let LangNodeKind::Expr(Expr::Ident(id)) = &node.kind {
            if !self.disallow_struct_literal && self.match_token(TokenKind::BraceL) {
                let mut fields = Vec::new();
                while !self.match_token(TokenKind::BraceR) {
                    let field_name_token = self.expect(TokenKind::Identifier)?;
                    let field_name = field_name_token.value.clone();
                    self.expect(TokenKind::Colon)?;
                    let field_value = self.parse_expr()?;
                    let field_ident = Ident(self.ast.add_symbol(&field_name));
                    fields.push((field_ident, field_value));

                    if !self.match_token(TokenKind::Comma) {
                        self.expect(TokenKind::BraceR)?; // Expect closing brace if no comma
                        break;
                    }
                }

                let node = LangNode {
                    kind: LangNodeKind::Expr(Expr::TypeDeclInstance {
                        type_name: *id,
                        fields,
                    }),
                    span: Span::new(pos, self.cursor.pos()),
                };

                Ok(self.ast.add(node))
            } else {
                Ok(base)
            }
        } else {
            Ok(base)
        }


    }
}