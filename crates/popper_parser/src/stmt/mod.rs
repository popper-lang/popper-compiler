use crate::Parser;
use popper_ast::ast::LangNodeId;
use popper_ast::token::TokenKind;

mod assign;
mod block;
mod comptime;
mod const_;
mod extend;
mod function;
mod if_;
mod import;
mod let_;
mod macro_;
mod type_decl;
mod while_;

impl Parser {
    pub fn parse_stmt(&mut self, expr_terminated: bool) -> crate::error::Result<LangNodeId> {
        let token = self.cursor.peek_token()?;
        match token.kind {
            TokenKind::KeywordLet => self.parse_let_stmt(),
            TokenKind::KeywordIf => self.parse_if_stmt(),
            TokenKind::KeywordFunc => self.parse_function_stmt(false),
            TokenKind::KeywordReturn => self.parse_return(),
            TokenKind::KeywordStruct | TokenKind::KeywordUnion => self.parse_type_decl(),
            TokenKind::KeywordWhile => self.parse_while_stmt(),
            TokenKind::KeywordConst => self.parse_const_stmt(),
            TokenKind::KeywordMcro => self.parse_macro_stmt(),
            TokenKind::KeywordComptime => self.parse_comptime_stmt(),
            TokenKind::KeywordContext => self.parse_context_decl(),
            TokenKind::KeywordRequire => self.parse_require_context_decl(),
            TokenKind::KeywordImport => self.parse_import_decl(),
            TokenKind::KeywordIn => self.parse_in_context(),
            _ => {
                let (res, r) = self.parse_assign()?;

                if expr_terminated || !r {
                    self.expect(TokenKind::Semicolon)?;
                } else {
                    self.expect(TokenKind::Newline)?;
                }
                Ok(res)
            }
        }
    }
}
