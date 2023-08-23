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
    /// `func(type,*) : type`
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

impl ToString for TypeKind {
    fn to_string(&self) -> String {
        match self.clone() {
            TypeKind::Tuple(tys) =>
                format!(
                    "({})",
                    tys
                        .iter()
                        .map(|ty| ty.type_kind.to_string())
                        .collect::<Vec<String>>()
                        .join(",")
                )
            ,
            TypeKind::Array(ty, size) =>
                format!("[{}:{}]", ty.type_kind.to_string().clone(), size)
            ,
            TypeKind::Function(tys, ret) =>
                format!("func({}): {}",
                        tys
                            .iter()
                            .map(|t| t.type_kind.to_string())
                            .collect::<Vec<String>>()
                            .join(","),
                        ret.type_kind.to_string()
                )
            ,
            TypeKind::Pointer(ty) =>
                format!("*{}", ty.type_kind.to_string())
            ,
            TypeKind::Reference(ty) =>
                format!("&{}", ty.type_kind.to_string())
            ,
            TypeKind::Unit => String::from("()"),
            TypeKind::Int => String::from("int"),
            TypeKind::Float => String::from("float"),
            TypeKind::Bool => String::from("bool"),
            TypeKind::Char => String::from("char"),
            TypeKind::String => String::from("string")
        }
    }
}