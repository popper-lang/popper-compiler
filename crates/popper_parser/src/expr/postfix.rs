use popper_ast::ast::{Expr, Ident, LangNode, LangNodeId, LangNodeKind, Span};
use popper_ast::layer::Ast;
use popper_ast::token::TokenKind;
use crate::Parser;

impl Parser {
    pub(crate) fn parse_postfix(&mut self) -> crate::error::Result<LangNodeId> {
        let start = self.cursor.pos();
        let mut expr = self.parse_struct_instance()?;
        loop {
            if self.match_token(TokenKind::ParenL) {
                let mut arguments = Vec::new();
                let mut i = 0;
                while !self.match_token(TokenKind::ParenR) {
                    if i > 0 {
                        self.expect(TokenKind::Comma)?;
                    }

                    let arg = self.parse_expr()?;
                    arguments.push(arg);
                    i += 1;
                }

                let end = self.cursor.pos();

                let node = LangNode {
                    kind: LangNodeKind::FunctionCall {
                        function: expr,
                        args: arguments
                    },
                    span: Span::new(start, end),
                };
                expr = self.ast.add(node);
            } else if self.match_token(TokenKind::Dot) {
                let field = self.expect(TokenKind::Identifier)?;
                let end = self.cursor.pos();
                let node = LangNode {
                    kind: LangNodeKind::Expr(Expr::FieldAccess {
                        base: expr,
                        field: Ident(self.ast.add_symbol(&field.value)),
                    }),
                    span: Span::new(start, end),
                };
                expr = self.ast.add(node);
            } else if self.match_token(TokenKind::BracketL) {
                let index = self.parse_expr()?;
                self.expect(TokenKind::BracketR)?;
                let end = self.cursor.pos();
                let node = LangNode {
                    kind: LangNodeKind::Expr(Expr::Index {
                        base: expr,
                        index,
                    }),
                    span: Span::new(start, end),
                };
                expr = self.ast.add(node);
            } else {
                break;
            }
        }

        Ok(expr)
    }
}