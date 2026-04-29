use crate::error::ParserError;
use crate::{parse_error, Parser};
use popper_ast::ast::{Expr, Ident, LangNode, LangNodeId, LangNodeKind, Span};
use popper_ast::layer::Ast;
use popper_ast::token::TokenKind;

impl Parser {
    fn parse_int_literal(&mut self) -> crate::error::Result<LangNodeId> {
        let token = self.expect(TokenKind::Number)?;
        let span = token.span;

        let node = LangNode {
            kind: LangNodeKind::Expr(Expr::Int(token.value.parse().unwrap())),
            span,
        };
        let id = self.ast.add(node);
        Ok(id)
    }

    fn parse_ident_literal(&mut self) -> crate::error::Result<LangNodeId> {
        let token = self.expect(TokenKind::Identifier)?;
        let span = token.span;

        let symbol = self.ast.add_symbol(&token.value);

        let node = LangNode {
            kind: LangNodeKind::Expr(Expr::Ident(Ident(symbol))),
            span,
        };
        let id = self.ast.add(node);
        Ok(id)
    }

    fn parse_string_literal(&mut self) -> crate::error::Result<LangNodeId> {
        let token = self.expect(TokenKind::String)?;
        let span = token.span;
        let node = LangNode {
            kind: LangNodeKind::Expr(Expr::String(token.value)),
            span,
        };

        Ok(self.ast.add(node))
    }

    fn parse_char_literal(&mut self) -> crate::error::Result<LangNodeId> {
        let token = self.expect(TokenKind::Char)?;
        let span = token.span;

        let node = LangNode {
            span,
            kind: LangNodeKind::Expr(Expr::Char(
                token
                    .value
                    .chars()
                    .nth(1)
                    .expect("It shouldnt panic (otherwise its a bug)"),
            )),
        };

        Ok(self.ast.add(node))
    }

    fn parse_bool_literal(&mut self) -> crate::error::Result<LangNodeId> {
        let pos = self.cursor.pos();
        if self.match_token(TokenKind::KeywordTrue) {
            let node = LangNode {
                kind: LangNodeKind::Expr(Expr::Bool(true)),
                span: Span::new(pos, self.cursor.pos()),
            };
            Ok(self.ast.add(node))
        } else if self.match_token(TokenKind::KeywordFalse) {
            let node = LangNode {
                kind: LangNodeKind::Expr(Expr::Bool(false)),
                span: Span::new(pos, self.cursor.pos()),
            };
            Ok(self.ast.add(node))
        } else {
            Err(
                parse_error!(expect tokens [KeywordTrue, KeywordFalse] but got (self.cursor.peek_token()?)),
            )
        }
    }

    fn parse_list_literal(&mut self) -> crate::error::Result<LangNodeId> {
        let start = self.expect(TokenKind::BracketL)?;
        let mut elements = Vec::new();
        if self.cursor.peek_token()?.kind != TokenKind::BracketR {
            loop {
                let element = self.parse_expr()?;
                elements.push(element);
                if self.cursor.peek_token()?.kind == TokenKind::BracketR {
                    break;
                }
                self.expect(TokenKind::Comma)?;
            }
        }
        let end = self.expect(TokenKind::BracketR)?;

        let node = LangNode {
            kind: LangNodeKind::Expr(Expr::List(elements)),
            span: start.span.merge(end.span),
        };
        Ok(self.ast.add(node))
    }

    fn parse_builtin_call(&mut self) -> crate::error::Result<LangNodeId> {
        self.expect(TokenKind::At)?;
        let name_token = self.expect(TokenKind::Identifier)?;
        self.expect(TokenKind::ParenL)?;
        let mut args = Vec::new();
        while !self.match_token(TokenKind::ParenR) {
            args.push(self.parse_expr()?);
            if !self.match_token(TokenKind::Comma) {
                if self.match_token(TokenKind::ParenR) {
                    break;
                }
                self.expect(TokenKind::ParenR)?;
                break;
            }
        }
        let end_pos = self.cursor.pos();
        let node = LangNode {
            kind: LangNodeKind::Expr(Expr::BuiltinCall {
                name: name_token.value,
                args,
            }),
            span: Span::new(name_token.span.lo - 1, end_pos),
        };
        Ok(self.ast.add(node))
    }

    fn parse_type_expr(&mut self) -> crate::error::Result<LangNodeId> {
        let pos = self.cursor.pos();
        let ty = self.parse_ty()?;
        let node = LangNode {
            kind: LangNodeKind::Expr(Expr::Type(ty)),
            span: Span::new(pos, self.cursor.pos()),
        };
        Ok(self.ast.add(node))
    }

    fn parse_list_literal_or_type_expr(&mut self) -> crate::error::Result<LangNodeId> {
        let saved_cursor = self.cursor.clone();
        if let Ok(ty_node_id) = self.parse_type_expr() {
            return Ok(ty_node_id);
        }
        self.cursor = saved_cursor;
        self.parse_list_literal()
    }

    pub(crate) fn parse_literal(&mut self) -> crate::error::Result<LangNodeId> {
        let token = self.cursor.peek_token()?;
        match token.kind {
            TokenKind::Number => self.parse_int_literal(),
            TokenKind::Identifier => self.parse_ident_literal(),
            TokenKind::String => self.parse_string_literal(),
            TokenKind::Char => self.parse_char_literal(),
            TokenKind::KeywordTrue | TokenKind::KeywordFalse => self.parse_bool_literal(),
            TokenKind::BracketL => self.parse_list_literal_or_type_expr(),
            TokenKind::At => self.parse_builtin_call(),
            TokenKind::ParenL => {
                self.match_token(TokenKind::ParenL);
                let expr = self.parse_expr()?;
                self.expect(TokenKind::ParenR)?;
                Ok(expr)
            }
            TokenKind::TypeInt
            | TokenKind::TypeChar
            | TokenKind::TypeFloat
            | TokenKind::TypeBool
            | TokenKind::TypeString
            | TokenKind::TypeVoid
            | TokenKind::TypeType
            | TokenKind::Bang => self.parse_type_expr(),
            _ => Err(
                parse_error!(expect tokens [Number, Identifier, String, Char, KeywordTrue, KeywordFalse, BracketL, At, TypeInt, TypeFloat, TypeBool, TypeString, TypeVoid, TypeType, Bang] but got (token)),
            ),
        }
    }
}
