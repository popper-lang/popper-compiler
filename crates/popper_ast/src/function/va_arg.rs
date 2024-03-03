use crate::{Span, Type};

#[cfg_attr(feature = "extra-trait", derive(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
pub struct VaArg {
    span: Span,
    pub ty: Type,
}

impl VaArg {
    pub fn new(span: Span, ty: Type) -> Self {
        Self { span, ty }
    }

    pub fn span(&self) -> Span {
        self.span
    }
}
