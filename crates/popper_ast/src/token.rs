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

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Clone, PartialEq)]
pub enum TokenKind {
    Identifier,
    Number,
    String,
    Char,
    Symbol,
    KeywordLet,
    KeywordIf,
    KeywordElse,
    KeywordFunc,
    KeywordReturn,
    TypeInt,
    TypeFloat,
    TypeBool,
    TypeString,
    TypeVoid,
    ParenL,
    ParenR, 
    BraceL, 
    BraceR,
    BracketL,
    BracketR,
    Whitespace,
    Comment,
    Comma,
    Dot,
    Colon,
    Semicolon,
    Arrow,
    Eq,
    Plus,
    Minus,
    Multiply,
    Divide,
    Newline,
    Eof,
}

impl std::fmt::Debug for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            TokenKind::Eof => "<eof>",
            TokenKind::Char => "<char>",
            TokenKind::Newline => "<newline>",
            TokenKind::Comment => "<comment>",
            TokenKind::Whitespace => "<whitespace>",
            TokenKind::KeywordLet => "<keyword let>",
            TokenKind::KeywordIf => "<keyword if>",
            TokenKind::KeywordElse => "<keyword else>",
            TokenKind::KeywordFunc => "<keyword func>",
            TokenKind::KeywordReturn => "<keyword return>",
            TokenKind::TypeInt => "<type int>",
            TokenKind::TypeFloat => "<type float>",
            TokenKind::TypeBool => "<type bool>",
            TokenKind::TypeString => "<type string>",
            TokenKind::TypeVoid => "<type void>",
            TokenKind::Symbol => "<symbol>",
            TokenKind::String => "<string>",
            TokenKind::Number => "<number>",
            TokenKind::Identifier => "<ident>",
            TokenKind::Comma => "<comma>",
            TokenKind::Dot => "<dot>",
            TokenKind::Colon => "<colon>",
            TokenKind::Arrow => "<arrow>",
            TokenKind::Eq => "<eq>",
            TokenKind::Plus => "<plus>",
            TokenKind::Minus => "<minus>",
            TokenKind::Multiply => "<multiply>",
            TokenKind::Divide => "<divide>",
            TokenKind::Semicolon => "<semicolon>",
            TokenKind::ParenL => "<paren left>",
            TokenKind::ParenR => "<paren right>",
            TokenKind::BraceL => "<brace left>",
            TokenKind::BraceR => "<brace right>",
            TokenKind::BracketL => "<bracket left>",
            TokenKind::BracketR => "<bracket right>",
        };

        write!(f, "{}", s)
    }
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            TokenKind::KeywordLet => "let",
            TokenKind::KeywordIf => "if",
            TokenKind::KeywordElse => "else",
            TokenKind::KeywordFunc => "func",
            TokenKind::KeywordReturn => "return",
            TokenKind::TypeInt => "int",
            TokenKind::TypeFloat => "float",
            TokenKind::TypeBool => "bool",
            TokenKind::TypeString => "string",
            TokenKind::TypeVoid => "void",
            TokenKind::Comma => ",",
            TokenKind::Eq => "=",
            TokenKind::Semicolon => ";",
            TokenKind::Dot => ".",
            TokenKind::Colon => ":",
            TokenKind::Arrow => "->",
            TokenKind::Plus => "+",
            TokenKind::Minus => "-",
            TokenKind::Multiply => "*",
            TokenKind::Divide => "/",
            TokenKind::ParenL => "(",
            TokenKind::ParenR => ")",
            TokenKind::BraceL => "{",
            TokenKind::BraceR => "}",
            TokenKind::BracketL => "[",
            TokenKind::BracketR => "]",
            _ => {
                return write!(f, "{:?}", self);
            }
        };

        write!(f, "{}", s)
    }
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
