use crate::context::Context;
use crate::types::function_types::FunctionType;
use crate::types::{Type, TypeEnum};
use crate::value::float_value::FloatValue;
use llvm_sys::core::LLVMDoubleTypeInContext;
use llvm_sys::prelude::LLVMTypeRef;

#[derive(Debug, Copy, Clone)]
pub struct FloatType {
    pub(crate) float_type: LLVMTypeRef,
}

impl FloatType {
    pub fn new_with_llvm_ref(float_type: LLVMTypeRef) -> Self {
        Self { float_type }
    }
    pub fn new_with_context(context: Context) -> Self {
        let float_type = unsafe { LLVMDoubleTypeInContext(context.context) };
        Self { float_type }
    }

    pub fn float(&self, value: f64) -> FloatValue {
        FloatValue::new_const(value, *self)
    }

    pub fn func(&self, args: Vec<TypeEnum>, is_var_args: bool) -> FunctionType {
        FunctionType::new(args, self.to_type_enum(), is_var_args)
    }

    pub fn to_type_enum(&self) -> TypeEnum {
        TypeEnum::FloatType(*self)
    }
}

impl Type for FloatType {
    fn is_sized(&self) -> bool {
        true
    }

    fn get_type_ref(&self) -> LLVMTypeRef {
        self.float_type
    }
}
