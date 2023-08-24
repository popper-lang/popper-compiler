use std::fmt::Debug;
use popper_ast::Argument;
use popper_ast::{Type, TypeKind};
use crate::instr::Bytecode;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    String(ByteStr),
    Boolean(bool),
    Null
}

impl Bytecode for Literal {
    fn to_bytecode(&self) -> Vec<u8> {
        match self {
            Literal::Integer(value) => {
                let mut bytecode = vec![0x01];
                bytecode.extend(value.to_bytecode());
                bytecode
            },
            Literal::Float(value) => {
                let mut bytecode = vec![0x02];
                bytecode.extend(value.to_bytecode());
                bytecode
            },
            Literal::String(value) => {
                let mut bytecode = vec![0x03];
                bytecode.extend(value.to_bytecode());
                bytecode
            },
            Literal::Boolean(value) => {
                let mut bytecode = vec![0x04];
                bytecode.extend(value.to_bytecode());
                bytecode
            },
            Literal::Null => vec![0x05],
        }
    }

    fn from_bytecode(bytecode: Vec<u8>) -> Self {
        match bytecode[0] {
            0x01 => Literal::Integer(i64::from_bytecode(bytecode[1..].to_vec())),
            0x02 => Literal::Float(f64::from_bytecode(bytecode[1..].to_vec())),
            0x03 => Literal::String(ByteStr::from_bytecode(bytecode[1..].to_vec())),
            0x04 => Literal::Boolean(bool::from_bytecode(bytecode[1..].to_vec())),
            0x05 => Literal::Null,
            _ => panic!("Invalid bytecode for Literal"),
        }
    }
}

/// str ptr that can be represented in bytecode
#[derive(Debug, Clone, PartialEq)]
pub struct ByteStr {
    pub str: String
}


impl ByteStr {
    pub fn new(str: String) -> Self {
        Self { str }
    }
}


impl Bytecode for ByteStr {
    fn to_bytecode(&self) -> Vec<u8> {
        self.str.as_bytes().to_vec()
    }

    fn from_bytecode(bytecode: Vec<u8>) -> Self {
        let bytecode = bytecode;
        let str = String::from_utf8(bytecode).expect("Invalid bytecode for ByteStr");
        Self::new(str)
    }
}

impl Bytecode for usize {
    fn to_bytecode(&self) -> Vec<u8> {
        let mut bytecode = vec![];
        bytecode.extend(self.to_le_bytes());
        bytecode
    }

    fn from_bytecode(bytecode: Vec<u8>) -> Self {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&bytecode);
        usize::from_le_bytes(bytes)
    }
}

impl Bytecode for i64 {
    fn to_bytecode(&self) -> Vec<u8> {
        let mut bytecode = vec![];
        bytecode.extend(self.to_le_bytes());
        bytecode
    }

    fn from_bytecode(bytecode: Vec<u8>) -> Self {
        let bytecode = bytecode;
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&bytecode);
        i64::from_le_bytes(bytes)
    }
}

impl Bytecode for f64 {
    fn to_bytecode(&self) -> Vec<u8> {
        let mut bytecode = vec![];
        bytecode.extend(self.to_le_bytes());
        bytecode
    }

    fn from_bytecode(bytecode: Vec<u8>) -> Self {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&bytecode);
        f64::from_le_bytes(bytes)
    }
}

impl Bytecode for bool {
    fn to_bytecode(&self) -> Vec<u8> {
        vec![ if *self { 1 } else { 0 } ]
    }

    fn from_bytecode(bytecode: Vec<u8>) -> Self {
        match bytecode[0] {
            0 => false,
            1 => true,
            _ => panic!("Invalid bytecode for bool"),
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct ByteArg {
    pub name: ByteStr,
    pub ty: ByteType
}

#[derive(Debug, Clone, PartialEq)]
pub enum ByteType {
    Unit,
    Int,
    Float,
    Str,
    Bool,
    Array(Box<ByteType>),
    Fn(Vec<ByteType>, Box<ByteType>)
}

impl ByteArg {
    pub fn new(name: ByteStr, ty: ByteType) -> Self {
        Self {
            name,
            ty
        }
    }

    pub fn from_ast_argument(ast_argument: Argument) -> Self {
        Self {
            name: ByteStr::new(ast_argument.name),
            ty: ByteType::from_ast_type(ast_argument.ty)
        }
    }
}

impl ByteType {
    pub fn from_ast_type(ast_type: Type) -> Self {
        Self::from_ast_type_kind(&ast_type.type_kind)
    }

    pub fn from_ast_type_kind(ast_type_kind: &TypeKind) -> Self {
        match ast_type_kind.clone() {
            TypeKind::Int => ByteType::Int,
            TypeKind::Float => ByteType::Float,
            TypeKind::String => ByteType::Str,
            TypeKind::Bool => ByteType::Bool,
            TypeKind::Array(ty, _) => ByteType::Array(Box::new(ByteType::from_ast_type(*ty))),
            TypeKind::Function(args, ret) => {
                let args = args.into_iter().map(|arg| ByteType::from_ast_type(arg)).collect();
                ByteType::Fn(args, Box::new(ByteType::from_ast_type(*ret)))
            },
            TypeKind::Unit => ByteType::Unit,
            e => todo!("{:?}", e)

        }
    }
}

impl Bytecode for ByteArg {
    fn to_bytecode(&self) -> Vec<u8> {
        let mut bytecode = vec![];
        bytecode.extend(self.name.to_bytecode());
        bytecode.extend(self.ty.to_bytecode());
        bytecode
    }

    fn from_bytecode(bytecode: Vec<u8>) -> Self {
        let name = ByteStr::from_bytecode(bytecode[0..16].to_vec());
        let ty = ByteType::from_bytecode(bytecode[16..].to_vec());
        Self {
            name,
            ty
        }
    }
}

impl Bytecode for ByteType {
    fn to_bytecode(&self) -> Vec<u8> {
        match self {
            ByteType::Int => vec![0],
            ByteType::Float => vec![1],
            ByteType::Str => vec![2],
            ByteType::Bool => vec![3],
            ByteType::Array(ty) => {
                let mut bytecode = vec![4];
                bytecode.extend(ty.to_bytecode());
                bytecode
            }
            ByteType::Fn(args, ret) => {
                let mut bytecode = vec![5];
                bytecode.extend(args.to_bytecode());
                bytecode.extend(ret.to_bytecode());
                bytecode
            }
            ByteType::Unit => vec![6]
        }
    }

    fn from_bytecode(bytecode: Vec<u8>) -> Self {
        match bytecode[0] {
            0 => ByteType::Int,
            1 => ByteType::Float,
            2 => ByteType::Str,
            3 => ByteType::Bool,
            4 => ByteType::Array(Box::new(ByteType::from_bytecode(bytecode[1..].to_vec()))),
            5 => {
                let args = Vec::<ByteType>::from_bytecode(bytecode[1..].to_vec());
                let ret = Box::new(ByteType::from_bytecode(bytecode[1..].to_vec()));
                ByteType::Fn(args, ret)
            },
            6 => ByteType::Unit,
            _ => panic!("Invalid bytecode for ByteType")
        }
    }
}

impl<A, B> Bytecode for (A, B)
where
    A: Bytecode,
    B: Bytecode,
{
    fn to_bytecode(&self) -> Vec<u8> {
        let mut bytecode = vec![];
        bytecode.extend(self.0.to_bytecode());
        bytecode.extend(self.1.to_bytecode());
        bytecode
    }

    fn from_bytecode(bytecode: Vec<u8>) -> Self {
        let a = A::from_bytecode(bytecode[0..9].to_vec());
        let b = B::from_bytecode(bytecode[9..18].to_vec());
        (a, b)
    }
}

impl<T> Bytecode for Box<T>
    where T: Bytecode {
    fn to_bytecode(&self) -> Vec<u8> {
        self.as_ref().to_bytecode()
    }

    fn from_bytecode(bytecode: Vec<u8>) -> Self {
        Box::new(T::from_bytecode(bytecode))
    }
}

impl<T> Bytecode for Vec<T>
where T: Bytecode {
    fn to_bytecode(&self) -> Vec<u8> {
        let mut bytecode = vec![];
        bytecode.extend(self.len().to_bytecode());
        for item in self {
            bytecode.extend(item.to_bytecode());
        }
        bytecode
    }

    fn from_bytecode(bytecode: Vec<u8>) -> Self {
        let len = usize::from_bytecode(bytecode[0..8].to_vec());
        let mut vec = vec![];
        let mut index = 8;
        for _ in 0..len {
            let item = T::from_bytecode(bytecode[index..index+9].to_vec());
            vec.push(item);
            index += 9;
        }
        vec
    }
}