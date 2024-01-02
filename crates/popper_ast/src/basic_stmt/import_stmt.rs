use crate::{Span, Statement};
use crate::Ident;

#[cfg_attr(feature = "extra-trait", derive(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
pub struct ImportStmt {
    pub path: PathImport,
    pub module_stmts: Vec<Statement>, // module_stmts is the statements of the imported module
    pub span: Span,
}

impl ImportStmt {
    pub fn new(span: Span, path: PathImport, module_stmts: Vec<Statement>) -> Self {
        Self { span, path, module_stmts }
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

#[cfg(feature = "extra-trait")]
impl std::fmt::Display for PathImport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut path = String::new();
        for segment in &self.segments {
            path.push_str(&segment.name);
            path.push_str("::");
        }
        write!(f, "{}", path)
    }
}