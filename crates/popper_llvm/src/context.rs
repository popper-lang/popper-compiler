use crate::basic_block::BasicBlock;
use crate::builder::Builder;
use llvm_sys::core::{LLVMAppendBasicBlockInContext, LLVMContextCreate};
use llvm_sys::prelude::LLVMContextRef;

use crate::module::Module;
use crate::types::{float_types, int_types};
use crate::value::function_value::FunctionValue;

#[derive(Clone, Copy, Debug)]
pub struct Context {
    pub(crate) context: LLVMContextRef,
}

impl Context {
    pub fn new() -> Self {
        let context = unsafe { LLVMContextCreate() };
        Self { context }
    }

    pub fn new_module(&self, name: &str) -> Module {
        Module::new(name, *self)
    }

    pub fn new_builder(&mut self) -> Builder {
         Builder::new(*self)
    }

    pub fn void_type(&self) -> int_types::IntType {
        int_types::IntType::new_with_context(0, *self)
    }

    pub fn i1_type(&self) -> int_types::IntType {
        int_types::IntType::new_with_context(1, *self)
    }

    pub fn i8_type(&self) -> int_types::IntType {
        int_types::IntType::new_with_context(8, *self)
    }

    pub fn i16_type(&self) -> int_types::IntType {
        int_types::IntType::new_with_context(16, *self)
    }

    pub fn i32_type(&self) -> int_types::IntType {
        int_types::IntType::new_with_context(32, *self)
    }

    pub fn i64_type(&self) -> int_types::IntType {
        int_types::IntType::new_with_context(64, *self)
    }

    pub fn float_type(&self) -> float_types::FloatType {
        float_types::FloatType::new_with_context(*self)
    }

    pub fn append_basic_block(&self, name: &str, fn_value: FunctionValue) -> BasicBlock {
        let name = std::ffi::CString::new(name).unwrap();
        let block = unsafe {
            LLVMAppendBasicBlockInContext(self.context, fn_value.function_value, name.as_ptr())
        };
        BasicBlock::new(block, *self)
    }
}


impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}
