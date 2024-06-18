use popper_ast::{Type, TypeKind};
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Clone, Debug)]
pub enum ValueFlag {
    Integer,
    Float,
    String(u32),
    Boolean,
    None,
    List(Box<ValueFlag>, usize),
    Function(Vec<ValueFlag>, Box<ValueFlag>, bool),
    Struct(String),
    StructInstance(String),
    Pointer(Box<ValueFlag>),
    Module(HashMap<String, String>),
}

impl Display for ValueFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueFlag::Integer => write!(f, "int"),
            ValueFlag::Float => write!(f, "float"),
            ValueFlag::String(len) => write!(f, "string:{}", len),
            ValueFlag::Boolean => write!(f, "bool"),
            ValueFlag::None => write!(f, "unit"),
            ValueFlag::List(t, u) => write!(f, "[{}: {}]", t, u),
            ValueFlag::Function(args, returntype, is_var_args) => {
                let mut args_string = String::new();
                for arg in args {
                    args_string.push_str(&arg.to_string());
                    args_string.push_str(", ");
                }
                args_string.pop();
                args_string.pop();
                if *is_var_args {
                    args_string.push_str("...");
                }
                write!(f, "func({}): {}", args_string, returntype)
            }
            ValueFlag::Pointer(ptr) => {
                write!(f, "*{}", ptr)
            }
            ValueFlag::Struct(name) => {
                write!(f, "struct({})", name)
            }

            ValueFlag::StructInstance(name) => write!(f, "struct({})", name),
            ValueFlag::Module(hash) => write!(f, "module({:?})", hash),
        }
    }
}

impl ValueFlag {
    pub fn from_ty_kind(ty: TypeKind) -> Self {
        match ty {
            TypeKind::String(len) => ValueFlag::String(len),
            TypeKind::Bool => ValueFlag::Boolean,
            TypeKind::Int => ValueFlag::Integer,
            TypeKind::Unit => ValueFlag::None,
            TypeKind::List(ty, l) => ValueFlag::List(Box::new(Self::from_ty(*ty)), l),
            TypeKind::Function(args, ret, var) => ValueFlag::Function(
                args.iter().cloned().map(Self::from_ty).collect(),
                Box::new(Self::from_ty(*ret)),
                var,
            ),
            TypeKind::Struct(name) => {
                ValueFlag::Struct(name)
            }
            TypeKind::StructInstance(name) => ValueFlag::StructInstance(name),
            TypeKind::Pointer(ptr) => ValueFlag::Pointer(Box::new(Self::from_ty(*ptr))),
            _ => unimplemented!(),
        }
    }

    pub fn from_ty(ty: Type) -> Self {
        Self::from_ty_kind(ty.type_kind)
    }

    pub fn get_minor_type(&self) -> Option<&ValueFlag> {
        match self {
            ValueFlag::List(ty, _) => Some(ty),
            ValueFlag::Pointer(ty) => Some(ty),
            _ => None,
        }
    }
    
    

    pub fn is_same(&self, other: &Self) -> bool {
        self == other
    }
    
    pub fn is_static(&self) -> bool {
        match self {
            ValueFlag::Struct(_) => true,
            ValueFlag::Module(_) => true,
            ValueFlag::Function(..) => true,
            ValueFlag::StructInstance(_) => true,
            _ => false,
        }
    }
}

impl PartialEq for ValueFlag {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ValueFlag::Integer, ValueFlag::Integer) => true,
            (ValueFlag::Float, ValueFlag::Float) => true,
            (ValueFlag::String(_), ValueFlag::String(_)) => true,
            (ValueFlag::Boolean, ValueFlag::Boolean) => true,
            (ValueFlag::None, ValueFlag::None) => true,
            (ValueFlag::List(ty1, len1), ValueFlag::List(ty2, len2)) => ty1 == ty2 && len1 == len2,
            (ValueFlag::Function(args1, ret1, a1), ValueFlag::Function(args2, ret2, a2)) => {
                args1 == args2 && ret1 == ret2 && a1 == a2
            }
            (ValueFlag::Struct(fields1), ValueFlag::Struct(fields2)) => fields1 == fields2,
            (ValueFlag::StructInstance(name1), ValueFlag::StructInstance(name2)) => name1 == name2,
            (ValueFlag::Module(hash1), ValueFlag::Module(hash2)) => hash1 == hash2,
            (ValueFlag::Pointer(ty1), ValueFlag::Pointer(ty2)) => ty1 == ty2,
            _ => false,
        }
    }
}
