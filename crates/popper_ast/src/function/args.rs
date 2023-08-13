use crate::Type;
use crate::Span;

#[cfg_attr(feature = "extra-trait", derive(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
pub struct Argument {
    pub name: String,
    pub ty: Type,
    pub span: Span
}

impl Argument {
    pub fn new(name: String, ty: Type, span: Span) -> Self {
        Self {
            name,
            ty,
            span
        }
    }
}

#[cfg_attr(feature = "extra-trait", derive(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
pub struct Arguments {
    pub args: Vec<Argument>,
    pub argument_total: usize,
    pub span: Span
}

impl Arguments {
    pub fn new(args: Vec<Argument>, span: Span) -> Self {
        Self {
            args: args.clone(),
            argument_total: args.len(),
            span
        }
    }
}