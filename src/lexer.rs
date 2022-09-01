
macro_rules! token {
    ($type: ident,$lexeme: expr, $line: expr, $pos: expr) => {
        Token {
            line: $line,
            pos: $pos,
            token_type: TokenType::$type,
            lexeme: $lexeme.to_string()
        }
    };
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum TokenType {
    /* 
    * The token type is represented by a single character.
    */

    // symbol token
    LPAREN, RPAREN, LBRACE, RBRACE, LBRACKET, RBRACKET, COMMA, DOT, SEMICOLON, TWODOTS,

    // literal token
    NUMBER, STRING, IDENT,

    // keyword token
    IF, ELSE, WHILE, CONST, FUN, LET, TO, CAST, CLASS,
    
    // operator token
    ADD, SUB, MUL, DIV, MOD, POW, EQ, NEQ, LT, GT, LTE, GTE, EQUAL, OR, AND, ASSIGN, NOT,

    // type token
    INT_TYPE, STRING_TYPE, BOOLEAN_TYPE, ARRAY_TYPE,

    // bool token
    TRUE, FALSE,

    Illegal,
    EOF,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Token {
    pub line: i32,
    pub pos: usize,
    pub token_type: TokenType,
    pub lexeme: String
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
        let token = match self.ch {
            '(' => {
                self.read_char();
                token!(LPAREN, "(", self.line, self.pos)
            }
            ')' => {
                self.read_char();
                token!(RPAREN, ")", self.line, self.pos)
            }
            '{' => {
                self.read_char();
                token!(LBRACE, "{", self.line, self.pos)
            }
            '}' => {
                self.read_char();
                token!(RBRACE, "}", self.line, self.pos)
            }
            '[' => {
                self.read_char();
                token!(LBRACKET, "[", self.line, self.pos)
            }
            ']' => {
                self.read_char();
                token!(RBRACKET, "]", self.line, self.pos)
            }
            ',' => {
                self.read_char();
                token!(COMMA, ",", self.line, self.pos)
            }
            '.' => {
                self.read_char();
                token!(DOT, ".", self.line, self.pos)
            }
            '"' => {
                let s = self.read_string();
                token!(STRING, s.as_str(), self.line, self.pos)
            }
            '0'..='9' => {
                let n = self.read_number();
                token!(NUMBER, n.to_string().as_str(), self.line, self.pos)
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let s = self.read_identifier();
                match s.as_str() {
                    "if" => token!(IF, "if", self.line, self.pos),
                    "else" => token!(ELSE, "else", self.line, self.pos),
                    "to" => token!(TO, "to", self.line, self.pos),
                    "cast" => token!(CAST, "cast", self.line, self.pos),
                    "const" => token!(CONST, "const", self.line, self.pos),
                    "true" => token!(TRUE, "true", self.line, self.pos),
                    "false" => token!(FALSE, "false", self.line, self.pos),
                    "let" => token!(LET, "let", self.line, self.pos),
                    "int" => token!(INT_TYPE, "int", self.line, self.pos),
                    "string" => token!(STRING_TYPE, "string", self.line, self.pos),
                    "bool" => token!(BOOLEAN_TYPE, "bool", self.line, self.pos),
                    "array" => token!(ARRAY_TYPE, "array", self.line, self.pos),
                    "while" => token!(WHILE, "while", self.line, self.pos),
                    "fun" => token!(FUN, "fun", self.line, self.pos),
                    "class" => token!(CLASS, "class", self.line, self.pos),
                    e => token!(IDENT, e, self.line, self.pos)
                }
                
            }
            '+' => {
                self.read_char();
                token!(ADD, "+", self.line, self.pos)
            }
            '-' => {
                self.read_char();
                token!(SUB, "-", self.line, self.pos)
            }
            '*' => {
                self.read_char();
                token!(MUL, "*", self.line, self.pos)
            }
            '/' => {
                self.read_char();
                token!(DIV, "/", self.line, self.pos)
            }
            '%' => {
                self.read_char();
                token!(MOD, "%", self.line, self.pos)
            }
            '=' => {
                self.read_char();
                token!(ASSIGN, "=", self.line, self.pos)
            }
            '!' => {
                self.read_char();
                if self.ch == '=' {
                    self.read_char();
                    token!(NEQ, "!", self.line, self.pos)
                } else {
                    token!(NOT, "!", self.line, self.pos)
                }
            }
            '>' => {
                self.read_char();
                if self.ch == '=' {
                    self.read_char();
                    token!(GTE, ">=", self.line, self.pos)
                } else {
                    token!(GT, ">", self.line, self.pos)
                }
            }
            '<' => {
                self.read_char();
                if self.ch == '=' {
                    self.read_char();
                    token!(LTE, "<=", self.line, self.pos)
                } else {
                    token!(LT, "<", self.line, self.pos)
                }
            }
            '&' => {
                self.read_char();
                if self.ch == '&' {
                    self.read_char();
                    token!(AND, "&&", self.line, self.pos)
                } else {
                    token!(Illegal, "", self.line, self.pos)
                }
            }
            '|' => {
                self.read_char();
                if self.ch == '|' {
                    self.read_char();
                    token!(OR, "||", self.line, self.pos)
                } else {
                    token!(Illegal, "", self.line, self.pos)
                }
            }
            ';' => {
                self.read_char();
                token!(SEMICOLON, ";", self.line, self.pos)
            }
            ':' => {
                self.read_char();
                token!(TWODOTS, ":", self.line, self.pos)
            }
            
            '\0' => token!(EOF, "", self.line, self.pos),
            _ => {
                self.read_char();
                token!(Illegal, "", self.line, self.pos)
            }
        };
        token
    }

    pub fn scan_token(&mut self) -> Vec<Token> {
        let mut current = self.read_token();
        let mut list_token = Vec::new();
        while current.token_type != TokenType::EOF {
            list_token.push(current);
            current = self.read_token();
        }
        list_token.push(token!(EOF, "", self.line, self.pos));
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