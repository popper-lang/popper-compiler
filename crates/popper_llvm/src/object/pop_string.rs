use inkwell::values::{PointerValue};
use inkwell::types::ArrayType;
use inkwell::values::{ArrayValue, BasicValue};
use inkwell::context::Context;

use inkwell::builder::Builder;

#[derive(Debug, Clone)]
pub struct PopString<'a> {
    pub array_ty: ArrayType<'a>,
    pub array_value: ArrayValue<'a>,
}

impl<'a> PopString<'a> {
    pub fn new(array_ty: ArrayType<'a>, array_value: ArrayValue<'a>) -> Self {
        Self {
            array_ty,
            array_value
        }
    }

    pub fn from_value(value: ArrayValue<'a>) -> Self {
        let ty = value.get_type();
        Self::new(ty, value)
    }

    pub fn cast_to_ptr(&self, context: &'a Context, builder: &'a Builder<'a>) -> PointerValue<'a> {
        let _array_ty = context.i8_type().array_type(self.array_ty.len() + 1);
        let global = builder.build_alloca(self.array_ty, "string_literal").unwrap();
        builder.build_store(global, self.array_value.as_basic_value_enum()).unwrap();
        global
    }
}