use crate::consts::Ident;

#[derive(Debug, Clone)]
pub struct MarksSection {
    marks: Vec<Mark>
}

impl MarksSection {
    pub fn new() -> Self {
        Self {
            marks: Vec::new()
        }
    }

    pub fn add_mark(&mut self, mark: Mark) {
        self.marks.push(mark);
    }
}

#[derive(Debug, Clone)]
pub struct Mark {
    id: Ident,
    kind: MarkKind
}

impl Mark {
    pub fn new(id: Ident, kind: MarkKind) -> Self {
        Self {
            id,
            kind
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum MarkKind {
    ConstTable
}