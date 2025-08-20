use popper_ast::ast::Span;
use popper_error_core::Diagnostics;
use popper_error_macro::Diagnostics;

#[derive(Diagnostics, Debug)]
#[code = 1]
#[message = "Unexpected token `{unexpected_token}`"]
#[label = "Unexpected token"]
pub struct UnexpectedToken {
    unexpected_token: String,
    #[span]
    pub span: Span,
}

#[derive(Debug)]
pub enum LexerError {
    UnexpectedToken(UnexpectedToken),
}

impl LexerError {
    pub fn unexpected_token(unexpected_token: String, span: Span) -> Self {
        LexerError::UnexpectedToken(UnexpectedToken {
            unexpected_token,
            span,
        })
    }
}

impl Diagnostics for LexerError {
    fn message(&self) -> String {
        match self {
            LexerError::UnexpectedToken(err) => err.message(),
        }
    }

    fn code(&self) -> u32 {
        match self {
            LexerError::UnexpectedToken(err) => err.code(),
        }
    }

    fn label(&self) -> String {
        match self {
            LexerError::UnexpectedToken(err) => err.label(),
        }
    }

    fn span(&self) -> Span {
        match self {
            LexerError::UnexpectedToken(err) => err.span(),
        }
    }

    fn note(&self) -> Option<String> {
        match self {
            LexerError::UnexpectedToken(err) => err.note(),
        }
    }
}
