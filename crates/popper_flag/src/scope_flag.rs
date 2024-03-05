/// Scope flag is used to save the scope
#[derive(Debug, Default, PartialEq, Clone)]
pub enum ScopeFlag {
    #[default]
    Global,
    Local,
    Function,
    Block,
    Loop
}

impl ScopeFlag {
    /// check if it is global
    pub fn is_global(&self) -> bool {
        matches!(self, ScopeFlag::Global)
    }
    /// check if it is local
    pub fn is_local(&self) -> bool {
        matches!(self, ScopeFlag::Local)
    }
    /// check if it is in function
    pub fn is_function(&self) -> bool {
        matches!(self, ScopeFlag::Function)
    }
    /// check if it is in block
    pub fn is_block(&self) -> bool {
        matches!(self, ScopeFlag::Block)
    }

    pub fn is_loop(&self) -> bool {
        matches!(self, ScopeFlag::Loop)
    }
}
