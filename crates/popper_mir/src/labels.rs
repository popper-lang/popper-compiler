use std::fmt::Display;
use crate::stmt::Statement;

#[derive(Debug, Clone)]
pub struct Label {
    id: LabelId,
    stmts: Vec<Statement>,
}

impl Label {
    pub fn create() -> Self {
        Self {
            id: LabelId::new(0),
            stmts: Vec::new(),
        }
    }

    pub fn new(last: Label) -> Self {
        Self {
            id: LabelId::new(last.id.id + 1),
            stmts: Vec::new(),
        }
    }

    pub fn add_stmt(&mut self, stmt: Statement) {
        self.stmts.push(stmt);
    }

    pub fn get_stmts(&self) -> &Vec<Statement> {
        &self.stmts
    }

    pub fn get_id(&self) -> &LabelId {
        &self.id
    }
}

#[derive(Debug, Copy, Clone)]
pub struct LabelId {
    id: usize,
}


impl LabelId {
    pub fn new(id: usize) -> Self {
        Self {
            id
        }
    }
}

impl Display for LabelId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[L{}]", self.id)
    }
}