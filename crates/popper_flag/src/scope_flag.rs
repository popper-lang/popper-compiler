
#[derive(Debug, Default, PartialEq, Clone)]
pub enum ScopeFlag {
    #[default] Global,
    Local,
    Function,
    Block
}

impl ScopeFlag {
    pub fn is_global(&self) -> bool {
        matches!(self, ScopeFlag::Global)
    }

    pub fn is_local(&self) -> bool {
        matches!(self, ScopeFlag::Local)
    }

    pub fn is_function(&self) -> bool {
        matches!(self, ScopeFlag::Function)
    }

    pub fn is_block(&self) -> bool {
        matches!(self, ScopeFlag::Block)
    }
}

