use popper_ast::ast::{Expr, LangNode, LangNodeId, LangNodeKind};
use popper_ast::layer::Ast;
use popper_ast::token::TokenKind;

use crate::error::Result;
use crate::{parse_error, Parser};

impl Parser {
    pub(crate) fn parse_lhs_assign(&mut self) -> Result<(LangNodeId, bool)> {
        let node_id  = self.parse_expr()?;
        let node = self.ast.get(node_id);
        match &node.kind {
            LangNodeKind::Expr(expr) => match expr {
                Expr::Ident(_) | Expr::Deref(_) => Ok((node_id, true)),
                Expr::FieldAccess { .. } => Ok((node_id, true)),
                Expr::Index { .. } => Ok((node_id, true)),
                _ => Ok((node_id, false)),
            },
            _ => Ok((node_id, false)),
        }
    }

    pub(crate) fn parse_assign(&mut self) -> Result<(LangNodeId, bool)> {
        let first = self.cursor.peek_token()?;
        let (lhs, is_valid_for_assign) = self.parse_lhs_assign()?;

        if self.match_token(TokenKind::Assign) {
            if !is_valid_for_assign {
                let node = self.ast.get(lhs);
                return Err(parse_error!(expect valid expressions [Ident, Deref] but got (node)));
            }
            let rhs = self.parse_expr()?;
            let last = self.cursor.peek_token()?;

            let span = first.span.merge(last.span);

            let node = LangNode {
                kind: LangNodeKind::Assign { lhs, rhs },
                span,
            };

            let node_id = self.ast.add(node);

            Ok((node_id, false))
        } else {
            Ok((lhs, true))
        }
    }
}
