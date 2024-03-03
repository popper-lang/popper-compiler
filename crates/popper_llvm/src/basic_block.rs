use crate::context::Context;
use llvm_sys::core::LLVMGetBasicBlockName;
use llvm_sys::prelude::LLVMBasicBlockRef;

#[derive(Debug, Clone, Copy)]
pub struct BasicBlock {
    pub(crate) basic_block: LLVMBasicBlockRef,
    pub(crate) context: Context,
}

impl BasicBlock {
    pub fn new(basic_block: LLVMBasicBlockRef, context: Context) -> Self {
        Self {
            basic_block,
            context,
        }
    }

    pub fn get_context(&self) -> Context {
        self.context
    }

    pub fn get_basic_block(&self) -> LLVMBasicBlockRef {
        self.basic_block
    }

    pub fn get_name(&self) -> String {
        let c_string = unsafe { std::ffi::CStr::from_ptr(LLVMGetBasicBlockName(self.basic_block)) };
        c_string.to_str().unwrap().to_string()
    }
}
