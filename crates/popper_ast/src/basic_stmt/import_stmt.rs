use crate::Span;
use crate::Ident;

#[cfg_attr(feature = "extra-trait", derive(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
pub struct ImportStmt {
    pub path: PathImport,
    pub span: Span,
}

impl ImportStmt {
    pub fn new(span: Span, path: PathImport) -> Self {
        Self { span, path }
    }

    pub fn span(&self) -> Span {
        self.span
    }
}


#[cfg_attr(feature = "extra-trait", derive(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
pub struct PathImport {
    pub segments: Vec<Ident>,
    pub span: Span,
}

impl PathImport {
    pub fn new(span: Span, segments: Vec<Ident>) -> Self {
        Self { span, segments }
    }

    pub fn span(&self) -> Span {
        self.span
    }
}