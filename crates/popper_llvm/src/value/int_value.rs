use crate::types::{int_types, TypeEnum};
use crate::value::{AsValueRef, Value, ValueEnum};
use llvm_sys::core::{LLVMConstInt, LLVMConstIntGetZExtValue, LLVMTypeOf};
use llvm_sys::prelude::{LLVMTypeRef, LLVMValueRef};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct IntValue {
    pub(crate) int_value: LLVMValueRef,
    pub(crate) int_type: int_types::IntType,
}

impl IntValue {
    /// # Safety
    /// This function is unsafe because it does not check if the LLVMValueRef is a valid int value.
    pub unsafe fn new_llvm_ref(lref: LLVMValueRef) -> Self {
        let int_type = int_types::IntType::new_llvm_ref(LLVMTypeOf(lref));
        Self {
            int_value: lref,
            int_type,
        }
    }
    pub fn new_const(value: u32, int_type: int_types::IntType, sign_extend: bool) -> Self {
        let int_value =
            unsafe { LLVMConstInt(int_type.int_type, value as u64, sign_extend.into()) };
        Self {
            int_value,
            int_type,
        }
    }

    pub fn get_int_type(&self) -> int_types::IntType {
        self.int_type
    }

    pub fn get_value(&self) -> u32 {
        unsafe { LLVMConstIntGetZExtValue(self.int_value) as u32 }
    }

    pub fn to_value_enum(self) -> ValueEnum {
        ValueEnum::IntValue(self)
    }
}

impl Value for IntValue {
    fn as_raw_ref(&self) -> LLVMValueRef {
        self.int_value
    }
    fn get_type_ref(&self) -> LLVMTypeRef {
        self.int_type.int_type
    }

    fn get_type(&self) -> TypeEnum {
        TypeEnum::IntType(self.int_type)
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
