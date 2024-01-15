use std::collections::HashMap;
use std::fmt::Display;
use popper_ast::{Type, TypeKind};


#[derive(Clone, Debug)]
pub enum ValueFlag {
    Integer,
    Float,
    String(u32),
    Boolean,
    None,
    Array(Box<ValueFlag>),
    Function(Vec<ValueFlag>, Box<ValueFlag>),
    Struct(HashMap<String, ValueFlag>),
    StructInstance(String),
    Pointer(Box<ValueFlag>),
    Module(HashMap<String, String>)
}

impl Display for ValueFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueFlag::Integer => write!(f, "int"),
            ValueFlag::Float => write!(f, "float"),
            ValueFlag::String(len) => write!(f, "string:{}", len),
            ValueFlag::Boolean => write!(f, "bool"),
            ValueFlag::None => write!(f, "unit"),
            ValueFlag::Array(t) => write!(f, "[{}]", t.to_string()),
            ValueFlag::Function(args, returntype) => {
                let mut args_string = String::new();
                for arg in args {
                    args_string.push_str(&arg.to_string());
                    args_string.push_str(", ");
                }
                args_string.pop();
                args_string.pop();
                write!(f, "func({}): {}", args_string, returntype.to_string())
            },
            ValueFlag::Pointer(ptr) => {
                write!(f, "*{}", ptr)
            },
            ValueFlag::Struct(fields) => {
                let mut fields_string = String::new();
                for (name, ty) in fields {
                    fields_string.push_str(&format!("{}: {}, ", name, ty));
                }
                fields_string.pop();
                fields_string.pop();
                write!(f, "struct({})", fields_string)
            },

            ValueFlag::StructInstance(name) => write!(f, "struct({})", name),
            ValueFlag::Module(hash) => write!(f, "module({:?})", hash)
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
            TypeKind::Array(ty, _) => ValueFlag::Array(
                Box::new(
                    Self::from_ty(*ty)
                )
            ),
            TypeKind::Function(args, ret) => ValueFlag::Function(args
                .iter()
                .cloned()
                .map(Self::from_ty).collect(),
                                                                 Box::new(Self::from_ty(*ret))
            ),
            TypeKind::Struct(fields) => {
                let mut hashmap = HashMap::new();
                for (name, ty) in fields {
                    hashmap.insert(name, Self::from_ty(ty));
                }
                ValueFlag::Struct(hashmap)
            },
            TypeKind::StructInstance(name) => {
                ValueFlag::StructInstance(name)
            },
            _ => unimplemented!()

        }
    }

    pub fn from_ty(ty: Type) -> Self {
        Self::from_ty_kind(ty.type_kind)
    }

    pub fn get(&self, name: &str) -> Option<&ValueFlag> {
        match self {
            ValueFlag::Struct(fields) => fields.get(name),
            _ => None
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
            (ValueFlag::Array(ty1), ValueFlag::Array(ty2)) => ty1 == ty2,
            (ValueFlag::Function(args1, ret1), ValueFlag::Function(args2, ret2)) => {
                args1 == args2 && ret1 == ret2
            },
            (ValueFlag::Struct(fields1), ValueFlag::Struct(fields2)) => fields1 == fields2,
            (ValueFlag::StructInstance(name1), ValueFlag::StructInstance(name2)) => name1 == name2,
            (ValueFlag::Module(hash1), ValueFlag::Module(hash2)) => hash1 == hash2,
            _ => false
        }
    }
}