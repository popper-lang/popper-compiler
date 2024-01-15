use inkwell::builder::Builder;
use inkwell::types::{BasicType, BasicTypeEnum, FloatType, FunctionType, IntType, StructType};
use inkwell::values::{BasicValue, BasicValueEnum, FloatValue, FunctionValue, IntValue, StructValue};
use inkwell::context::Context;
use crate::object::pop_pointer::PopPointer;
use crate::object::pop_string::PopString;
use crate::object::pop_type::PopType;

#[derive(Debug, Clone)]
pub enum PopObject<'a> {
    Int(IntType<'a>, IntValue<'a>),
    Bool(IntType<'a>, IntValue<'a>),
    Float(FloatType<'a>, FloatValue<'a>),
    String(PopString<'a>),
    Ptr(PopPointer<'a>),
    Function(FunctionType<'a>, FunctionValue<'a>),
    Struct(StructType<'a>, StructValue<'a>),
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

    pub fn new_ptr(ptr: PopPointer<'a>) -> Self {
        PopObject::Ptr(ptr)
    }

    pub fn new_function(func: FunctionValue<'a>) -> Self {
        PopObject::Function(func.get_type(), func)
    }

    pub fn new_struct(struct_value: StructValue<'a>) -> Self {
        PopObject::Struct(struct_value.get_type(), struct_value)
    }
    
    pub fn from_basic_value_enum(basic_enum: BasicValueEnum<'a>) -> Self {
        match basic_enum {
            BasicValueEnum::IntValue(int) => PopObject::Int(int.get_type(), int),
            BasicValueEnum::FloatValue(float) => PopObject::Float(float.get_type(), float),
            BasicValueEnum::PointerValue(ptr) => PopObject::Ptr(PopPointer::from_value(ptr)),
            BasicValueEnum::StructValue(struct_value) => PopObject::Struct(struct_value.get_type(), struct_value),
            _ => panic!("Unknown type(from_basic_value_enum)")
        }
    }
    pub fn to_basic_value_enum(self) -> BasicValueEnum<'a> {
        match self {
            PopObject::Int(_, int) => BasicValueEnum::IntValue(int),
            PopObject::Bool(_, int) => BasicValueEnum::IntValue(int),
            PopObject::Float(_, float) => BasicValueEnum::FloatValue(float),
            PopObject::String(string) => string.array_value.as_basic_value_enum(),
            PopObject::Ptr(ptr) => BasicValueEnum::PointerValue(ptr.value),
            PopObject::Struct(_, struct_value) => BasicValueEnum::StructValue(struct_value),
            _ => panic!("Unknown type(to_basic_value_enum)")
        }
    }

    pub fn into_safe_obj(&self, builder: &Builder<'a>) -> PopObject<'a> {
        if let PopObject::Ptr(ptr) = self {
            PopObject::from_basic_value_enum(builder.build_load(ptr.ty, ptr.value,  "load")
                .unwrap()
                .into())
        } else {
            self.clone()
        }
    }

    pub fn is_constant_global(&self) -> bool {
        match self {
            PopObject::String(_) => true,
            _ => false
        }
    }

    pub fn get_type(self) -> BasicTypeEnum<'a> {
        match self {
            PopObject::Int(ty, _) => BasicTypeEnum::IntType(ty),
            PopObject::Bool(ty, _) => BasicTypeEnum::IntType(ty),
            PopObject::Float(ty, _) => BasicTypeEnum::FloatType(ty),
            PopObject::Ptr(ty) =>   ty.ty.as_basic_type_enum(),
            PopObject::String(string) => string.array_ty.as_basic_type_enum(),
            PopObject::Struct(ty, _) => BasicTypeEnum::StructType(ty),
            _ => panic!("Unknown type")
        }
    }

    pub fn get_pop_type(&self) -> PopType {
        match self {
            PopObject::Int(_, _) => PopType::Int,
            PopObject::Bool(_, _) => PopType::Boolean,
            PopObject::Float(_, _) => PopType::Float,
            PopObject::String(x) => PopType::String(x.array_ty.len()),
            PopObject::Struct(st, _) => {
                let mut fields = Vec::new();
                for field in st.get_field_types() {
                    fields.push(PopType::from_llvm_type(field));
                }
                PopType::Struct(fields)
            },
            _ => panic!("Unknown type")
        }
    }

    pub fn cast_to_ptr(&self, context: &'a Context, builder: &'a Builder<'a>) -> PopPointer<'a> {
        match self {
            PopObject::Int(ty, int) => builder.build_int_to_ptr(*int, ty.ptr_type(Default::default()), "int_to_ptr").unwrap().into(),
            PopObject::Ptr(ptr) => ptr.clone(),
            PopObject::String(string) => string.cast_to_ptr(context, builder).into(),
            _ => panic!("Unknown type")
        }
    }
}
