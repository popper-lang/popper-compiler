use std::ffi::CString;
use inkwell::context::Context;
use inkwell::types::{ArrayType, BasicMetadataTypeEnum, BasicType, BasicTypeEnum, FloatType, IntType, PointerType};
use inkwell::values::{ArrayValue, BasicValueEnum, FloatValue, IntValue, PointerValue};
use popper_ast::TypeKind;


pub enum PopObject<'a> {
    Int(IntType<'a>, IntValue<'a>),
    Bool(IntType<'a>, IntValue<'a>),
    Float(FloatType<'a>, FloatValue<'a>),
    String(PopString<'a>),
}

impl<'a> PopObject<'a> {
    pub fn new_int(context: &'a Context, int: u64) -> Self {
        let ty = context.i32_type();
        let val = ty.const_int(int, false);
        PopObject::Int(ty, val)
    }

    pub fn new_boolean(context: &'a Context, boolean: bool) -> Self {
        let ty = context.i32_type();
        let val = ty.const_int(if boolean { 1 } else { 0 }, false);
        PopObject::Bool(ty, val)
    }

    pub fn new_float(context: &'a Context, float: f64) -> Self {
        let ty = context.f32_type();
        let val = ty.const_float(float);

        PopObject::Float(ty, val)
    }

    pub fn new_string(context: &'a Context, string: String) -> Self {
        PopObject::String(
            PopString::from_string(context, string)
        )
    }

    pub fn from_basic_value_enum(basic_enum: BasicValueEnum<'a>) -> Self {
        match basic_enum {
            BasicValueEnum::IntValue(int) => PopObject::Int(int.get_type(), int),
            BasicValueEnum::FloatValue(float) => PopObject::Float(float.get_type(), float),
            BasicValueEnum::ArrayValue(string) => PopObject::String(PopString::from_array_value(string)),
            _ => panic!("Unknown type")
        }
    }

    pub fn to_basic_value_enum(&self) -> BasicValueEnum {
        match self {
            PopObject::Int(_, int) => BasicValueEnum::IntValue(*int),
            PopObject::Bool(_, int) => BasicValueEnum::IntValue(*int),
            PopObject::Float(_, float) => BasicValueEnum::FloatValue(*float),
            PopObject::String(string) => BasicValueEnum::ArrayValue(string.array_value),
        }
    }
}

pub struct PopString<'a> {
    pub array_ty: ArrayType<'a>,
    pub array_value: ArrayValue<'a>,
}

impl<'a> PopString<'a> {
    pub fn from_string(context: &'a Context, string: String) -> Self {
        let i8_ty = context.i8_type();
        let cstring = CString::new(string).expect("Cast failed");
        let bytes: &[u8] = cstring.as_bytes_with_nul();
        let array_ty = i8_ty.array_type(bytes.len() as u32);
        let array_value = context.const_string(bytes, false);
        PopString {
            array_ty,
            array_value,
        }
    }

    pub fn from_array_value(array_value: ArrayValue<'a>) -> Self {
        let array_ty = array_value.get_type();
        PopString {
            array_ty,
            array_value,
        }
    }

    pub fn build_string(&self) -> String {
        self.array_value.to_string()
    }
}



pub enum PopType {
    Int,
    Float,
    String,
    Boolean
}

impl<'ctx> PopType {
    pub fn from_string(string: String) -> Self {
        match string.as_str() {
            "int" => PopType::Int,
            "float" => PopType::Float,
            "string" => PopType::String,
            "boolean" => PopType::Boolean,
            _ => panic!("Unknown type")
        }
    }

    pub fn from_ty_ast(ty: TypeKind) -> Self {
        match ty {
            TypeKind::Int => PopType::Int,
            TypeKind::Float => PopType::Float,
            TypeKind::String => PopType::String,
            TypeKind::Bool => PopType::Boolean,
            _ => panic!("Unknown type")
        }
    }

    pub fn to_llvm_type(self, context: &'ctx Context) -> BasicTypeEnum<'_> {
        match self {
            PopType::Int => BasicTypeEnum::IntType(context.i32_type()),
            PopType::Float => BasicTypeEnum::FloatType(context.f32_type()),
            PopType::String => {
                BasicTypeEnum::ArrayType(context.i8_type().array_type(u32::MAX))
            },
            PopType::Boolean => BasicTypeEnum::IntType(context.bool_type())
        }
    }

    pub fn to_basic_metadata_type(self, context: &'ctx Context) -> BasicMetadataTypeEnum<'_> {
        match self {
            PopType::Int => BasicMetadataTypeEnum::IntType(context.i32_type()),
            PopType::Float => BasicMetadataTypeEnum::FloatType(context.f32_type()),
            PopType::String => {
                BasicMetadataTypeEnum::ArrayType(context.i8_type().array_type(u32::MAX))
            },
            PopType::Boolean => BasicMetadataTypeEnum::IntType(context.bool_type())
        }
    }
}

pub struct Pointer<'ctx> {
    pub(crate) ty: PointerType<'ctx>,
    pub(crate) value: PointerValue<'ctx>
}

impl<'ctx> Pointer<'ctx> {
    pub fn new(ty: PointerType<'ctx>, value: PointerValue<'ctx>) -> Self {
        Pointer {
            ty,
            value
        }
    }
}
