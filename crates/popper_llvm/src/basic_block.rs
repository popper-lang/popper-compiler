use crate::context::Context;
use crate::util::ptr_to_option;
use llvm_sys::core::*;
use llvm_sys::prelude::*;
use crate::value::RawValue;

#[derive(Debug, Clone, Copy)]
pub struct BasicBlock {
    pub(crate) basic_block: LLVMBasicBlockRef,
}

impl BasicBlock {
    pub fn new(basic_block: LLVMBasicBlockRef) -> Self {
        Self {
            basic_block,
        }
    }
    
    pub fn get_basic_block_ref(&self) -> LLVMBasicBlockRef {
        self.basic_block
    }

    pub fn get_name(&self) -> String {
        let c_string = unsafe { std::ffi::CStr::from_ptr(LLVMGetBasicBlockName(self.basic_block)) };
        c_string.to_str().unwrap().to_string()
    }

    pub fn get_next_basic_block(&self) -> Option<BasicBlock> {
        let next = unsafe { LLVMGetNextBasicBlock(self.basic_block) };
        let next = ptr_to_option(next)?;
        Some(BasicBlock::new(next))
    }

    pub fn get_previous_basic_block(&self) -> Option<BasicBlock> {
        let prev = unsafe { LLVMGetPreviousBasicBlock(self.basic_block) };
        let prev = ptr_to_option(prev)?;
        Some(BasicBlock::new(prev))
    }
    
    pub fn as_value(&self) -> RawValue {
        let val = unsafe { LLVMBasicBlockAsValue(self.basic_block) };
        
        RawValue::new(val)
    }
}
