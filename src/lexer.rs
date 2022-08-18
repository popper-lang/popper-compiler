use std::{str::FromStr};


#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    /* 
    * The token type is represented by a single character.
    */

    // symbol token
    LPAREN, RPAREN, LBRACE, RBRACE, LBRACKET, RBRACKET, COMMA, DOT, SEMICOLON, TWODOTS,

    // literal token
    Number(i32), String(String), Ident(String),

    // keyword token
    IF, ELSE, WHILE, CONST, FUN, LET, TO, CAST,
    
    // operator token
    ADD, SUB, MUL, DIV, MOD, POW, EQ, NEQ, LT, GT, LTE, GTE, EQUAL, OR, AND, ASSIGN, NOT,

    // type token
    INT, STRING, BOOLEAN, ARRAY,

    // bool token
    TRUE, FALSE,

    Illegal,
    EOF,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub line: i32,
    pub pos: usize,
    pub token_type: TokenType,
}

#[derive(Debug, Clone)]
pub struct Lexer {
    /*
    * The lexer is a stateful object that keeps track of the current position in the input string.
    */
    input: String,
    position: usize,
    read_position: usize,
    pos: usize,
    ch: char,
    line: i32
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: ' ',
            line: 1,
            pos: 0
        };
        l.read_char();
        l
    }
    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
        self.pos += 1;
        
    }
    pub fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input.chars().nth(self.read_position).unwrap()
        }
    }
    pub fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\r' || self.ch == '\n' {
            if self.ch == '\n' {
                self.line += 1;
                self.pos = 0;
            }
            self.read_char();
        }
    }
    pub fn read_identifier(&mut self) -> String {
        let mut result = String::new();
        while self.ch.is_alphanumeric() || self.ch == '_' {
            result.push(self.ch);
            self.read_char();
        }
        result
    }
    pub fn read_number(&mut self) -> i32 {
        let mut result = String::new();
        while self.ch.is_digit(10) {
            result.push(self.ch);
            self.read_char();
        }
        result.parse::<i32>().unwrap()
    }
    pub fn read_string(&mut self) -> String {
        let mut result = String::new();
        self.read_char();
        while self.ch != '"' {
            result.push(self.ch);
            self.read_char();
        }
        self.read_char();
        result
    }

    pub fn read_token(&mut self) -> Token {
        self.skip_whitespace();
        let token = Token {
            line: self.line,
            pos: self.read_position,
            token_type: match self.ch {
            '(' => {
                self.read_char();
                TokenType::LPAREN
            }
            ')' => {
                self.read_char();
                TokenType::RPAREN
            }
            '{' => {
                self.read_char();
                TokenType::LBRACE
            }
            '}' => {
                self.read_char();
                TokenType::RBRACE
            }
            '[' => {
                self.read_char();
                TokenType::LBRACKET
            }
            ']' => {
                self.read_char();
                TokenType::RBRACKET
            }
            ',' => {
                self.read_char();
                TokenType::COMMA
            }
            '.' => {
                self.read_char();
                TokenType::DOT
            }
            '"' => {
                let s = self.read_string();
                TokenType::String(s)
            }
            '0'..='9' => {
                let n = self.read_number();
                TokenType::Number(n)
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let s = self.read_identifier();
                match s.as_str() {
                    "if" => TokenType::IF,
                    "else" => TokenType::ELSE,
                    "to" => TokenType::TO,
                    "cast" => TokenType::CAST,
                    "const" => TokenType::CONST,
                    "true" => TokenType::TRUE,
                    "false" => TokenType::FALSE,
                    "let" => TokenType::LET,
                    "int" => TokenType::INT,
                    "string" => TokenType::STRING,
                    "bool" => TokenType::BOOLEAN,
                    "array" => TokenType::ARRAY,
                    "while" => TokenType::WHILE,
                    _ => TokenType::Ident(s)
                }
                
            }
            '+' => {
                self.read_char();
                TokenType::ADD
            }
            '-' => {
                self.read_char();
                TokenType::SUB
            }
            '*' => {
                self.read_char();
                TokenType::MUL
            }
            '/' => {
                self.read_char();
                TokenType::DIV
            }
            '%' => {
                self.read_char();
                TokenType::MOD
            }
            '=' => {
                self.read_char();
                TokenType::ASSIGN
            }
            '!' => {
                self.read_char();
                if self.ch == '=' {
                    self.read_char();
                    TokenType::NEQ
                } else {
                    TokenType::NOT
                }
            }
            '>' => {
                self.read_char();
                if self.ch == '=' {
                    self.read_char();
                    TokenType::GTE
                } else {
                    TokenType::GT
                }
            }
            '<' => {
                self.read_char();
                if self.ch == '=' {
                    self.read_char();
                    TokenType::LTE
                } else {
                    TokenType::LT
                }
            }
            '&' => {
                self.read_char();
                if self.ch == '&' {
                    self.read_char();
                    TokenType::AND
                } else {
                    TokenType::Illegal
                }
            }
            '|' => {
                self.read_char();
                if self.ch == '|' {
                    self.read_char();
                    TokenType::OR
                } else {
                    TokenType::Illegal
                }
            }
            ';' => {
                self.read_char();
                TokenType::SEMICOLON
            }
            ':' => {
                self.read_char();
                TokenType::TWODOTS
            }
            
            '\0' => TokenType::EOF,
            _ => {
                self.read_char();
                TokenType::Illegal
            }
        }};
        token
    }

    pub fn scan_token(&mut self) -> Vec<Token> {
        let mut current = self.read_token();
        let mut list_token = Vec::new();
        while current.token_type != TokenType::EOF {
            list_token.push(current);
            current = self.read_token();
        }
        list_token.push(Token { token_type: TokenType::EOF, line: current.line, pos: current.pos});
        list_token
     }
    
}
/* 
mod tests {
    use super::*;
    #[test]
    fn test_ident() {
        let input = "foobar";
        let mut l = Lexer::new(input.to_string());
        let token = l.read_token();
        assert_eq!(token, TokenType::Ident("foobar".to_string()));
    }

    #[test]
    fn test_number() {
        let input = "12345";
        let mut l = Lexer::new(input.to_string());
        let token = l.read_token();
        assert_eq!(token, TokenType::Number(12345));
    }

    #[test]
    fn test_string() {
        let input = r#""foo bar""#;
        let mut l = Lexer::new(input.to_string());
        let token = l.read_token();
        assert_eq!(token, TokenType::String("foo bar".to_string()));
    }

    #[test]
    fn test_keyword() {
        let input = "let";
        let mut l = Lexer::new(input.to_string());
        let token = l.read_token();
        assert_eq!(token, TokenType::Keyword(Keyword::LET));
    }

    #[test]
    fn test_type() {
        let input = "int";
        let mut l = Lexer::new(input.to_string());
        let token = l.read_token();
        assert_eq!(token, TokenType::Type(TypeToken::INT));
    }

    #[test]
    fn test_op() {
        let input = "+";
        let mut l = Lexer::new(input.to_string());
        let token = l.read_token();
        assert_eq!(token, TokenType::Op(Op::ADD));
    }

    #[test]
    fn test_semicolon() {
        let input = ";";
        let mut l = Lexer::new(input.to_string());
        let token = l.read_token();
        assert_eq!(token, TokenType::SEMICOLON);
    }

    #[test]
    fn test_twodots() {
        let input = ":";
        let mut l = Lexer::new(input.to_string());
        let token = l.read_token();
        assert_eq!(token, TokenType::TWODOTS);
    }
}

*/