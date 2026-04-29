use crate::ast::TypeDeclKind;
use std::cmp::PartialEq;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fields {
    pub fields: Vec<(String, Type)>,
}

impl Fields {
    pub fn new() -> Self {
        Fields { fields: Vec::new() }
    }

    pub fn first(&self) -> Option<&(String, Type)> {
        self.fields.first()
    }

    pub fn add_field(&mut self, name: String, ty: Type) {
        self.fields.push((name, ty));
    }

    pub fn get_field(&self, name: &str) -> Option<&Type> {
        for (field_name, field_type) in &self.fields {
            if field_name == name {
                return Some(field_type);
            }
        }
        None
    }

    pub fn values(&self) -> impl Iterator<Item = &Type> {
        self.fields.iter().map(|(_, ty)| ty)
    }
}

impl IntoIterator for Fields {
    type Item = (String, Type);
    type IntoIter = std::vec::IntoIter<(String, Type)>;

    fn into_iter(self) -> Self::IntoIter {
        self.fields.into_iter()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int,
    Bool,
    Void,
    Float,
    String,
    Char,
    Ptr(Box<Type>),
    List(Box<Type>, usize), // List type with element type and size
    Function(Vec<Type>, Box<Type>, bool, bool, bool), // Function type with parameters, return type, and is_variadic flag
    AggregateType(String, Fields, TypeDeclKind), // Struct type with field names and their types
    Type(Option<TypeInfo>),
    SelfType,
}

impl Type {
    pub fn is_ptr(&self) -> bool {
        matches!(self, Type::Ptr(_))
    }

    pub fn is_function(&self) -> bool {
        matches!(self, Type::Function(_, _, _, _, _))
    }
    pub fn is_struct(&self) -> bool {
        matches!(self, Type::AggregateType(_, _, TypeDeclKind::Struct))
    }

    pub fn is_union(&self) -> bool {
        matches!(self, Type::AggregateType(_, _, TypeDeclKind::Union))
    }

    pub fn size_of(&self) -> usize {
        match self {
            Type::Int => 4,
            Type::Bool => 1,
            Type::Void => 0,
            Type::Float => 4,
            Type::String => 8, // Assuming a pointer to string data
            Type::Char => 1,
            Type::Ptr(_) => 8,                  // Assuming 64-bit pointers
            Type::Function(_, _, _, _, _) => 8, // Function pointers
            Type::List(ty, count) => ty.size_of() * count,
            Type::Type(_) => 8, // Assuming a pointer to type information
            Type::SelfType => 8,
            Type::AggregateType(_, fields, kind) => {
                let mut size = 0;
                for field_type in fields.values() {
                    size += field_type.size_of();
                }
                if *kind == TypeDeclKind::Union {
                    size = fields.values().map(|t| t.size_of()).max().unwrap_or(0);
                }
                size
            }
        }
    }

    pub fn get_type_info(&self) -> TypeInfo {
        if let Type::Type(Some(type_info)) = self.clone() {
            type_info
        } else {
            match self {
                Type::Bool | Type::Int | Type::Void | Type::Float | Type::String | Type::Char => {
                    TypeInfo {
                        builtin_type: true,
                        name: self.to_string(),
                        ty: Box::new(self.clone()),
                    }
                }
                Type::SelfType => TypeInfo {
                    builtin_type: false,
                    name: "Self".to_string(),
                    ty: Box::new(self.clone()),
                },
                Type::List(..) | Type::Ptr(..) | Type::Function(..) | Type::AggregateType(..) => {
                    TypeInfo {
                        builtin_type: false,
                        name: self.to_string(),
                        ty: Box::new(self.clone()),
                    }
                }
                _ => unreachable!(),
            }
        }
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Int => write!(f, "int"),
            Type::Char => write!(f, "char"),
            Type::Bool => write!(f, "bool"),
            Type::Void => write!(f, "void"),
            Type::Float => write!(f, "float"),
            Type::String => write!(f, "string"),
            Type::Type(_) => write!(f, "type"),
            Type::SelfType => write!(f, "Self"),
            Type::Ptr(inner) => write!(f, "!{:?}", inner),
            Type::Function(params, return_type, is_variadic, ..) => {
                write!(f, "fn(")?;
                for (i, param) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", param)?;
                }
                if *is_variadic {
                    if !params.is_empty() {
                        write!(f, ", ")?;
                    }
                    write!(f, "...")?;
                }
                write!(f, ") -> {}", return_type)
            }
            Type::AggregateType(name, ..) => write!(f, "{}", name),
            Type::List(element_type, count) => write!(f, "[{}; {}]", element_type, count),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeInfo {
    pub builtin_type: bool,
    pub name: String,
    pub ty: Box<Type>,
}
