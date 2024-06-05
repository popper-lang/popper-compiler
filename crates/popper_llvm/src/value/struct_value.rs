use llvm_sys::prelude::LLVMValueRef;
use crate::value::{RawValue, Value};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct StructValue {
    pub(crate) struct_value: RawValue,
}

impl StructValue {
    pub fn new(struct_value: RawValue) -> Self {
        Self { struct_value }
    }
    
    pub fn new_llvm_ref(struct_value: LLVMValueRef) -> Self {
        Self { struct_value: RawValue::new(struct_value) }
    }

    pub fn as_raw(&self) -> RawValue {
        self.struct_value
    }
}

impl Value for StructValue {
    fn as_raw(&self) -> RawValue {
        self.struct_value
    }

    fn is_null_or_undef(&self) -> bool {
        false
    }

    fn is_const(&self) -> bool {
        false
    }

    fn is_null(&self) -> bool {
        false
    }

    fn is_undef(&self) -> bool {
        false
    }
}