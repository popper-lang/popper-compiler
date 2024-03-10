use crate::consts::Ident;
use crate::debug::{DebugEntry, DebugSection, VarDebugKind};
use crate::labels::Label;
use crate::marks::MarksSection;
use crate::stmt::Statement;
use crate::types::Types;

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub args: Vec<Types>,
    pub stmts: Vec<Statement>,
    pub ret: Types,
    pub dbg: DebugSection,
    pub marks: MarksSection,
    pub labels: Vec<Label>
}

impl Function {
    pub fn set_debug_info(&mut self, id: Ident, dbg_info: VarDebugKind) {
        self.dbg.add_entry(DebugEntry::new(id, dbg_info));
    }
    
    pub fn remove_debug_info(&mut self, id: Ident) {
        self.dbg.remove_entry(id);
    }
}

impl Function {
    pub fn new(name: String, args: Vec<Types>, ret: Types) -> Self {
        Self {
            name,
            args,
            stmts: Vec::new(),
            ret,
            dbg: DebugSection::new(),
            marks: MarksSection::new(),
            labels: Vec::new()
        }
    }

    pub fn add_stmt(&mut self, stmt: Statement) {
        self.stmts.push(stmt);
    }

    pub fn add_label(&mut self, label: Label) {
        self.labels.push(label);
    }
}