use popper_ast::ast::{LangNode, LangNodeId, LangNodeKind, Span};
use popper_ast::layer::Ast;
use popper_ast::token::TokenKind;
use crate::Parser;

impl Parser {
    pub(crate) fn parse_block(&mut self) -> crate::error::Result<LangNodeId> {
        let start = self.cursor.pos();
        self.expect(TokenKind::BraceL)?;
        let mut stmts = Vec::new();
        while !self.match_token(TokenKind::BraceR) {
            stmts.push(self.parse_stmt(true)?);
        }
        
        let end = self.cursor.pos();
        
        let node = LangNode {
            kind: LangNodeKind::Block(stmts),
            span: Span::new(start, end),
        };
        
        
        Ok(self.ast.add(node))
        
        
    }
}