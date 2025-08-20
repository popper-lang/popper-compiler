use popper_ast::ast::Span;
use popper_ast::token::{Token, TokenKind};

pub mod cursor;
pub mod error;
mod tests;

type Result<T> = std::result::Result<T, error::LexerError>;

pub fn tokenize(input: &str) -> Result<Vec<Token>> {
    let mut cursor = cursor::Cursor::new(input);
    let mut tokens = Vec::new();
    while !cursor.is_at_end() {
        let token = cursor.next_token()?;
        tokens.push(token.clone());
        if token.kind == TokenKind::Eof {
            break;
        }
    }
    if tokens.is_empty() || tokens.last().unwrap().kind != TokenKind::Eof {
        tokens.push(Token::new(TokenKind::Eof, Span::new(0, input.len()), String::new()));
    }
    Ok(tokens)
}
