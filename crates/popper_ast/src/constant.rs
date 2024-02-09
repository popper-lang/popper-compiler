use crate::{span::Span, Expression};

#[cfg_attr(feature = "extra-trait", derive(Debug, PartialEq, Eq, Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
/// a ident must start by a letter and next by letter, number or `_` charatcter
pub struct Ident {
    pub span: Span,
    pub name: String,
}

impl Ident {
    pub fn new(span: Span, name: String) -> Self {
        Self { span, name }
    }
}

#[cfg_attr(feature = "extra-trait", derive(Debug, PartialEq, Eq, Hash))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
/// any int number: ex `192`, `4920`, `11`
pub struct Int {
    span: Span,
    pub value: i64,
}

impl Int {
    pub fn new(span: Span, value: i64) -> Self {
        Self { span, value }
    }
    pub fn span(&self) -> Span {
        self.span
    }
}

#[cfg_attr(feature = "extra-trait", derive(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
/// any float number: ex `0.38`, `39.21`
pub struct Float {
    span: Span,
    pub value: f64,
}

impl Float {
    fn new(span: Span, value: f64) -> Self {
        Self { span, value }
    }
    pub fn span(&self) -> Span {
        self.span
    }
}

#[cfg_attr(feature = "extra-trait", derive(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
/// any string literal starts by `"` and end by `"`: ex `"hello12"`, `"world__"`
pub struct StringLiteral {
    span: Span,
    pub value: String,
}

impl StringLiteral {
    pub fn new(span: Span, value: String) -> Self {
        Self { span, value }
    }
    pub fn span(&self) -> Span {
        self.span
    }
    pub fn len(&self) -> usize {
        self.value.len()
    }
}

#[cfg_attr(feature = "extra-trait", derive(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
/// true | false
pub struct Bool {
    span: Span,
    pub value: bool,
}

impl Bool {
    pub fn new(span: Span, value: bool) -> Self {
        Self { span, value }
    }
    pub fn span(&self) -> Span {
        self.span
    }
}

#[cfg_attr(feature = "extra-trait", derive(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
pub struct List {
    span: Span,
    pub value: Vec<Expression>,
}

impl List {
    pub fn new(span: Span, value: Vec<Expression>) -> Self {
        Self { span, value }
    }

    pub fn span(&self) -> Span {
        self.span
    }
}

#[cfg_attr(feature = "extra-trait", derive(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
/// null
pub struct Null {
    span: Span,
}

impl Null {
    fn new(span: Span) -> Self {
        Self { span }
    }
    pub fn span(&self) -> Span {
        self.span
    }
}

#[cfg_attr(feature = "extra-trait", derive(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
pub enum Constant {
    Ident(Ident),
    Int(Int),
    Float(Float),
    StringLiteral(StringLiteral),
    List(List),
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
            Self::List(list) => list.span()
        }
    }
}
