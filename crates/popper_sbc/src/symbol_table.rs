use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SymbolTable {
    symbols: HashMap<String, Symbol>,
    parent: Option<Box<SymbolTable>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            parent: None,
        }
    }

    pub fn new_child(&mut self) -> Self {
        Self {
            symbols: HashMap::new(),
            parent: Some(Box::new(self.clone())),
        }
    }

    pub fn insert(&mut self, symbol: Symbol) {
        self.symbols.insert(symbol.name.clone(), symbol);
    }

    pub fn get(&self, name: &str) -> Option<Symbol> {
        match self.symbols.get(name) {
            Some(symbol) => Some(symbol.clone()),
            None => match &self.parent {
                Some(parent) => parent.get(name),
                None => None,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct Symbol {
    name: String,
    value: SymbolValue,
}

impl Symbol {
    pub fn new(name: String, value: SymbolValue) -> Self {
        Self { name, value }
    }
}

#[derive(Debug, Clone)]
pub enum SymbolValue {
    Variable,
    Function,
    Lambda
}

