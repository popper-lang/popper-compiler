use llvm_sys::prelude::LLVMTypeRef;
use llvm_sys::core::{
    LLVMDoubleTypeInContext
};
use crate::context::Context;
use crate::types::Type;


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

}

impl Type for FloatType {
    fn is_sized(&self) -> bool {
        true
    }

    fn get_type_ref(&self) -> LLVMTypeRef {
        self.float_type
    }

}