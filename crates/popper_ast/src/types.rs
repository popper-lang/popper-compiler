use crate::Span;

#[cfg_attr(feature = "extra-trait", derive(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
pub struct Type {
    pub span: Span,
    pub type_kind: TypeKind,
    pub generics: Vec<Type>,
}

impl Type {
    pub fn new(span: Span, type_kind: TypeKind, generics: Vec<Type>) -> Self {
        Self {
            span,
            type_kind,
            generics,
        }
    }
}

#[cfg_attr(feature = "extra-trait", derive(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
pub enum TypeKind {
    Tuple(Vec<Type>),
    Array(Box<Type>, usize),
    Function(Vec<Type>, Box<Type>),
    Pointer(Box<Type>),
    Reference(Box<Type>),
    Unit,
    Int,
    Float,
    Bool,
    Char,
    String
}