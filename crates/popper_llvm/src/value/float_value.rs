use llvm_sys::prelude::{
    LLVMValueRef,
    LLVMTypeRef,
};
use llvm_sys::core:: {
    LLVMConstReal,
    LLVMConstRealGetDouble,
    LLVMTypeOf
};
use crate::types;
use crate::types::TypeEnum;
use crate::value::{Value, ValueEnum};

#[derive(Debug, Copy, Clone)]
pub struct FloatValue {
    pub(crate) float_value: LLVMValueRef,
    pub(crate) float_type: types::float_types::FloatType,
}

impl FloatValue {

    pub fn new_llvm_ref(lref: LLVMValueRef) -> Self {
        let float_type = types::float_types::FloatType::new_with_llvm_ref(unsafe { LLVMTypeOf(lref) });
        Self { float_value: lref, float_type }
    }
    pub fn new_const(value: f64, float_type: types::float_types::FloatType) -> Self {
        let float_value = unsafe { LLVMConstReal(float_type.float_type, value) };
        Self { float_value, float_type }
    }

    pub fn get_value(&self) -> f64 {
        let double = unsafe { LLVMConstRealGetDouble(self.float_value, &mut 0) };
        double as f64
    }

    pub fn to_value_enum(&self) -> ValueEnum {
        ValueEnum::FloatValue(*self)
    }
}

impl Value for FloatValue {
    fn get_type_ref(&self) -> LLVMTypeRef {
        self.float_type.float_type
    }

    fn get_type(&self) -> TypeEnum {
        TypeEnum::FloatType(self.float_type)
    }

    fn as_value_ref(&self) -> LLVMValueRef {
        self.float_value
    }

    fn is_null_or_undef(&self) -> bool {
        false
    }

    fn is_const(&self) -> bool {
        true
    }

    fn is_null(&self) -> bool {
        false
    }

    fn is_undef(&self) -> bool {
        false
    }

}