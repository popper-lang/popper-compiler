use crate::ast::Span;


#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    pub value: String,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span, value: String) -> Token {
        Token { kind, span, value }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Identifier,
    Number,
    String,
    Char,
    Symbol,
    KeywordLet,
    Whitespace,
    Comment,
    Newline,
    Eof,
}

pub struct TokenStream {
    pub tokens: Vec<Token>,
    pub index: usize,
}

impl TokenStream {
    pub fn new() -> TokenStream {
        TokenStream {
            tokens: Vec::new(),
            index: 0,
        }
    }

    pub fn push(&mut self, token: Token) {
        self.tokens.push(token);
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }




}
