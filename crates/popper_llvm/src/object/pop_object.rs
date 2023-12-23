use inkwell::types::{FloatType, IntType, PointerType};
use inkwell::values::{BasicValueEnum, FloatValue, IntValue, PointerValue};
use inkwell::context::Context;
use crate::object::pop_string::PopString;

pub enum PopObject<'a> {
    Int(IntType<'a>, IntValue<'a>),
    Bool(IntType<'a>, IntValue<'a>),
    Float(FloatType<'a>, FloatValue<'a>),
    String(PopString<'a>),
    Ptr(PointerType<'a>, PointerValue<'a>)
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
            BasicValueEnum::PointerValue(ptr) => PopObject::Ptr(ptr.get_type(), ptr),
            _ => panic!("Unknown type")
        }
    }

    pub fn to_basic_value_enum(&self) -> BasicValueEnum {
        match self {
            PopObject::Int(_, int) => BasicValueEnum::IntValue(*int),
            PopObject::Bool(_, int) => BasicValueEnum::IntValue(*int),
            PopObject::Float(_, float) => BasicValueEnum::FloatValue(*float),
            PopObject::String(string) => BasicValueEnum::ArrayValue(string.array_value),
            PopObject::Ptr(_, ptr) => BasicValueEnum::PointerValue(*ptr)
        }
    }
}
