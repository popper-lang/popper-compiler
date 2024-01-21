use llvm_sys::core::{LLVMConstArray2, LLVMTypeOf};
use llvm_sys::prelude::LLVMValueRef;
use crate::types;
use crate::value::ValueEnum;

#[derive(Debug, Copy, Clone)]
pub struct ArrayValue {
    pub(crate) array_value: LLVMValueRef,
    pub(crate) array_type: types::array_types::ArrayType,
}

impl ArrayValue {

    pub fn new_const(value: &[ValueEnum], array_type: types::array_types::ArrayType) -> Self {
        let array_value = unsafe { LLVMConstArray2(array_type.array_type, value.as_ptr() as *mut LLVMValueRef, value.len() as u64) };
        Self { array_value, array_type }
    }

    pub fn new_llvm_ref(lref: LLVMValueRef) -> Self {
        let array_type = types::array_types::ArrayType::new_with_llvm_ref(unsafe { LLVMTypeOf(lref) });
        Self { array_value: lref, array_type }
    }

    pub fn get_value(&self) -> LLVMValueRef {
        self.array_value
    }
}