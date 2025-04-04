use std::str::Chars;
use popper_ast::ast::Span;
use popper_ast::token::{Token, TokenKind};

#[derive(Debug, Clone)]
pub struct Cursor<'a> {
    chars: Chars<'a>,
    pos: usize,
    remaining: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Self {
        let remaining = input.len();
        Cursor {
            chars: input.chars(),
            pos: 0,
            remaining,
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.remaining == 0
    }

    pub fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }

    pub fn next(&mut self) -> Option<char> {
        let next = self.chars.next();
        if let Some(c) = next {
            self.pos += c.len_utf8();
            self.remaining -= c.len_utf8();
        }
        next
    }

    pub fn take(&mut self, n: usize) -> &'a str {
        let start = self.pos;
        let end = start + n;
        let s = &self.chars.as_str()[start..end];
        self.pos = end;
        self.remaining -= n;
        s
    }

    pub fn take_while<F>(&mut self, mut f: F) -> &'a str
    where
        F: FnMut(char) -> bool,
    {

        let start = 0;
        let mut end = start;
        let mut chars = self.chars.clone().as_str();
        while let Some(c) = self.chars.next() {
            if f(c) {
                end += c.len_utf8();
            } else {
                break;
            }
        }
        &chars[start..end]
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn remaining(&self) -> usize {
        self.remaining
    }

    pub fn next_token(&mut self) -> Token {
        let start = self.pos;
        let c = if let Some(c) = self.next() {
            c
        } else {
            return Token {
                kind: TokenKind::Eof,
                span: Span::new(start, self.pos),
                value: String::new(),
            };
        };

        let kind = match c {
            'a'..='z' | 'A'..='Z' | '_' => {
                let value = format!("{}{}", c, self.take_while(|c| c.is_ascii_alphanumeric() || c == '_'));
                let kind = match value.as_str() {
                    "let" => TokenKind::KeywordLet,
                    _ => TokenKind::Identifier,
                };

                Token::new(kind, Span::new(start, self.pos), value)
            }
            '0'..='9' => {
                let value = self.take_while(char::is_numeric);
                Token::new(TokenKind::Number, Span::new(start, self.pos), format!("{}{}", c, value))
            }
            '"' => {
                let value = self.take_while(|c| c != '"');
                self.next();
                Token::new(TokenKind::String, Span::new(start, self.pos), value.to_string())
            }
            '\'' => {
                let value = self.take(1);
                Token::new(TokenKind::Char, Span::new(start, self.pos), value.to_string())
            }
            _ => todo!()
        };

        kind
    }
}

