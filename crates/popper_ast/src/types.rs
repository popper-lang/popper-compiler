use crate::Span;

#[cfg_attr(feature = "extra-trait", derive(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
/// a type
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
    /// `(type,*)`
    Tuple(Vec<Type>),
    /// `[type]`
    Array(Box<Type>, usize),
    /// `func(type,*) -> type`
    Function(Vec<Type>, Box<Type>),
    /// `*type`
    Pointer(Box<Type>),
    /// `&type`
    Reference(Box<Type>),
    /// `()`
    Unit,
    /// `int`
    Int,
    /// `float`
    Float,
    /// `bool`
    Bool,
    /// `char`
    Char,
    /// `string`
    String
}