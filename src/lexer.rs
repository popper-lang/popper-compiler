macro_rules! token {
    ($type: ident,$lexeme: expr, $line: expr, $pos: expr) => {
        Token {
            line: $line,
            pos: $pos,
            token_type: TokenType::$type,
            lexeme: $lexeme.to_string(),
        }
    };
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum TokenType {
    /*
     * The token type is represented by a single character.
     */
    // symbol token
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    LBRACKET,
    RBRACKET,
    COMMA,
    DOT,
    SEMICOLON,
    COLON,
    DOUBLECOLON,

    // literal token
    NUMBER,
    STRING,
    IDENT,

    // keyword token
    IF,
    ELSE,
    WHILE,
    CONST,
    FUN,
    LET,
    TO,
    CAST,
    CLASS,
    USE,
    AS,
    IN,
    IMPORT,
    IMPL,
    STRUCT,
    INIT,

    // operator token
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,
    POW,
    EQ,
    NEQ,
    LT,
    GT,
    LTE,
    GTE,
    EQUAL,
    OR,
    AND,
    ASSIGN,
    NOT,

    // type token
    INT_TYPE,
    STRING_TYPE,
    BOOL_TYPE,
    LIST_TYPE,

    ASM,

    // bool token
    TRUE,
    FALSE,

    NEWLINE,
    TAB,
    SPACE,
    Illegal,
    EOF,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Token {
    pub line: i32,
    pub pos: usize,
    pub token_type: TokenType,
    pub lexeme: String,
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
    line: i32,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: ' ',
            line: 1,
            pos: 0,
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
        while self.ch == '\r' || self.ch == ' ' || self.ch == '\t' {
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
        //self.skip_whitespace();
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
                    "bool" => token!(BOOL_TYPE, "bool", self.line, self.pos),
                    "array" => token!(LIST_TYPE, "array", self.line, self.pos),
                    "while" => token!(WHILE, "while", self.line, self.pos),
                    "fun" => token!(FUN, "fun", self.line, self.pos),
                    "class" => token!(CLASS, "class", self.line, self.pos),
                    "use" => token!(USE, "use", self.line, self.pos),
                    "as" => token!(AS, "as", self.line, self.pos),
                    "in" => token!(IN, "in", self.line, self.pos),
                    "import" => token!(IMPORT, "import", self.line, self.pos),
                    "impl" => token!(IMPL, "impl", self.line, self.pos),
                    "struct" => token!(STRUCT, "struct", self.line, self.pos),
                    "init" => token!(INIT, "init", self.line, self.pos),
                    "asm" => {
                        let mut asm = String::new();
                        if self.peek_char() == '{' {
                            self.read_char();
                        } else {
                            panic!("expected {{ after asm")
                        }
                        self.read_char();
                        while self.ch != '}' {
                            asm.push(self.ch);
                            self.read_char();

                        }
                        self.read_char();
                        token!(ASM, asm.as_str(), self.line, self.pos)
                    }
                    e => token!(IDENT, e, self.line, self.pos),
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
                if self.peek_char() == '/' {
                    self.read_char();
                    self.read_char();
                    while self.ch != '\n' {
                        self.read_char();
                    }
                    self.read_char();
                    return self.read_token();
                } else {
                    self.read_char();
                    token!(DIV, "/", self.line, self.pos)
                }
            }
            '%' => {
                self.read_char();
                token!(MOD, "%", self.line, self.pos)
            }
            '=' => {

                if self.peek_char() == '=' {
                    self.read_char();
                    self.read_char();
                    token!(EQ, "==", self.line, self.pos)
                } else {
                    token!(ASSIGN, "=", self.line, self.pos)
                }
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
                if self.ch == ':' {
                    self.read_char();
                    token!(DOUBLECOLON, "::", self.line, self.pos)
                } else {
                    token!(COLON, ":", self.line, self.pos)
                }
            }
            '\n' => {
                self.read_char();
                token!(NEWLINE, "\n", self.line, self.pos)
            }
            ' ' => {
                self.read_char();
                token!(SPACE, " ", self.line, self.pos)
            }
            '\t' => {
                self.read_char();
                token!(TAB, " ", self.line, self.pos)
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
pub(crate) fn tokens_to_string(tokens: Vec<Token>) -> String {
    let mut string = String::new();
    for token in tokens {
        if [
            TokenType::LET,  TokenType::IF, TokenType::ELSE,
            TokenType::WHILE, TokenType::FALSE, TokenType::TRUE,
            TokenType::ArrayType, TokenType::BouleanType, TokenType::CLASS,
            TokenType::IntType, TokenType::CAST, TokenType::FUN,
            TokenType::CONST, TokenType::StringType, TokenType::TO
        ].contains(&token.token_type) {
            string += format!("{} ", token.lexeme).as_str();
        } else {
            println!("{:?}", token);
            string += token.lexeme.as_str();
        }
    }
    string
}*/

mod tests {
    use super::*;
    #[test]
    fn test_ident() {
        let input = "foobar";
        let mut l = Lexer::new(input.to_string());
        let token = l.read_token();
        assert_eq!(token.token_type, TokenType::IDENT);
        assert_eq!(token.lexeme, "foobar");
    }

    #[test]
    fn test_number() {
        let input = "12345";
        let mut l = Lexer::new(input.to_string());
        let token = l.read_token();

        assert_eq!(token.token_type, TokenType::NUMBER);
        assert_eq!(token.lexeme, "12345")
    }

    #[test]
    fn test_string() {
        let input = r#""foo bar""#;
        let mut l = Lexer::new(input.to_string());
        let token = l.read_token();
        assert_eq!(token.token_type, TokenType::STRING);
        assert_eq!(token.lexeme, "foo bar")
    }

    #[test]
    fn test_keyword() {
        let input = "let";
        let mut l = Lexer::new(input.to_string());
        let token = l.read_token();
        assert_eq!(token.token_type, TokenType::LET);
    }

    #[test]
    fn test_type() {
        let input = "int";
        let mut l = Lexer::new(input.to_string());
        let token = l.read_token();
        assert_eq!(token.token_type, TokenType::IntType);
    }

    #[test]
    fn test_op() {
        let input = "+";
        let mut l = Lexer::new(input.to_string());
        let token = l.read_token();
        assert_eq!(token.token_type, TokenType::ADD);
    }

    #[test]
    fn test_semicolon() {
        let input = ";";
        let mut l = Lexer::new(input.to_string());
        let token = l.read_token();
        assert_eq!(token.token_type, TokenType::SEMICOLON);
    }

    #[test]
    fn test_twodots() {
        let input = ":";
        let mut l = Lexer::new(input.to_string());
        let token = l.read_token();
        assert_eq!(token.token_type, TokenType::TWODOTS);
    }
}
