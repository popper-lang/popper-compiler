#![allow(dead_code)]

use inkwell::context::Context;
use inkwell::types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum};
use popper_ast::TypeKind;


#[derive(Debug, Clone)]
pub enum PopType {
    Int,
    Float,
    String(u32),
    Boolean,
    Pointer(Box<PopType>),
    Struct(Vec<PopType>),
    StructInstance(String),
}

impl<'ctx> PopType {
    pub fn from_string(string: String) -> Self {
        match string.as_str() {
            "int" => PopType::Int,
            "float" => PopType::Float,
            "string" => PopType::String(0),
            "boolean" => PopType::Boolean,
            _ => panic!("Unknown type")
        }
    }

    pub fn from_ty_ast(ty: TypeKind) -> Self {
        match ty {
            TypeKind::Int => PopType::Int,
            TypeKind::Float => PopType::Float,
            TypeKind::String(len) => PopType::String(len),
            TypeKind::Bool => PopType::Boolean,
            TypeKind::Pointer(ty) => PopType::Pointer(
                Box::new(
                    PopType::from_ty_ast(ty.type_kind)
                )
            ),
            TypeKind::Struct(tys) => PopType::Struct(
                tys.into_iter().map(|ty| PopType::from_ty_ast(ty.1.type_kind)).collect()
            ),
            TypeKind::StructInstance(name) => PopType::StructInstance(name),
            _ => panic!("Unknown type")
        }
    }

    pub fn from_llvm_type(ty: BasicTypeEnum<'_>) -> Self {
        match ty {
            BasicTypeEnum::IntType(_) => PopType::Int,
            BasicTypeEnum::FloatType(_) => PopType::Float,
            BasicTypeEnum::ArrayType(array_ty) => {
                PopType::String(array_ty.len() - 1)
            },
            BasicTypeEnum::StructType(struct_ty) => {
                PopType::Struct(
                    struct_ty
                        .get_field_types()
                        .iter()
                        .map(|ty| PopType::from_llvm_type(*ty))
                        .collect()
                )
            },
            _ => panic!("Unknown type(from_llvm_type)")
        }
    }

    pub fn to_llvm_type(self, context: &'ctx Context) -> BasicTypeEnum<'ctx> {
        match self {
            PopType::Int => BasicTypeEnum::IntType(context.i32_type()),
            PopType::Float => BasicTypeEnum::FloatType(context.f32_type()),
            PopType::String(len) => {
                BasicTypeEnum::ArrayType(context.i8_type().array_type(len + 1))
            },
            PopType::Boolean => BasicTypeEnum::IntType(context.bool_type()),
            PopType::Pointer(ty) => {
                BasicTypeEnum::PointerType(
                    ty.to_llvm_type(context).ptr_type(Default::default())
                )
            },
            PopType::Struct(tys) => {
                let mut struct_types = Vec::new();
                for ty in tys {
                    struct_types.push(ty.to_llvm_type(context));
                }
                BasicTypeEnum::StructType(context.struct_type(&struct_types, false))
            },
            PopType::StructInstance(ref _name) => {
                todo!()
            }
        }
    }

    pub fn to_basic_metadata_type(self, context: &'ctx Context) -> BasicMetadataTypeEnum<'ctx> {
        self.to_llvm_type(context).into()
    }

    pub fn is_global_val_stored(self) -> bool {
        match self {
            PopType::String(_) => true,
            _ => false
        }
    }
}
