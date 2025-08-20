use popper_ast::ast::LangNodeId;
use popper_ast::token::TokenKind;
use crate::Parser;

mod let_;
mod if_;
mod function;
mod block;

impl Parser {
    pub fn parse_stmt(&mut self, expr_terminated: bool) -> crate::error::Result<LangNodeId> {
        let token = self.cursor.peek_token()?;
        match token.kind {
            TokenKind::KeywordLet => {
                self.parse_let_stmt()
            }
            TokenKind::KeywordIf => {
                self.parse_if_stmt()
            }
            TokenKind::KeywordFunc => {
                self.parse_function_stmt()
            }
            TokenKind::KeywordReturn => {
                self.parse_return()
            }
            _ => {
                let res = self.parse_expr()?;
                if expr_terminated {
                    self.expect(TokenKind::Semicolon)?;
                } else {
                    self.expect(TokenKind::Newline)?;
                }
                Ok(res)
            }
        }
    }
}