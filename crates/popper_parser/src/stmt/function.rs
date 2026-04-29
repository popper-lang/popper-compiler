use crate::Parser;
use popper_ast::ast::{Ident, LangNode, LangNodeId, LangNodeKind, ParamDef, Span};
use popper_ast::layer::Ast;
use popper_ast::token::TokenKind;
use popper_ast::type_::Type;

impl Parser {
    pub(crate) fn parse_function_stmt(
        &mut self,
        is_method: bool,
    ) -> crate::error::Result<LangNodeId> {
        let start = self.expect(TokenKind::KeywordFunc)?;
        let mut attributes = vec![];
        if self.match_token(TokenKind::ParenL) {
            let mut i = 0;
            while !self.match_token(TokenKind::ParenR) {
                if i > 0 {
                    self.expect(TokenKind::Comma)?;
                }
                let attribute = self.parse_attribute()?;
                attributes.push(attribute);
                i += 1;
            }
        }
        let identifier = self.expect(TokenKind::Identifier)?;
        self.expect(TokenKind::ParenL)?; // Expect opening parenthesis for parameters
        let mut parameters = Vec::new();
        if is_method && !attributes.contains(&popper_ast::attribute::Attribute::Static) {
            parameters.push(ParamDef {
                name: Ident(self.ast.add_symbol("this")),
                ty: Type::SelfType,
            });
        }
        let mut i = 0;
        let mut is_vararg = false;
        while self.cursor.peek_token()?.kind != TokenKind::ParenR {
            if i > 0 {
                self.expect(TokenKind::Comma)?;
            }
            if self.match_token(TokenKind::DotDotDot) {
                is_vararg = true;
                break;
            }
            let param = self.expect(TokenKind::Identifier)?;
            self.expect(TokenKind::Colon)?;
            let ty = self.parse_ty()?;

            let ident = Ident(self.ast.add_symbol(&param.value));
            parameters.push(ParamDef { name: ident, ty });

            i += 1;
        }

        self.expect(TokenKind::ParenR)?;
        let return_type = if self.match_token(TokenKind::Arrow) {
            self.parse_ty()?
        } else {
            Type::Void
        };
        let (body, is_expr) = if self.match_token(TokenKind::Assign) {
            (Some(self.parse_expr()?), true)
        } else if self.match_token(TokenKind::Semicolon) {
            (None, false)
        } else {
            (Some(self.parse_block()?), false)
        };
        let end = self.cursor.pos();

        let node = LangNode {
            kind: LangNodeKind::FunctionDef {
                name: Ident(self.ast.add_symbol(&identifier.value)),
                attrs: attributes,
                params: parameters,
                ret: return_type,
                body,
                is_expr,
                is_vararg,
            },
            span: Span::new(start.span.lo, end),
        };

        let id = self.ast.add(node);
        Ok(id)
    }

    pub(crate) fn parse_return(&mut self) -> crate::error::Result<LangNodeId> {
        let start = self.expect(TokenKind::KeywordReturn)?;
        let expr = self.parse_expr()?;
        let end = self.expect(TokenKind::Semicolon)?;

        let node = LangNode {
            kind: LangNodeKind::Return(expr),
            span: start.span.merge(end.span),
        };

        Ok(self.ast.add(node))
    }
}
