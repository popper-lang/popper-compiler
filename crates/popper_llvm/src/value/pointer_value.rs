use crate::types::TypeEnum;
use crate::value::{Value, ValueEnum};
use llvm_sys::core::{LLVMConstPointerCast, LLVMIsNull};
use llvm_sys::prelude::LLVMValueRef;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PointerValue {
    pub(crate) pointer_value: LLVMValueRef,
}

impl PointerValue {
    pub fn new_constant(value: ValueEnum, ty: TypeEnum) -> Self {
        let ptr = unsafe { LLVMConstPointerCast(value.as_llvm_ref(), ty.get_type_ref()) };
        Self { pointer_value: ptr }
    }
    pub fn new_llvm_ref(pointer_value: LLVMValueRef) -> Self {
        Self { pointer_value }
    }

    pub fn get_llvm_ref(&self) -> LLVMValueRef {
        self.pointer_value
    }

    pub fn to_value_enum(&self) -> ValueEnum {
        ValueEnum::PointerValue(*self)
    }
}
impl Value for PointerValue {
    fn is_null_or_undef(&self) -> bool {
        unsafe { LLVMIsNull(self.as_raw_ref()) != 0 }
    }

    fn as_raw_ref(&self) -> LLVMValueRef {
        self.pointer_value
    }

    fn is_const(&self) -> bool {
        false
    }

    fn is_null(&self) -> bool {
        self.is_null_or_undef()
    }

    fn is_undef(&self) -> bool {
        self.is_null_or_undef()
    }
}
