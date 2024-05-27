use std::fmt::Display;
use crate::consts::Ident;

#[derive(Debug, Clone)]
pub struct MarksSection {
    marks: Vec<Mark>
}

impl MarksSection {
    pub fn get_all_marks(&self) -> &Vec<Mark> {
        &self.marks
    }
    
    pub fn contains_mark(&self, ident: &Ident, mark: MarkKind) -> bool {
        self.marks.iter().any(|m| m.kind == mark && &m.id == ident)
    }
}

impl MarksSection {
    pub fn new() -> Self {
        Self {
            marks: Vec::new()
        }
    }

    pub fn add_mark(&mut self, mark: Mark) {
        if self.marks.contains(&mark) {
            return;
        }
        self.marks.push(mark);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Mark {
    pub id: Ident,
    pub kind: MarkKind
}

impl Mark {
    pub fn new(id: Ident, kind: MarkKind) -> Self {
        Self {
            id,
            kind
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MarkKind {
    ConstTable
}

impl Display for MarkKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MarkKind::ConstTable => write!(f, "ConstTable")
        }
    }
}