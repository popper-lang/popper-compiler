use std::alloc::handle_alloc_error;
use std::iter::Peekable;
use std::str::Chars;
use popper_ast::ast::Span;
use popper_ast::token::{Token, TokenKind};

#[derive(Debug, Clone)]
pub struct Cursor {
    string: String,
    pos: usize,
    remaining: usize,
}

impl Cursor {
    pub fn new(input: &str) -> Self {
        let remaining = input.len();
        Cursor {
            string: input.to_string(),
            pos: 0,
            remaining,
        }
    }
    

    pub fn is_at_end(&self) -> bool {
        self.remaining == 0
    }

    pub fn peek(&self) -> Option<char> {
        self.string[self.pos..].chars().next()
    }

    pub fn next(&mut self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }
        let c = self.string[self.pos..].chars().next()?;
        self.pos += c.len_utf8();
        self.remaining -= c.len_utf8();
        Some(c)
        
    }

    pub fn take(&mut self, n: usize, peek: bool) -> String {
        let start = self.pos;
        let end = start + n;
        if end > self.string.len() {
            self.remaining = 0;
            return String::new(); // or handle error
        }
        let s = &self.string[start..end];
        if peek {
            return s.to_string();
        }
        self.pos = end;
        self.remaining -= n;
        s.to_string()
    }
    

    pub fn take_while<F>(&mut self, mut f: F, peek: bool) -> String
    where
        F: FnMut(char) -> bool,
    {

        let start = 0;
        let mut end = start;
        let s = self.string[self.pos..].chars().collect::<String>();
        for c in s.chars() {
            if f(c) {
                end += c.len_utf8();
                if !peek {
                    self.pos += c.len_utf8();
                    self.remaining -= c.len_utf8();
                }
            } else {
                break;
            }
        }
        s[start..end].to_string()
        
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn remaining(&self) -> usize {
        self.remaining
    }

    fn process_token(&mut self, c: char, start: usize, peek: bool) -> super::Result<Token> {
        let kind = match c {
            'a'..='z' | 'A'..='Z' | '_' => {
                let value = self.take_while(|c| c.is_ascii_alphanumeric() || c == '_', peek);
                let kind = match value.as_str() {
                    "let" => TokenKind::KeywordLet,
                    "if" => TokenKind::KeywordIf,
                    "else" => TokenKind::KeywordElse,
                    "func" => TokenKind::KeywordFunc,
                    "return" => TokenKind::KeywordReturn,
                    "int" => TokenKind::TypeInt,
                    "float" => TokenKind::TypeFloat,
                    "bool" => TokenKind::TypeBool,
                    "string" => TokenKind::TypeString,
                    "void" => TokenKind::TypeVoid,
                    _ => TokenKind::Identifier,
                };

                Token::new(kind, Span::new(start, self.pos), value)
            }
            '0'..='9' => {
                let value = self.take_while(char::is_numeric, peek);
                Token::new(TokenKind::Number, Span::new(start, self.pos), value)
            }
            '"' => {
                if peek {
                    self.pos += 1;
                    self.remaining -= 1;
                } else {
                    self.take(1, peek);
                }
                let value = self.take_while(|c| c != '"', peek);
                if peek {
                    self.pos -= 1;
                    self.remaining += 1;
                } else {
                    self.take(1, peek);
                }
                Token::new(TokenKind::String, Span::new(start, self.pos), value.to_string())
            }
            '\'' => {
                let value = self.take(1, peek);
                Token::new(TokenKind::Char, Span::new(start, self.pos), value.to_string())
            }
            ',' => {
                self.take(1, peek);
                Token::new(TokenKind::Comma, Span::new(start, self.pos), ",".to_string())
            }
            '(' => {
                self.take(1, peek);
                Token::new(TokenKind::ParenL, Span::new(start, self.pos), "(".to_string())
            }
            ')' => {
                self.take(1, peek);
                Token::new(TokenKind::ParenR, Span::new(start, self.pos), ")".to_string())
            }
            '{' => {
                self.take(1, peek);
                Token::new(TokenKind::BraceL, Span::new(start, self.pos), "{".to_string())
            }
            '}' => {
                self.take(1, peek);
                Token::new(TokenKind::BraceR, Span::new(start, self.pos), "}".to_string())
            }
            '[' => {
                self.take(1, peek);
                Token::new(TokenKind::BracketL, Span::new(start, self.pos), "[".to_string())
            }
            ']' => {
                self.take(1, peek);
                Token::new(TokenKind::BracketR, Span::new(start, self.pos), "]".to_string())
            }
            '=' => {
                self.take(1, peek);
                Token::new(TokenKind::Eq, Span::new(start, self.pos), "=".to_string())
            }
            '+' => {
                self.take(1, peek);
                Token::new(TokenKind::Plus, Span::new(start, self.pos), "+".to_string())
            }
            '-' => {
                self.take(1, peek);
                let speek = if peek {
                    self.string[self.pos+1..].chars().next()
                } else {
                    self.peek()
                };
                if speek == Some('>') {
                    self.take(1, peek);
                    return Ok(Token::new(TokenKind::Arrow, Span::new(start, self.pos), "->".to_string()));
                }
                Token::new(TokenKind::Minus, Span::new(start, self.pos), "-".to_string())
            }
            '*' => {
                self.take(1, peek);
                Token::new(TokenKind::Multiply, Span::new(start, self.pos), "*".to_string())
            }
            '/' => {
                self.take(1, peek);
                Token::new(TokenKind::Divide, Span::new(start, self.pos), "/".to_string())
            }
            ';' => {
                self.take(1, peek);
                Token::new(TokenKind::Semicolon, Span::new(start, self.pos), ";".to_string())
            }
            ':' => {
                self.take(1, peek);
                Token::new(TokenKind::Colon, Span::new(start, self.pos), ":".to_string())
            }
            ' ' | '\t' | '\r' | '\n' => {
                
                let v = self.take_while(|c| c.is_whitespace(), false);
                let c = self.peek_token()?;
                c
            }
            _ => return Err(crate::error::LexerError::unexpected_token(
                format!("{}", c),
                Span::new(start, self.pos),
            ))
        };
        
        Ok(kind)
    }
    pub fn next_token(&mut self) -> super::Result<Token> {
        let start = self.pos;
        let c = if let Some(c) = self.peek() {
            c
        } else {
            return Ok(Token {
                kind: TokenKind::Eof,
                span: Span::new(start, self.pos),
                value: String::new(),
            });
        };

        self.process_token(c, start, false)
        
    }

    pub fn peek_token(&mut self) -> super::Result<Token> {
        let c = if let Some(c) = self.peek() {
            c
        } else {
            return Ok(Token {
                kind: TokenKind::Eof,
                span: Span::new(self.pos, self.pos),
                value: String::new(),
            });
        };
        
        let start = self.pos;
        self.process_token(c, start, true)
    }

}

