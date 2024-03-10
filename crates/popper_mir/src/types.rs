use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum Types {
    Unit,
    Int,
    Float,
    Bool,
    List(Box<Types>, usize),
    String(usize),
    LLVMPtr,
    Ptr(Box<Types>),
    Label,
}

impl Display for Types {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Types::Unit => write!(f, "unit"),
            Types::Int => write!(f, "int"),
            Types::Bool => write!(f, "bool"),
            Types::Float => write!(f, "float"),
            Types::String(s) => write!(f, "string({})", s),
            Types::List(t, s) => write!(f, "list<{}>({})", t, s),
            Types::LLVMPtr => write!(f, "llvm_ptr"),
            Types::Ptr(t) => write!(f, "ptr<{}>", t),
            Types::Label => write!(f, "label"),
        }

    }
}