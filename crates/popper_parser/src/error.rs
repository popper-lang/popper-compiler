use popper_ast::Span;


#[derive(Debug, Clone, PartialEq)]
pub struct Error {
    pub error: ErrorType,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorType {
    UnexpectedToken(String),
    UnexpectedEOF,
    ExpectedToken {
        expected: String,
        got: String,
    },
    CanBe(Vec<String>)
}
