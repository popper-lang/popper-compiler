pub mod error;
mod stmt;
mod expr;
mod ty;
mod attribute;

use popper_ast::ast::{LangAst, LangNode, LangNodeKind, Span};
use popper_ast::file::SourceFileInfo;
use popper_ast::layer::Ast;
use popper_ast::token::{Token, TokenKind};
use popper_lexer::cursor::Cursor;

pub struct Parser {
    cursor: Cursor,
    ast: LangAst
}

impl Parser {
    
    pub fn from_source_file(source_file_info: SourceFileInfo) -> Self {
        let source = source_file_info.source();
        Parser::new(source)
    }
    pub fn new(s: &str) -> Self {
        Parser {
            cursor: Cursor::new(s),
            ast: LangAst::new()
        }
    }

    fn match_token(&mut self, expected: TokenKind) -> bool {
        if let Ok(token) = self.cursor.peek_token() {
            if token.kind == expected {
                let _ = self.cursor.next_token();
                return true;
            }
        }
        false
    }

    
    fn expect(&mut self, expected: TokenKind) -> error::Result<Token> {
        let current_token = self.cursor.peek_token()?;
        if current_token.kind == expected {
            self.cursor.next_token()?;
            return Ok(current_token);
        }

        Err(
            error::ParserError::expected_token(
                &[expected],
                current_token.clone(),
                current_token.span
            )
        )
    }

    fn expect_after(&mut self, expected: TokenKind) -> error::Result<Token> {
        let next_token = self.cursor.next_token()?;
        if next_token.kind == expected {
            return Ok(next_token);
        }

        let current_token = self.cursor.peek_token()?;


        Err(
            error::ParserError::expected_token(
                &[expected],
                current_token.clone(),
                current_token.span
            )
        )
    }
    
    
    pub fn parse(&mut self) -> error::Result<LangAst> {
        let start = self.cursor.pos();
        let mut block = Vec::new();
        while !self.cursor.is_at_end() {
            let token = self.cursor.peek_token()?;
            if token.kind == TokenKind::Eof {
                break;
            }
            block.push(self.parse_stmt(true)?);
        }
        let end = self.cursor.pos();
        let node = LangNode {
            kind: LangNodeKind::Block(block),
            span: Span::new(start, end)
        };
        
        let root = self.ast.add(node);
        self.ast.set_root(root);
        Ok(self.ast.clone())
    }
}
