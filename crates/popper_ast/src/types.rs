use crate::Span;
use std::{collections::HashMap, fmt::Display};

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
    List(Box<Type>, usize),
    /// `func(type,*) : type`
    Function(Vec<Type>, Box<Type>, bool),
    /// `*type`
    Pointer(Box<Type>),
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
    String(u32), // u32: size of string
    Struct(String),
    /// `struct name
    StructInstance(String),
}

impl Display for TypeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeKind::Tuple(types) => {
                write!(f, "(")?;
                for (i, ty) in types.iter().enumerate() {
                    write!(f, "{}", ty.type_kind)?;
                    if i != types.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")")
            }
            TypeKind::List(ty, size) => write!(f, "[{}; {}]", ty.type_kind, size),
            TypeKind::Function(args, ret, var_args) => {
                write!(f, "func(")?;
                for (i, ty) in args.iter().enumerate() {
                    write!(f, "{}", ty.type_kind)?;
                    if i != args.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ") : {}", ret.type_kind)?;
                if *var_args {
                    write!(f, "...")
                } else {
                    Ok(())
                }
            }
            TypeKind::Pointer(ty) => write!(f, "*{}", ty.type_kind),
            TypeKind::Unit => write!(f, "()"),
            TypeKind::Int => write!(f, "int"),
            TypeKind::Float => write!(f, "float"),
            TypeKind::Bool => write!(f, "bool"),
            TypeKind::Char => write!(f, "char"),
            TypeKind::String(size) => write!(f, "string[{}]", size),
            TypeKind::Struct(name) => {
                write!(f, "struct {}", name)
            }
            TypeKind::StructInstance(name) => write!(f, "struct {}", name),
        }

    }
}
