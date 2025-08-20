use std::fmt::Display;
use popper_ast::{
    ast::Span,
    token::{Token, TokenKind},
};
use popper_error_core::Diagnostics;
use popper_error_macro::Diagnostics;
use popper_lexer::error::LexerError;

struct FormatedListString<T: Display>(Vec<T>);

impl <T: Display> Display for FormatedListString<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.0.iter();
        if let Some(first) = iter.next() {
            write!(f, "{}", first)?;
            for item in iter {
                write!(f, ", {}", item)?;
            }
        }
        Ok(())
    }
}


#[derive(Diagnostics, Debug)]
#[message("expected token `{0}` but found `{1}`", FormatedListString(expects_token.clone()), found_token.kind)]
#[code = 3]
#[label = "found token `{found_token}`"]
pub struct UnexpectedToken {
    expects_token: Vec<TokenKind>,
    found_token: Token,

    #[span]
    pub span: Span,
}

impl UnexpectedToken {
    pub fn new(expected: &[TokenKind], found: Token, span: Span) -> Self {
        Self {
            expects_token: expected.to_vec(),
            found_token: found,
            span,
        }
    }
}

#[derive(Diagnostics, Debug)]
#[message("expected a valid attribute but found `{0}`", found_token.kind)]
#[code = 4]
#[label = "found token `{found_token}`"]
pub struct ExpectedValidAttribute {
    found_token: Token,

    #[span]
    pub span: Span,
}

impl ExpectedValidAttribute {
    pub fn new(found: Token, span: Span) -> Self {
        ExpectedValidAttribute {
            found_token: found,
            span
        }
    }
}


#[derive(Debug)]
pub enum ParserError {
    LexerError(LexerError),
    UnexpectedToken(UnexpectedToken),
    ExpectedValidAttribute(ExpectedValidAttribute),
}

impl ParserError {
    pub fn expected_token(expected: &[TokenKind], found: Token, span: Span) -> Self {
        ParserError::UnexpectedToken(UnexpectedToken::new(expected, found, span))
    }
    
    pub fn expected_valid_attribute(found: Token, span: Span) -> Self {
        ParserError::ExpectedValidAttribute(ExpectedValidAttribute::new(found, span))
    }
    
    
    
    pub fn span(&self) -> Span {
        match self {
            ParserError::LexerError(err) => err.span(),
            ParserError::UnexpectedToken(err) => err.span,
            ParserError::ExpectedValidAttribute(err) => err.span,
        }
    }
}

#[macro_export]
macro_rules! parse_error {
    (expect tokens [$($expected:ident),+] but got $found:ident in $span:expr) => {
        $crate::error::ParserError::expected_token(
            &[$(TokenKind::$expected),*],
            $found,
            $span
        )
    };
    (expect tokens [$($expected:ident),+] but got ($found:expr)) => {
        $crate::error::ParserError::expected_token(
            &[$(TokenKind::$expected),*],
            $found.clone(),
            $found.span
        )
    };
    (expect token $expected:ident but got $found:ident in $span:expr) => {
        $crate::error::ParserError::expected_token(
            &[$expected],
            $found,
            $span
        )
    };
    (expect token $expected:ident but got $found:expr) => {
        $crate::error::ParserError::expected_token(
            &[$expected],
            $found,
            $found.span
        )
    };
    (expect a valid attribute but got ($found:ident) in $span:expr) => {
        $crate::error::ParserError::expected_valid_attribute(
            $found,
            $span
        )
    };
    (expect a valid attribute but got ($found:expr)) => {
        $crate::error::ParserError::expected_valid_attribute(
            $found.clone(),
            $found.span
        )
    };
    () => {};
}

impl From<LexerError> for ParserError {
    fn from(value: LexerError) -> Self {
        ParserError::LexerError(value)
    }
}

pub type Result<T> = std::result::Result<T, ParserError>;
