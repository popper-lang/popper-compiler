use popper_ast::token::{Token, TokenKind};

mod cursor;
mod tests;

pub fn tokenize(input: &str) -> Vec<Token> {

    let mut cursor = cursor::Cursor::new(input);
    let mut tokens = Vec::new();
    while !cursor.is_at_end() {
        let token = cursor.next_token();
        tokens.push(token.clone());
        if token.kind == TokenKind::Eof {
            break;
        }
    }
    tokens
}