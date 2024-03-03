use crate::*;

// struct ast
#[cfg_attr(feature = "extra-trait", derive(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
pub struct StructStmt {
    pub name: String,
    pub fields: Vec<StructField>,
    pub span: Span,
}

impl StructStmt {
    pub fn new(name: String, fields: Vec<StructField>, span: Span) -> Self {
        Self { name, fields, span }
    }
}

#[cfg_attr(feature = "extra-trait", derive(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
pub struct StructField {
    pub name: String,
    pub ty: Type,
    pub span: Span,
}

impl StructField {
    pub fn new(name: String, ty: Type, span: Span) -> Self {
        Self { name, ty, span }
    }
}

#[cfg_attr(feature = "extra-trait", derive(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
pub struct StructInstance {
    pub name: String,
    pub fields: Vec<StructFieldInstance>,
    pub span: Span,
}

impl StructInstance {
    pub fn new(name: String, fields: Vec<StructFieldInstance>, span: Span) -> Self {
        Self { name, fields, span }
    }
}

#[cfg_attr(feature = "extra-trait", derive(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
pub struct StructFieldInstance {
    pub name: String,
    pub value: Expression,
    pub span: Span,
}

impl StructFieldInstance {
    pub fn new(name: String, value: Expression, span: Span) -> Self {
        Self { name, value, span }
    }
}

#[cfg_attr(feature = "extra-trait", derive(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
pub struct StructFieldAccess {
    pub name: String,
    pub field: String,
    pub span: Span,
}

impl StructFieldAccess {
    pub fn new(name: String, field: String, span: Span) -> Self {
        Self { name, field, span }
    }
}
