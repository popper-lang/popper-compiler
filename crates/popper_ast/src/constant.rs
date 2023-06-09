use crate::span::Span;

#[derive(Debug, Clone)]
pub struct Ident {
    span: Span,
    pub name: String,
}

impl Ident {
    pub fn new(span: Span, name: String) -> Self {
        Self { span, name }
    }
}

#[derive(Debug, Clone)]
pub struct Int {
    span: Span,
    pub value: i64,
}

impl Int {
    pub fn new(span: Span, value: i64) -> Self {
        Self { span, value }
    }
}

#[derive(Debug, Clone)]
pub struct Float {
    span: Span,
    pub value: f64,
}

impl Float {
    fn new(span: Span, value: f64) -> Self {
        Self { span, value }
    }
}

#[derive(Debug, Clone)]
pub struct StringLiteral {
    span: Span,
    pub value: String,
}

impl StringLiteral {
    fn new(span: Span, value: String) -> Self {
        Self { span, value }
    }
}

#[derive(Debug, Clone)]
pub struct Bool {
    span: Span,
    pub value: bool,
}

impl Bool {
    pub fn new(span: Span, value: bool) -> Self {
        Self { span, value }
    }
}

#[derive(Debug, Clone)]
pub struct Null {
    span: Span,
}

impl Null {
    fn new(span: Span) -> Self {
        Self { span }
    }
}

#[derive(Debug, Clone)]
pub enum Constant {
    Ident(Ident),
    Int(Int),
    Float(Float),
    StringLiteral(StringLiteral),
    Bool(Bool),
    Null(Null),
}

impl Constant {
    pub fn span(&self) -> Span {
        match self {
            Self::Ident(ident) => ident.span,
            Self::Int(int) => int.span,
            Self::Float(float) => float.span,
            Self::StringLiteral(string_literal) => string_literal.span,
            Self::Bool(bool) => bool.span,
            Self::Null(null) => null.span,
        }
    }
}