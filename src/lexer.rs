use std::{str::FromStr};


#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    /*
    * Keywords list from:
    */
    FUN,
    LET,
    IF,
    ELSE,
    WHILE,
    CONST
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeToken {
    /*
    * Type(int, string, bool, array) list from:
    */
    INT,
    STRING,
    BOOL,
    ARRAY,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    /*
    * Operators list from:
    */
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,
    EQ,
    NEQ,
    GT,
    LT,
    GTE,
    LTE,
    AND,
    OR,
    NOT,
    ASSIGN,
    
}

impl FromStr for Keyword {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "fun" => Ok(Keyword::FUN),
            "let" => Ok(Keyword::LET),
            "if" => Ok(Keyword::IF),
            "else" => Ok(Keyword::ELSE),
            "while" => Ok(Keyword::WHILE),
            "const" => Ok(Keyword::CONST),
            _ => Err(format!("invalid keyword: {}", s))
        }
    }
}

impl FromStr for TypeToken {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "int" => Ok(TypeToken::INT),
            "string" => Ok(TypeToken::STRING),
            "bool" => Ok(TypeToken::BOOL),
            "array" => Ok(TypeToken::ARRAY),
            _ => Err(format!("invalid type: {}", s))
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /* 
    * The token type is represented by a single character.
    */
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    LBRACKET,
    RBRACKET,
    COMMA,
    DOT,
    Number(i32),
    String(String),
    Ident(String),
    Keyword(Keyword),
    Op(Op),
    Type(TypeToken),
    Illegal,
    SEMICOLON,
    TWODOTS,
    EOF,
}

#[derive(Debug, Clone)]
pub struct Lexer {
    /*
    * The lexer is a stateful object that keeps track of the current position in the input string.
    */
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: ' ',
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
    }
    pub fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input.chars().nth(self.read_position).unwrap()
        }
    }
    pub fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
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
        let token = match self.ch {
            '(' => {
                self.read_char();
                Token::LPAREN
            }
            ')' => {
                self.read_char();
                Token::RPAREN
            }
            '{' => {
                self.read_char();
                Token::LBRACE
            }
            '}' => {
                self.read_char();
                Token::RBRACE
            }
            '[' => {
                self.read_char();
                Token::LBRACKET
            }
            ']' => {
                self.read_char();
                Token::RBRACKET
            }
            ',' => {
                self.read_char();
                Token::COMMA
            }
            '.' => {
                self.read_char();
                Token::DOT
            }
            '"' => {
                let s = self.read_string();
                Token::String(s)
            }
            '0'..='9' => {
                let n = self.read_number();
                Token::Number(n)
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let s = self.read_identifier();
                Keyword::from_str(s.as_str())
                .ok()
                .map(Token::Keyword)
                .unwrap_or(
                    TypeToken::from_str(s.as_str()).ok().map(Token::Type).unwrap_or(
                        Token::Ident(s)
                    )
                )
            }
            '+' => {
                self.read_char();
                Token::Op(Op::ADD)
            }
            '-' => {
                self.read_char();
                Token::Op(Op::SUB)
            }
            '*' => {
                self.read_char();
                Token::Op(Op::MUL)
            }
            '/' => {
                self.read_char();
                Token::Op(Op::DIV)
            }
            '%' => {
                self.read_char();
                Token::Op(Op::MOD)
            }
            '=' => {
                self.read_char();
                Token::Op(Op::ASSIGN)
            }
            '!' => {
                self.read_char();
                if self.ch == '=' {
                    self.read_char();
                    Token::Op(Op::NEQ)
                } else {
                    Token::Op(Op::NOT)
                }
            }
            '>' => {
                self.read_char();
                if self.ch == '=' {
                    self.read_char();
                    Token::Op(Op::GTE)
                } else {
                    Token::Op(Op::GT)
                }
            }
            '<' => {
                self.read_char();
                if self.ch == '=' {
                    self.read_char();
                    Token::Op(Op::LTE)
                } else {
                    Token::Op(Op::LT)
                }
            }
            '&' => {
                self.read_char();
                if self.ch == '&' {
                    self.read_char();
                    Token::Op(Op::AND)
                } else {
                    Token::Illegal
                }
            }
            '|' => {
                self.read_char();
                if self.ch == '|' {
                    self.read_char();
                    Token::Op(Op::OR)
                } else {
                    Token::Illegal
                }
            }
            ';' => {
                self.read_char();
                Token::SEMICOLON
            }
            ':' => {
                self.read_char();
                Token::TWODOTS
            }
            '\0' => Token::EOF,
            _ => {
                self.read_char();
                Token::Illegal
            }
        };
        token
    }
    
}

mod tests {
    use super::*;
    #[test]
    fn test_ident() {
        let input = "foobar";
        let mut l = Lexer::new(input.to_string());
        let token = l.read_token();
        assert_eq!(token, Token::Ident("foobar".to_string()));
    }

    #[test]
    fn test_number() {
        let input = "12345";
        let mut l = Lexer::new(input.to_string());
        let token = l.read_token();
        assert_eq!(token, Token::Number(12345));
    }

    #[test]
    fn test_string() {
        let input = r#""foo bar""#;
        let mut l = Lexer::new(input.to_string());
        let token = l.read_token();
        assert_eq!(token, Token::String("foo bar".to_string()));
    }

    #[test]
    fn test_keyword() {
        let input = "let";
        let mut l = Lexer::new(input.to_string());
        let token = l.read_token();
        assert_eq!(token, Token::Keyword(Keyword::LET));
    }

    #[test]
    fn test_type() {
        let input = "int";
        let mut l = Lexer::new(input.to_string());
        let token = l.read_token();
        assert_eq!(token, Token::Type(TypeToken::INT));
    }

    #[test]
    fn test_op() {
        let input = "+";
        let mut l = Lexer::new(input.to_string());
        let token = l.read_token();
        assert_eq!(token, Token::Op(Op::ADD));
    }

    #[test]
    fn test_semicolon() {
        let input = ";";
        let mut l = Lexer::new(input.to_string());
        let token = l.read_token();
        assert_eq!(token, Token::SEMICOLON);
    }

    #[test]
    fn test_twodots() {
        let input = ":";
        let mut l = Lexer::new(input.to_string());
        let token = l.read_token();
        assert_eq!(token, Token::TWODOTS);
    }
}

        