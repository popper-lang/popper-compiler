use std::ffi::CString;
use llvm_sys::prelude::{LLVMBuilderRef, LLVMTypeRef, LLVMValueRef};
use llvm_sys::core::{LLVMCreateBuilderInContext, LLVMBuildAdd, LLVMPositionBuilderAtEnd, LLVMBuildSub, LLVMBuildMul, LLVMBuildNSWMul, LLVMBuildFDiv, LLVMBuildNSWAdd};
use crate::basic_block::BasicBlock;
use crate::context::Context;
use crate::value::function_value::FunctionValue;
use crate::value::int_value::IntValue;
use crate::value::{Value, ValueEnum};

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

    pub fn build_int_sub(&self, lhs: &IntValue, rhs: &IntValue, name: &str) -> IntValue {
        let name = std::ffi::CString::new(name).unwrap();
        let value = unsafe { LLVMBuildSub(self.builder, lhs.int_value, rhs.int_value, name.as_ptr()) };
        IntValue::new_llvm_ref(value)
    }

    pub fn build_int_mul(&self, lhs: &IntValue, rhs: &IntValue, name: &str) -> IntValue {
        let name = std::ffi::CString::new(name).unwrap();
        let value = unsafe { LLVMBuildMul(self.builder, lhs.int_value, rhs.int_value, name.as_ptr()) };
        IntValue::new_llvm_ref(value)
    }

    pub fn build_int_div(&self, lhs: &IntValue, rhs: &IntValue, name: &str) -> IntValue {
        let name = std::ffi::CString::new(name).unwrap();
        let value = unsafe { LLVMBuildFDiv(self.builder, lhs.int_value, rhs.int_value, name.as_ptr()) };
        IntValue::new_llvm_ref(value)
    }

    pub fn build_int_nsw_add(&self, lhs: &IntValue, rhs: &IntValue, name: &str) -> IntValue {
        let name = CString::new(name).unwrap();
        let value = unsafe { LLVMBuildNSWAdd(self.builder, lhs.int_value, rhs.int_value, name.as_ptr()) };
        IntValue::new_llvm_ref(value)
    }

    pub fn build_call(&self, function: FunctionValue, args: Vec<ValueEnum>, name: &str) -> ValueEnum {
        let name = CString::new(name).unwrap();
        let value = self.build_direct_call(function, args.into_iter().map(|v| v.into()).collect(), name);
        value.into()
    }

    pub fn build_direct_call(&self, function: FunctionValue, args: Vec<LLVMValueRef>, name: CString) -> LLVMValueRef {
        let mut args = args;
        unsafe { llvm_sys::core::LLVMBuildCall2(self.builder, function.get_type_ref(), function.as_value_ref(), args.as_mut_ptr(), args.len() as u32, name.as_ptr()) }
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