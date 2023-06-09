use std::collections::HashMap;


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

    pub fn new_with_parent(parent: SymbolTable) -> Self {
        Self {
            symbols: HashMap::new(),
            parent: Some(Box::new(parent)),
        }
    }

    pub fn insert(&mut self, symbol: Symbol) {
        self.symbols.insert(symbol.name.clone(), symbol);
    }

    pub fn get(&self, name: &str) -> Option<&Symbol> {
        match self.symbols.get(name) {
            Some(symbol) => Some(symbol),
            None => {
                match &self.parent {
                    Some(parent) => parent.get(name),
                    None => None,
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Symbol {
    name: String,
    symbol_type: SymbolType,
    scope: Scope,
}

#[derive(Debug, Clone)]
pub enum SymbolType {
    Variable,
    Function,
    Constant(ConstantType),
    Lambda,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Scope {
    Local,
    Global,
    Function,
    Module
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConstantType {
    Integer,
    Float,
    String,
    Boolean,
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Flag {
    Type(Type),
    Scope(Scope),
    Constant(ConstantType),
    Identifier,

}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Integer,
    Float,
    String,
    Boolean,
    None,
}

#[derive(Debug, Clone)]
pub struct SymbolFlags(Vec<Flag>);

impl SymbolFlags {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add_flag(&mut self, flag: Flag) {
        self.0.push(flag);
    }

    pub fn has_flag(&self, flag: Flag) -> bool {
        self.0.contains(&flag)
    }

    pub fn set_integer(&mut self) -> &mut Self {
        self.add_flag(Flag::Type(Type::Integer));
        self
    }

    pub fn set_float(&mut self) -> &mut Self {
        self.add_flag(Flag::Type(Type::Float));
        self
    }

    pub fn set_string(&mut self) -> &mut Self {
        self.add_flag(Flag::Type(Type::String));
        self
    }

    pub fn set_boolean(&mut self) -> &mut Self {
        self.add_flag(Flag::Type(Type::Boolean));
        self
    }

    pub fn set_none(&mut self) -> &mut Self {
        self.add_flag(Flag::Type(Type::None));
        self
    }

    pub fn set_ident(&mut self) -> &mut Self {
        self.add_flag(Flag::Identifier);
        self
    }

    pub fn set_local(&mut self) -> &mut Self {
        self.add_flag(Flag::Scope(Scope::Local));
        self
    }

    pub fn set_global(&mut self) -> &mut Self {
        self.add_flag(Flag::Scope(Scope::Global));
        self
    }

    pub fn set_function(&mut self) -> &mut Self {
        self.add_flag(Flag::Scope(Scope::Function));
        self
    }

    pub fn set_module(&mut self) -> &mut Self {
        self.add_flag(Flag::Scope(Scope::Module));
        self
    }

    pub fn set_constant(&mut self, constant_type: ConstantType) -> &mut Self {
        self.add_flag(Flag::Constant(constant_type));
        self
    }

    pub fn is_same_type(&self, other: &Self) -> bool {
        self.0.iter().any(|flag| {
            match flag {
                Flag::Type(t) => other.has_flag(Flag::Type(t.clone())),
                _ => false,
            }
        })
    }
}

impl ToString for Type {
    fn to_string(&self) -> String {
        match self {
            Type::Integer => "Integer".to_string(),
            Type::Float => "Float".to_string(),
            Type::String => "String".to_string(),
            Type::Boolean => "Boolean".to_string(),
            Type::None => "None".to_string(),
        }
    }
}