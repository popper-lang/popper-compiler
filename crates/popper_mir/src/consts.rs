use crate::expr::Expr;
use crate::types::Types;

#[derive(Debug, Clone, PartialEq)]
pub enum ConstKind {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    List(Vec<Expr>),
    Null,
}

impl ConstKind {
    pub fn get_type(&self) -> Types {
        match self {
            ConstKind::Int(_) => Types::Int,
            ConstKind::Float(_) => Types::Float,
            ConstKind::Str(s) => Types::String(s.len()),
            ConstKind::Bool(_) => Types::Bool,
            ConstKind::List(l) => {
                let mut ty = Types::Unit;
                for c in l {
                    ty = c.get_type();
                }
                Types::List(Box::new(ty), l.len())
            },
            ConstKind::Null => Types::Unit,
        }
    }
}

impl std::fmt::Display for ConstKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConstKind::Int(i) => write!(f, "{}", i),
            ConstKind::Float(fl) => write!(f, "{}", fl),
            ConstKind::Str(s) => write!(f, "\"{}\"", s),
            ConstKind::Bool(b) => write!(f, "{}", b),
            ConstKind::List(l) => {
                write!(f, "[")?;
                for (i, c) in l.iter().enumerate() {
                    write!(f, "{}", c)?;
                    if i < l.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            },
            ConstKind::Null => write!(f, "null"),
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct Ident {
    index_table: u64,
    ty: Types
}

impl Ident {
    pub fn create(index_table: u64, types: Types) -> Self {
        Self {
            index_table,
            ty:  types
        }
    }

    pub fn new(last: Ident, types: Types) -> Self {
        Self {
            index_table: last.index_table + 1,
            ty: types
        }
    }

    pub fn get_index(&self) -> u64 {
        self.index_table
    }
    
    pub fn get_type(&self) -> Types { 
        self.ty.clone()
    }
    
    pub fn set_type(&mut self, ty: Types) {
        self.ty = ty;
    }

}

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "_{}", self.index_table)
    }
}
