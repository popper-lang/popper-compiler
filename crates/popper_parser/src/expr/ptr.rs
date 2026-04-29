use popper_ast::{
    ast::{Expr, LangNode, LangNodeId, LangNodeKind},
    layer::Ast,
    token::TokenKind,
};

use crate::{error::Result, Parser};

impl Parser {
    pub(crate) fn parse_ref(&mut self) -> Result<LangNodeId> {
        let token = self.cursor.peek_token()?;
        if token.kind == TokenKind::Ampersand {
            let _ = self.cursor.next_token()?;
            let inner = self.parse_deref()?;
            let node = LangNode {
                kind: LangNodeKind::Expr(Expr::Ref(inner)),
                span: token.span.merge(self.cursor.peek_token()?.span),
            };
            Ok(self.ast.add(node))
        } else {
            self.parse_deref()
        }
    }

    pub(crate) fn parse_deref(&mut self) -> Result<LangNodeId> {
        let token = self.cursor.peek_token()?;
        if token.kind == TokenKind::Bang {
            let _ = self.cursor.next_token()?;
            let inner = self.parse_postfix()?;
            let node = LangNode {
                kind: LangNodeKind::Expr(Expr::Deref(inner)),
                span: token.span.merge(self.cursor.peek_token()?.span),
            };
            Ok(self.ast.add(node))
        } else {
            self.parse_postfix()
        }
    }
}
