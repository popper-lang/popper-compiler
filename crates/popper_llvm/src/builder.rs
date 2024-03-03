use crate::basic_block::BasicBlock;
use crate::context::Context;
use crate::types::Type;
use crate::types::TypeEnum;
use crate::value::function_value::FunctionValue;
use crate::value::int_value::IntValue;
use crate::value::pointer_value::PointerValue;
use crate::value::{Value, ValueEnum};
use llvm_sys::core::{
    LLVMBuildAdd, LLVMBuildFDiv, LLVMBuildMul, LLVMBuildNSWAdd, LLVMBuildSub, LLVMConstInt,
    LLVMCreateBuilderInContext, LLVMPositionBuilderAtEnd,
};
use llvm_sys::prelude::{LLVMBuilderRef, LLVMValueRef};
use std::ffi::CString;

#[derive(Debug, Clone)]
pub struct Builder {
    pub(crate) builder: LLVMBuilderRef,
    pub(crate) context: Context,
    pub(crate) entry_block: Option<BasicBlock>,
}

impl Builder {
    pub fn new(context: Context) -> Self {
        let builder = unsafe { LLVMCreateBuilderInContext(context.context) };
        Self {
            builder,
            context,
            entry_block: None,
        }
    }

    pub fn get_context(&self) -> Context {
        self.context
    }

    pub fn build_int_add(&self, lhs: &IntValue, rhs: &IntValue, name: &str) -> IntValue {
        let name = std::ffi::CString::new(name).unwrap();
        let value =
            unsafe { LLVMBuildAdd(self.builder, lhs.int_value, rhs.int_value, name.as_ptr()) };
        IntValue::new_llvm_ref(value)
    }

    pub fn build_int_sub(&self, lhs: &IntValue, rhs: &IntValue, name: &str) -> IntValue {
        let name = std::ffi::CString::new(name).unwrap();
        let value =
            unsafe { LLVMBuildSub(self.builder, lhs.int_value, rhs.int_value, name.as_ptr()) };
        IntValue::new_llvm_ref(value)
    }

    pub fn build_int_mul(&self, lhs: &IntValue, rhs: &IntValue, name: &str) -> IntValue {
        let name = std::ffi::CString::new(name).unwrap();
        let value =
            unsafe { LLVMBuildMul(self.builder, lhs.int_value, rhs.int_value, name.as_ptr()) };
        IntValue::new_llvm_ref(value)
    }

    pub fn build_int_div(&self, lhs: &IntValue, rhs: &IntValue, name: &str) -> IntValue {
        let name = std::ffi::CString::new(name).unwrap();
        let value =
            unsafe { LLVMBuildFDiv(self.builder, lhs.int_value, rhs.int_value, name.as_ptr()) };
        IntValue::new_llvm_ref(value)
    }

    pub fn build_int_nsw_add(&self, lhs: &IntValue, rhs: &IntValue, name: &str) -> IntValue {
        let name = CString::new(name).unwrap();
        let value =
            unsafe { LLVMBuildNSWAdd(self.builder, lhs.int_value, rhs.int_value, name.as_ptr()) };
        IntValue::new_llvm_ref(value)
    }

    pub fn build_call(
        &self,
        function: FunctionValue,
        args: Vec<ValueEnum>,
        name: &str,
    ) -> ValueEnum {
        let name = CString::new(name).unwrap();
        let value =
            self.build_direct_call(function, args.into_iter().map(|v| v.into()).collect(), name);
        value.into()
    }

    pub fn build_direct_call(
        &self,
        function: FunctionValue,
        args: Vec<LLVMValueRef>,
        name: CString,
    ) -> LLVMValueRef {
        let i64t = self.context.i64_type().get_type_ref();
        function.dump();
        let mut args = unsafe { [LLVMConstInt(i64t, 1, 0), LLVMConstInt(i64t, 1, 0)] };
        let ty = function.get_type();
        let length = args.len() as u32;
        unsafe {
            llvm_sys::core::LLVMBuildCall2(
                self.builder,
                function.get_type_ref(),
                function.as_value_ref(),
                args.as_mut_ptr(),
                length,
                b"entry\0".as_ptr() as *const _,
            )
        }
    }

    pub fn build_ret(&self, r: ValueEnum) {
        unsafe { llvm_sys::core::LLVMBuildRet(self.builder, r.into()) };
    }

    pub fn build_alloca(&self, ty: TypeEnum, name: &str) -> PointerValue {
        let name = CString::new(name).unwrap();
        let value = unsafe {
            llvm_sys::core::LLVMBuildAlloca(self.builder, ty.get_type_ref(), name.as_ptr())
        };
        PointerValue::new_llvm_ref(value)
    }

    pub fn build_store(&self, value: ValueEnum, ptr: PointerValue) {
        unsafe { llvm_sys::core::LLVMBuildStore(self.builder, value.into(), ptr.get_llvm_ref()) };
    }

    pub fn build_load(&self, ty: TypeEnum, ptr: PointerValue, name: &str) -> ValueEnum {
        let name = CString::new(name).unwrap();
        let value = unsafe {
            llvm_sys::core::LLVMBuildLoad2(
                self.builder,
                ty.get_type_ref(),
                ptr.get_llvm_ref(),
                name.as_ptr(),
            )
        };
        value.into()
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

impl Drop for Builder {
    fn drop(&mut self) {
        unsafe { llvm_sys::core::LLVMDisposeBuilder(self.builder) }
    }
}
