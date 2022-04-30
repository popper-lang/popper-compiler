



#[derive(Debug, PartialEq, Clone)]
pub enum Lexer {
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
    Keyword(String),
    String(String),
    Error(String),
    Eof
    }

impl Lexer {
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

    pub fn check_for_keyword(input : &str) -> bool {
        match input {
            // check if one of the keywords is in the input
            "if" | "else" | "for" | "while" | "let" | "then" => true,
            _ => false
        }
    }

    pub fn check_for_seperator(input: &str) -> bool {
        match input {
            "(" | ")" | "{" | "}" | "[" | "]" | ";" | "," => true,
            _ => false
        }
    }

}
                
pub fn tokenize(string: &str) -> Vec<Lexer> {
    let mut tokens = Vec::new();
    let buffer = string.to_string();
    let mut tkchar = String::new();
    let mut is_string = false;
    let lenght = buffer.len();
    let mut string_curr;
    let mut d = false;
    for (i, c) in &mut buffer.chars().enumerate() {
        string_curr = c.to_string();
        if d {
             d = false;
            continue
        }
        if is_string {
            if string_curr == "\"" {
                is_string = false;
                tokens.push(Lexer::String(tkchar));
                tkchar = String::new();
                d = true;
                continue
            } else {
                tkchar += &c.to_string();
                continue
            }

        }
        if string_curr == "\"" {
            is_string = true;
            continue
        }

        if i == lenght - 1 {
            tkchar += &string_curr;
            if Lexer::check_for_number(&tkchar)  {
                tokens.push(Lexer::Number(tkchar));
                tkchar = String::new();

            } else if Lexer::check_for_operator(&tkchar) {
                tokens.push(Lexer::Operator(tkchar));
                tkchar = String::new();
            } else if Lexer::check_for_keyword(&tkchar) {
                tokens.push(Lexer::Keyword(tkchar));
                tkchar = String::new();
            } else if Lexer::check_for_identifier(&tkchar) {
                tokens.push(Lexer::Identifier(tkchar));
                tkchar = String::new();
            } else {
                tokens.push(Lexer::Error(tkchar));
                tkchar = String::new();
            }
            break;
        }
        if string_curr == " ".to_string() || string_curr == "\n".to_string() || string_curr == "\r".to_string() {
            if i == 0 {
                continue;
            }

            if Lexer::check_for_number(&tkchar)  {
                tokens.push(Lexer::Number(tkchar));
                tkchar = String::new();
            
            } else if Lexer::check_for_operator(&tkchar) {
                tokens.push(Lexer::Operator(tkchar));
                tkchar = String::new();
            } else if Lexer::check_for_keyword(&tkchar) {
                tokens.push(Lexer::Keyword(tkchar));
                tkchar = String::new();
            } else if Lexer::check_for_identifier(&tkchar) {
                tokens.push(Lexer::Identifier(tkchar));
                tkchar = String::new();
            } else {
                tokens.push(Lexer::Error(tkchar));
                tkchar = String::new();
            }

            continue
        } 
        tkchar += &string_curr;
    }
            
    return tokens;
}



