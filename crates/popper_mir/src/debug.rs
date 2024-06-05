use crate::consts::Ident;

#[derive(Debug, Clone)]
pub struct DebugSection {
    debug_table: Vec<DebugEntry>
}

impl DebugSection {
    pub fn new() -> Self {
        Self {
            debug_table: Vec::new()
        }
    }

    pub fn add_entry(&mut self, entry: DebugEntry) {
        self.debug_table.push(entry);
    }
    pub fn remove_entry(&mut self, id: Ident) {
        self.debug_table.retain(|x| x.id != id);
    }

    pub fn get_debug_info(&self, id: &Ident) -> Option<&VarDebugKind> {
        for entry in &self.debug_table {
            if &entry.id == id {
                return Some(&entry.kind);
            }
        }
        None
    }

    pub fn get_mut_debug_info(&mut self, id: &Ident) -> Option<&mut VarDebugKind> {
        for entry in &mut self.debug_table {
            if &entry.id == id {
                return Some(&mut entry.kind);
            }
        }
        None
    }

    pub fn get_all_debug_info(&self) -> &Vec<DebugEntry> {
        &self.debug_table
    }

    pub fn set_uses(&mut self, id: Ident, uses: i64) {
        if let Some(entry) = self.get_mut_debug_info(&id) {
            if let VarDebugKind::Use(n) = entry {
                *n = uses;
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct DebugEntry {
    pub id: Ident,
    pub kind: VarDebugKind
}

impl DebugEntry {
    pub fn new(id: Ident, kind: VarDebugKind) -> Self {
        Self {
            id,
            kind
        }
    }
}

#[derive(Debug, Clone)]
pub enum VarDebugKind {
    Var(String),
    Internal,
    Use(i64)
}

impl VarDebugKind {
    pub fn get_var_name(&self) -> Option<&str> {
        match self {
            VarDebugKind::Var(name) => Some(name),
            _ => None
        }
    }

    pub fn get_use(&self) -> Option<i64> {
        match self {
            VarDebugKind::Use(use_id) => Some(*use_id),
            _ => None
        }
    }

    pub fn is_internal(&self) -> bool {
        matches!(self, VarDebugKind::Internal)
    }
}
