#![allow(dead_code)]

use inkwell::context::Context;
use inkwell::types::{BasicMetadataTypeEnum, BasicTypeEnum};
use popper_ast::TypeKind;


#[derive(Debug, Clone, Copy)]
pub enum PopType {
    Int,
    Float,
    String(u32),
    Boolean
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
            _ => panic!("Unknown type")
        }
    }

    pub fn to_llvm_type(self, context: &'ctx Context) -> BasicTypeEnum<'_> {
        match self {
            PopType::Int => BasicTypeEnum::IntType(context.i32_type()),
            PopType::Float => BasicTypeEnum::FloatType(context.f32_type()),
            PopType::String(len) => {
                BasicTypeEnum::ArrayType(context.i8_type().array_type(len + 1))
            },
            PopType::Boolean => BasicTypeEnum::IntType(context.bool_type())
        }
    }

    pub fn to_basic_metadata_type(self, context: &'ctx Context) -> BasicMetadataTypeEnum<'_> {
        match self {
            PopType::Int => BasicMetadataTypeEnum::IntType(context.i32_type()),
            PopType::Float => BasicMetadataTypeEnum::FloatType(context.f32_type()),
            PopType::String(len) => {
                BasicMetadataTypeEnum::ArrayType(context.i8_type().array_type(len))
            },
            PopType::Boolean => BasicMetadataTypeEnum::IntType(context.bool_type())
        }
    }

    pub fn is_global_val_stored(self) -> bool {
        match self {
            PopType::String(_) => true,
            _ => false
        }
    }
}
