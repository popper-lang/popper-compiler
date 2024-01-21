use llvm_sys::prelude::LLVMBuilderRef;
use llvm_sys::core::{
    LLVMCreateBuilderInContext,
    LLVMBuildAdd,
    LLVMPositionBuilderAtEnd
};
use crate::basic_block::BasicBlock;
use crate::context::Context;
use crate::value::int_value::IntValue;
use crate::value::ValueEnum;

#[derive(Debug, Copy, Clone)]
pub struct Builder {
    pub(crate) builder: LLVMBuilderRef,
    pub(crate) context: Context,
    pub(crate) entry_block: Option<BasicBlock>,
}

impl Builder {
    pub fn new(context: Context) -> Self {
        let builder = unsafe { LLVMCreateBuilderInContext(context.context) };
        Self { builder, context, entry_block: None }
    }

    pub fn get_context(&self) -> Context {
        self.context
    }

    pub fn build_int_add(&self, lhs: &IntValue, rhs: &IntValue, name: &str) -> IntValue {
        let name = std::ffi::CString::new(name).unwrap();
        let value = unsafe { LLVMBuildAdd(self.builder, lhs.int_value, rhs.int_value, name.as_ptr()) };
        IntValue::new_llvm_ref(value)
    }

    pub fn build_ret(&self, r: ValueEnum) {
        unsafe { llvm_sys::core::LLVMBuildRet(self.builder, r.into()) };
    }

    pub fn position_at_end(&mut self, basic_block: BasicBlock) {
        unsafe { LLVMPositionBuilderAtEnd(self.builder, basic_block.basic_block) }
    }

    pub fn get_entry_block(&self) -> Option<BasicBlock> {
        self.entry_block.clone()
    }

    pub fn set_entry_block(&mut self, basic_block: BasicBlock) {
        self.entry_block = Some(basic_block);
    }
}