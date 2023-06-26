

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum TypeFlag {
    Bool,
    Int,
    Float,
    String
}

impl TypeFlag {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "bool" => Some(TypeFlag::Bool),
            "int" => Some(TypeFlag::Int),
            "float" => Some(TypeFlag::Float),
            "string" => Some(TypeFlag::String),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            TypeFlag::Bool => "bool".to_string(),
            TypeFlag::Int => "int".to_string(),
            TypeFlag::Float => "float".to_string(),
            TypeFlag::String => "string".to_string(),
        }
    }

    pub fn is_bool(&self) -> bool {
        matches!(self, TypeFlag::Bool)
    }

    pub fn is_int(&self) -> bool {
        matches!(self, TypeFlag::Int)
    }

    pub fn is_float(&self) -> bool {
        matches!(self, TypeFlag::Float)
    }

    pub fn is_string(&self) -> bool {
        matches!(self, TypeFlag::String)
    }

    pub fn is_numeric(&self) -> bool {
        self.is_int() || self.is_float()
    }
}