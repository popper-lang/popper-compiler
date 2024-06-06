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
    Struct(String, Vec<Types>),
    TypeId(String)
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
            Types::Struct(n, s) => {
                write!(f, "struct {} {{", n)?;
                for i in s {
                    write!(f, "{}, ", i)?;
                }
                write!(f, "}}")
            },
            Types::TypeId(s) => write!(f, "{}", s)
        }

    }
}

impl Types {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Types::Unit => vec![0],
            Types::Int => vec![1],
            Types::Bool => vec![2],
            Types::Float => vec![3],
            Types::String(s) => {
                let mut bytes = vec![4];
                bytes.extend(s.to_be_bytes());
                bytes
            }
            Types::List(t, s) => {
                let mut bytes = vec![5];
                bytes.extend(t.to_bytes());
                bytes.extend(s.to_be_bytes());
                bytes
            }
            Types::LLVMPtr => vec![6],
            Types::Ptr(t) => {
                let mut bytes = vec![7];
                bytes.extend(t.to_bytes());
                bytes
            }
            Types::Label => vec![8],
            Types::Struct(_, s) => {
                let mut bytes = vec![9];
                for i in s {
                    bytes.extend(i.to_bytes());
                }
                bytes
            },
            _ => panic!("Not implemented")
        }
        
        
    }
    
    pub fn into_struct(self) -> (String, Vec<Types>) {
        match self {
            Types::Struct(n, s) => (n, s),
            _ => panic!("Not a struct")
        }
    }
    
    pub fn get_ptr_inner_type(&self) -> Types {
        match self {
            Types::Ptr(t) => *t.clone(),
            _ => panic!("Not a pointer")
        }
    }
}
