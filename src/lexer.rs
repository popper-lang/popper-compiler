use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier(pub String);

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Number(f64),
    String(String),
    Bool(bool),
}



#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
	If,
	While,
	Let,
	For,
	Else,
    In
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Assign,
    Eq,
    Neq,
    Lt,
    Gt,
    Le,
    Ge,
    And,
    Or
}



#[derive(Debug, Clone, PartialEq)]
pub enum Seperator {
    SemiColon,
    Comma,
    Dot,
    Colon,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket
}

impl FromStr for Keyword {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "if" => Ok(Keyword::If),
            "while" => Ok(Keyword::While),
            "let" => Ok(Keyword::Let),
            "for" => Ok(Keyword::For),
            "else" => Ok(Keyword::Else),
            "in" => Ok(Keyword::In),
            _ => Err(format!("{} is not a keyword", s))
        }
    }
}

impl FromStr for Operator {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operator::Add),
            "-" => Ok(Operator::Sub),
            "*" => Ok(Operator::Mul),
            "/" => Ok(Operator::Div),
            "%" => Ok(Operator::Mod),
            "=" => Ok(Operator::Assign),
            "==" => Ok(Operator::Eq),
            "!=" => Ok(Operator::Neq),
            "<" => Ok(Operator::Lt),
            ">" => Ok(Operator::Gt),
            "<=" => Ok(Operator::Le),
            ">=" => Ok(Operator::Ge),
            "&&" => Ok(Operator::And),
            "||" => Ok(Operator::Or),
            _ => Err(format!("{} is not an operator", s))
        }
    }
}



impl FromStr for Seperator {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ";" => Ok(Seperator::SemiColon),
            "," => Ok(Seperator::Comma),
            "." => Ok(Seperator::Dot),
            ":" => Ok(Seperator::Colon),
            "(" => Ok(Seperator::LeftParen),
            ")" => Ok(Seperator::RightParen),
            "{" => Ok(Seperator::LeftBrace),
            "}" => Ok(Seperator::RightBrace),
            "[" => Ok(Seperator::LeftBracket),
            "]" => Ok(Seperator::RightBracket),
            _ => Err(format!("{} is not a seperator", s))
        }
    }
}

impl FromStr for Literal {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "true" => Ok(Literal::Bool(true)),
            "false" => Ok(Literal::Bool(false)),
            _ => {
                if let Ok(num) = s.parse::<f64>() {
                    Ok(Literal::Number(num))
                } else {
                    Ok(Literal::String(s.to_string()))
                }
            }
        }
    }
}

impl FromStr for Identifier {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 0 {
            Err("Identifier cannot be empty".to_string())
        } else if ! s.chars().next().unwrap().is_alphabetic() {
            Err(format!("{} is not a valid identifier", s))
        } else if s.chars().any(|c| ! c.is_alphanumeric() && c != '_') {
            Err(format!("{} is not a valid identifier", s))
        } else {
            Ok(Identifier(s.to_string()))
        }
    }
}
#[derive(Debug,  Clone, PartialEq)]
pub enum Token {
    // The token enum is a simple state machine with the following states.
    //
    // * `Seperator`: The lexer expects to find  or one
    //   of the symbols `(`, `)`, `{`, `}`, `[`, `]`, `;`, `,`
    //
    // * `Operator`: The lexer expects to find one of the symbols `+`, `-`, `*`, `/`, `%`, `=`, `!`, `<`, `>`, `&`, `|`, `^`, `~`, `?`
    // * `Comment`: The lexer expects to find a `*/` to end the comment.
    //
    // * `String`: The lexer expects to find a `"` to end the string.
    //
    // * `Number`: The lexer expects to find a number.
    //
    // * `Identifier`: The lexer expects to find an identifier.
    //
    // * `Operator`: The lexer expects to find an operator.
    //
    // * `Keyword`: The lexer expects to find a keyword.
    //
    // * `Error`: The lexer expects to find an error.
    //
    // * `Eof`: The lexer expects to find the end of the file.
    Comment(String),
    Literal(Literal),
    Identifier(Identifier),
    Operator(Operator),
    Keyword(Keyword),
    Seperator(Seperator),
    NewLine    
}


impl Token {
	pub fn from_str(string: &str) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        let mut is_char = false;
        let mut chars = "".to_string();
        let mut is_num = false;
        let mut num = "".to_string();
        let mut s;
        for (_, c) in string.chars().enumerate() {
            s = c.to_string();
            if s == " " || s == "\n" || s == "\r" || s == "\t" {
                if is_char {
                    if is_char {
                        if let Ok(k) = Keyword::from_str(&chars) {
                            tokens.push(Token::Keyword(k));
                        } else if let Ok(i) = Identifier::from_str(&chars) {
                            tokens.push(Token::Identifier(i));
                        } else if let Ok(Literal::Bool(b)) = Literal::from_str(&chars) {
                            tokens.push(Token::Literal(Literal::Bool(b)));
                        } 
                        chars = "".to_string();
                        is_char = false;
                    }
                    chars = "".to_string();
                    is_char = false;
                }
                if is_num {
                    
                    if let Ok(n) = Literal::from_str(&num) {
                        tokens.push(Token::Literal(n));
                        num = "".to_string();
                        is_num = false;
                    } else {
                        return Err(format!("{} is not a valid number", num));
                    }
                }
                if s == "\n" {
                    tokens.push(Token::NewLine);
                }
            }
            if let Ok(sep) = Seperator::from_str(&s) {
                if is_char {
                    if is_char {
                        if let Ok(k) = Keyword::from_str(&chars) {
                            tokens.push(Token::Keyword(k));
                        } else if let Ok(i) = Identifier::from_str(&chars) {
                            tokens.push(Token::Identifier(i));
                        } else if let Ok(Literal::Bool(b)) = Literal::from_str(&chars) {
                            tokens.push(Token::Literal(Literal::Bool(b)));
                        } 
                        chars = "".to_string();
                        is_char = false;
                    }
                    chars = "".to_string();
                    is_char = false;
                }
                if is_num {
                    if let Ok(n) = Literal::from_str(&num) {
                        tokens.push(Token::Literal(n));
                        num = "".to_string();
                        is_num = false;
                    } else {
                        return Err(format!("{} is not a valid number", num));
                    }
                }
                tokens.push(Token::Seperator(sep));
            }
            if let Ok(op) = Operator::from_str(&s) {
                if is_char {
                    if let Ok(k) = Keyword::from_str(&chars) {
                        tokens.push(Token::Keyword(k));
                    } else if let Ok(i) = Identifier::from_str(&chars) {
                        tokens.push(Token::Identifier(i));
                    } else if let Ok(Literal::Bool(b)) = Literal::from_str(&chars) {
                        tokens.push(Token::Literal(Literal::Bool(b)));
                    } 
                    chars = "".to_string();
                    is_char = false;
                }
                if is_num {
                    if let Ok(n) = Literal::from_str(&num) {
                        tokens.push(Token::Literal(n));
                        num = "".to_string();
                        is_num = false;
                    } else {
                        return Err(format!("{} is not a valid number", num));
                    }
                }
                tokens.push(Token::Operator(op));
            }
    
            if let Ok(_) = Identifier::from_str(&s) {
                if is_num {
                    return Err("Identifier cannot be a number".to_string());
                    
                } else {
                    chars += &s;
                    is_char = true;
                }
                
            }
            if let Ok(Literal::Number(_)) = Literal::from_str(&s) {
                if is_char {
                    chars += &s;
                } else {
                    num += &s;
                    is_num = true;
                }
            }
        }
        if is_char {
            if let Ok(k) = Keyword::from_str(&chars) {
                tokens.push(Token::Keyword(k));
            } else if let Ok(i) = Identifier::from_str(&chars) {
                tokens.push(Token::Identifier(i));
            } else if let Ok(Literal::Bool(b)) = Literal::from_str(&chars) {
                tokens.push(Token::Literal(Literal::Bool(b)));
            } 
            chars = "".to_string();
            is_char = false;
        }
        if is_num {
            if let Ok(n) = Literal::from_str(&num) {
                tokens.push(Token::Literal(n));
                num = "".to_string();
                is_num = false;
            } else {
                return Err(format!("{} is not a valid number", num));
            }
        }
        return Ok(tokens);
    
    
	}
}