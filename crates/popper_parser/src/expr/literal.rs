use crate::error::ParserError;
use popper_ast::ast::{Expr, Ident, LangNode, LangNodeId, LangNodeKind};
use popper_ast::layer::Ast;
use popper_ast::token::TokenKind;
use crate::{parse_error, Parser};

impl Parser {
    fn parse_int_literal(&mut self) -> crate::error::Result<LangNodeId> {
        let token = self.expect(TokenKind::Number)?;
        let span = token.span;

        let node = LangNode {
            kind: LangNodeKind::Expr(
                Expr::Int(token.value.parse().unwrap())
            ),
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
            kind: LangNodeKind::Expr(
                Expr::Ident(Ident(symbol))
            ),
            span,
        };
        let id = self.ast.add(node);
        Ok(id)
    }
    
    fn parse_string_literal(&mut self) -> crate::error::Result<LangNodeId> {
        let token = self.expect(TokenKind::String)?;
        let span = token.span;
        let node  = LangNode {
            kind: LangNodeKind::Expr(
                Expr::String(token.value)
            ),
            span,
        };
        
        Ok(self.ast.add(node))
        
    }

    pub(crate) fn parse_literal(&mut self) -> crate::error::Result<LangNodeId> {
        let token = self.cursor.peek_token()?;
        match token.kind {
            TokenKind::Number => self.parse_int_literal(),
            TokenKind::Identifier => self.parse_ident_literal(),
            TokenKind::String => self.parse_string_literal(),
            _ => Err(parse_error!(expect tokens [Number, Identifier] but got (token))),
        }
    }
}