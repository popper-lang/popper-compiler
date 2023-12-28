use inkwell::builder::Builder;
use inkwell::types::{BasicType, BasicTypeEnum, FloatType, FunctionType, IntType, PointerType};
use inkwell::values::{BasicValue, BasicValueEnum, FloatValue, FunctionValue, IntValue, PointerValue};
use inkwell::context::Context;
use crate::object::pop_string::PopString;

#[derive(Debug, Clone)]
pub enum PopObject<'a> {
    Int(IntType<'a>, IntValue<'a>),
    Bool(IntType<'a>, IntValue<'a>),
    Float(FloatType<'a>, FloatValue<'a>),
    String(PopString<'a>),
    Ptr(PointerType<'a>, PointerValue<'a>),
    Function(FunctionType<'a>, FunctionValue<'a>)
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

    pub fn new_ptr(ptr: PointerValue<'a>) -> Self {
        PopObject::Ptr(ptr.get_type(), ptr)
    }

    pub fn new_function(_context: &'a Context, func: FunctionValue<'a>) -> Self {
        PopObject::Function(func.get_type(), func)
    }
    
    pub fn from_basic_value_enum(basic_enum: BasicValueEnum<'a>) -> Self {
        match basic_enum {
            BasicValueEnum::IntValue(int) => PopObject::Int(int.get_type(), int),
            BasicValueEnum::FloatValue(float) => PopObject::Float(float.get_type(), float),
            BasicValueEnum::PointerValue(ptr) => PopObject::Ptr(ptr.get_type(), ptr),
            _ => panic!("Unknown type(from_basic_value_enum)")
        }
    }
    pub fn to_basic_value_enum(&self) -> BasicValueEnum {
        match self {
            PopObject::Int(_, int) => BasicValueEnum::IntValue(*int),
            PopObject::Bool(_, int) => BasicValueEnum::IntValue(*int),
            PopObject::Float(_, float) => BasicValueEnum::FloatValue(*float),
            PopObject::String(string) => string.array_value.as_basic_value_enum(),
            PopObject::Ptr(_, ptr) => BasicValueEnum::PointerValue(*ptr),
            _ => panic!("Unknown type(to_basic_value_enum)")
        }
    }

    pub fn into_safe_obj(&self, builder: &Builder<'a>) -> PopObject<'a> {
        if let PopObject::Ptr(ty, ptr) = self {
            PopObject::from_basic_value_enum(builder.build_load(*ty, *ptr,  "load")
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
            PopObject::Ptr(ty, _) => BasicTypeEnum::PointerType(ty),
            PopObject::String(string) => string.array_ty.as_basic_type_enum(),
            _ => panic!("Unknown type")
        }
    }

    pub fn cast_to_ptr(&self, context: &'a Context, builder: &'a Builder<'a>) -> PointerValue<'a> {
        match self {
            PopObject::Int(ty, int) => builder.build_int_to_ptr(*int, ty.ptr_type(Default::default()), "int_to_ptr").unwrap(),
            PopObject::Ptr(_, ptr) => *ptr,
            PopObject::String(string) => string.cast_to_ptr(context, builder),
            _ => panic!("Unknown type")
        }
    }
}
