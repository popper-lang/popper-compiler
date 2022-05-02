

pub enum Keyword {
	If,
	While,
	Let,
	For,
	Else
}

pub struct Integrer;





impl FromStr for Keyword {
    type Err = String;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"if" => Ok(Keyword::If),
			"else" => Ok(Keyword::Else),
			"let" => Ok(Keyword::Let),
			"for" => Ok(Keyword::For),
			_ => Err(format!("{s} is not a valid keyword"))
		}
	}


#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // The lexer is a simple state machine with the following states.
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
    Number(String),
    Identifier(String),
    Operator(String),
    Keyword(Keyword),
    String(String),
    Bool(String),
    Error(String),
    Seperator(String),
    Eof
    }

impl Token {
    pub fn check_for_number(input: &str) -> bool {
        input.to_string().parse::<f64>().is_ok()
    }
    pub fn check_for_identifier(input: &str) -> bool {
        let chars = input.chars().collect::<Vec<char>>();
        if input.len() == 0 {
                return false;
            }
            chars[0].is_alphabetic() || chars[0] == '_'

        }
    
    pub fn check_for_bool(input: &str) -> bool {
        input.to_string().parse::<bool>().is_ok()
    }
    pub fn check_for_operator(input: &str) -> bool {
        if input.len() > 1 {
            return false;
        }
        match input {
            "+"| "-" | "*" | "/" | "%"  => true,
            "=" | "!" | "<" | ">" | "&" | "|" | "^" | "~" | "?" => true,
            _ => return false
        }
    }

    

    pub fn check_for_seperator(input: &str) -> bool {
        match input {
            "(" | ")" | "{" | "}" | "[" | "]" | ";" | "," => true,
            _ => false
        }
    }

}

impl FromStr for Token {
	type Err = String; 
	fn from_str(string: &str) -> Result<Vec<Self>, Self::Err> {
		
	}
}



pub fn tokenize(string: &str) -> Vec<Lexer> {
    let mut tokens = Vec::new();
    let mut is_char = false;
    let mut chars = "".to_string();
    let mut is_num = false;
    let mut num = "".to_string();
    let mut s;
    for (i, c) in string.chars().enumerate() {
        s = c.to_string();
        if s == " " || s == "\n" || s == "\r" || s == "\t" {
            if is_char {
                if Keyword::from_str(chars) {
                    tokens.push(Lexer::Keyword(chars));
                } else if Lexer::check_for_identifier(&chars) {
                    tokens.push(Lexer::Identifier(chars));
                } else if Lexer::check_for_bool(&chars) {
                    tokens.push(Lexer::Bool(chars));
                } else {
                    tokens.push(Lexer::Error(chars));
                }
                chars = "".to_string();
                is_char = false;
            }
            if is_num {
                tokens.push(Lexer::Number(num));
                num = "".to_string();
                is_num = false;
            }
        }
        if Lexer::check_for_seperator(&s) {
            if is_char {
                if Lexer::check_for_keyword(&chars) {
                    tokens.push(Lexer::Keyword(chars));
                } else if Lexer::check_for_identifier(&chars) {
                    tokens.push(Lexer::Identifier(chars));
                } else if Lexer::check_for_bool(&chars) {
                    tokens.push(Lexer::Bool(chars));
                } else {
                    tokens.push(Lexer::Error(chars));
                }
                chars = "".to_string();
                is_char = false;
            }
            if is_num {
                tokens.push(Lexer::Number(num));
                num = "".to_string();
                is_num = false;
            }
            tokens.push(Lexer::Seperator(s.clone()));
        }
        if Lexer::check_for_operator(&s) {
            if is_char {
                if Lexer::check_for_keyword(&chars) {
                    tokens.push(Lexer::Keyword(chars));
                } else if Lexer::check_for_identifier(&chars) {
                    tokens.push(Lexer::Identifier(chars));
                } else if Lexer::check_for_bool(&chars) {
                    tokens.push(Lexer::Bool(chars));
                } else {
                    tokens.push(Lexer::Error(chars));
                }
                chars = "".to_string();
                is_char = false;
            }
            if is_num {
                tokens.push(Lexer::Number(num));
                num = "".to_string();
                is_num = false;
            }
            tokens.push(Lexer::Operator(s.clone()));
        }

        if Lexer::check_for_identifier(&s)  {
            if is_num {
                tokens.push(Lexer::Error("".to_string()));
                
            } else {
                chars += &s;
                is_char = true;
            }
            
        }
        if Lexer::check_for_number(&s) {
            if is_char {
                chars += &s;
            } else {
                num += &s;
                is_num = true;
            }
        }
    }
    if is_char {
        if Lexer::check_for_keyword(&chars) {
            tokens.push(Lexer::Keyword(chars));
        } else if Lexer::check_for_identifier(&chars) {
            tokens.push(Lexer::Identifier(chars));
        } else if Lexer::check_for_bool(&chars) {
            tokens.push(Lexer::Bool(chars));
        } else {
            tokens.push(Lexer::Error(chars));
        }
        chars = "".to_string();
        is_char = false;
    }
    if is_num {
        tokens.push(Lexer::Number(num));
        num = "".to_string();
        is_num = false;
    }
    return tokens;



}