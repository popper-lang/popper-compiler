use std::fmt::Display;
use crate::labels::LabelId;
use crate::types::Types;

#[derive(Debug, Clone)]
pub enum Expr {
    Const(crate::consts::ConstKind),
    Ident(crate::consts::Ident),
    Label(LabelId)
}

impl Expr {
    pub fn expect_const(self) -> crate::consts::ConstKind {
        match self {
            Expr::Const(c) => c,
            _ => panic!("Not a const")
        }
    }

    pub fn expect_ident(self) -> crate::consts::Ident {
        match self {
            Expr::Ident(i) => i,
            _ => panic!("Not an ident")
        }
    }

    pub fn expect_label(self) -> LabelId {
        match self {
            Expr::Label(l) => l,
            _ => panic!("Not a label")
        }
    }
    
    pub fn get_type(&self) -> Types {
        match self {
            Expr::Const(c) => c.get_type(),
            Expr::Ident(i) => i.get_type(),
            Expr::Label(_) => Types::Label
        }
    
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Const(c) => write!(f, "{}", c),
            Expr::Ident(i) => write!(f, "{}", i),
            Expr::Label(l) => write!(f, "{}", l),
        }
    }
}