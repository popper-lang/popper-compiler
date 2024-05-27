use crate::basic_block::BasicBlock;
use crate::context::Context;
use crate::types::TypeEnum;
use crate::value::function_value::FunctionValue;
use crate::value::int_value::IntValue;
use crate::value::pointer_value::PointerValue;
use crate::value::{AsValueRef, MathValue, Value, ValueEnum};
use llvm_sys::core::{LLVMArrayType2, LLVMBuildAdd, LLVMBuildCall2, LLVMBuildFAdd, LLVMBuildFDiv, LLVMBuildFMul, LLVMBuildFSub, LLVMBuildGlobalString, LLVMBuildGlobalStringPtr, LLVMBuildMul, LLVMBuildNSWAdd, LLVMBuildNSWMul, LLVMBuildNSWSub, LLVMBuildNUWAdd, LLVMBuildNUWMul, LLVMBuildNUWSub, LLVMBuildPointerCast, LLVMBuildSub, LLVMCreateBuilderInContext, LLVMIntType, LLVMIntTypeInContext, LLVMPointerType, LLVMPositionBuilderAtEnd, LLVMPrintTypeToString, LLVMPrintValueToString, LLVMTypeOf};
use llvm_sys::prelude::{LLVMBuilderRef, LLVMTypeRef, LLVMValueRef};
use std::ffi::{c_char, c_uint, CString};
use crate::value::float_value::FloatValue;
use popper_mem::array::RawArray;
use popper_mem::string::to_c_str;

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub enum MathOpType {
    NSW,
    NUW,

    #[default]
    None
}

#[derive(Debug, Clone)]
pub struct Builder {
    pub builder: LLVMBuilderRef,
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
    pub fn build_int_add(
        &self,
        lhs: IntValue,
        rhs: IntValue,
        math_op_type: MathOpType,
        name: &str) -> ValueEnum {
        let name = CString::new(name).unwrap();
        unsafe {
            if math_op_type == MathOpType::NSW {
                LLVMBuildNSWAdd(self.builder, lhs.as_raw_ref(), rhs.as_raw_ref(), name.as_ptr()).into()
            } else if math_op_type == MathOpType::NUW {
                LLVMBuildNUWAdd(self.builder, lhs.as_raw_ref(), rhs.as_raw_ref(), name.as_ptr()).into()
            } else {
                LLVMBuildAdd(self.builder, lhs.as_raw_ref(), rhs.as_raw_ref(), name.as_ptr()).into()
            }
        }
    }

    pub fn build_float_add(&self,
                           lhs: FloatValue,
                           rhs: FloatValue,
                           name: &str) -> ValueEnum {
        let name = CString::new(name).unwrap();
        unsafe {
            LLVMBuildFAdd(self.builder, lhs.as_raw_ref(), rhs.as_raw_ref(), name.as_ptr()).into()
        }
    }

    pub fn build_int_sub(
        &self,
        lhs: IntValue,
        rhs: IntValue,
        math_op_type: MathOpType,
        name: &str) -> ValueEnum {
        let name = CString::new(name).unwrap();
        unsafe {
            if math_op_type == MathOpType::NSW {
                LLVMBuildNSWSub(self.builder, lhs.as_raw_ref(), rhs.as_raw_ref(), name.as_ptr()).into()
            } else if math_op_type == MathOpType::NUW {
                LLVMBuildNUWSub(self.builder, lhs.as_raw_ref(), rhs.as_raw_ref(), name.as_ptr()).into()
            } else {
                LLVMBuildSub(self.builder, lhs.as_raw_ref(), rhs.as_raw_ref(), name.as_ptr()).into()
            }
        }
    }

    pub fn build_float_sub(&self,
                           lhs: FloatValue,
                           rhs: FloatValue,
                           name: &str) -> ValueEnum {
        let name = CString::new(name).unwrap();
        unsafe {
            LLVMBuildFSub(self.builder, lhs.as_raw_ref(), rhs.as_raw_ref(), name.as_ptr()).into()
        }
    }

    pub fn build_int_mul(
        &self,
        lhs: IntValue,
        rhs: IntValue,
        math_op_type: MathOpType,
        name: &str) -> ValueEnum {
        let name = to_c_str(name);
        unsafe {
            if math_op_type == MathOpType::NSW {
                LLVMBuildNSWMul(self.builder, lhs.as_raw_ref(), rhs.as_raw_ref(), name.as_ptr()).into()
            } else if math_op_type == MathOpType::NUW {
                LLVMBuildNUWMul(self.builder, lhs.as_raw_ref(), rhs.as_raw_ref(), name.as_ptr()).into()
            } else {
                LLVMBuildMul(self.builder, lhs.as_raw_ref(), rhs.as_raw_ref(), name.as_ptr()).into()
            }
        }
    }

    pub fn build_float_mul(&self,
                           lhs: FloatValue,
                           rhs: FloatValue,
                           name: &str) -> ValueEnum {
        let name = to_c_str(name);
        unsafe {
            LLVMBuildFMul(self.builder, lhs.as_raw_ref(), rhs.as_raw_ref(), name.as_ptr()).into()
        }
    }

    pub fn build_float_div(&self, lhs: &FloatValue, rhs: &FloatValue, name: &str) -> FloatValue {
        let name = to_c_str(name);
        let value =
            unsafe { LLVMBuildFDiv(self.builder, lhs.float_value, rhs.float_value, name.as_ptr()) };
        unsafe { FloatValue::new_llvm_ref(value) }
    }



    pub fn build_call(
        &self,
        function: FunctionValue,
        args: &[ValueEnum],
        name: &str,
    ) -> ValueEnum {
        let mut args = args
            .iter().map(|x: &ValueEnum| x.as_value_ref()).by_ref().collect::<Vec<LLVMValueRef>>();
        let function_type_ref = function.get_raw_function_type().unwrap();
        let length = args.len() as u32;
        let name = to_c_str(name);
        let value = unsafe {
            LLVMBuildCall2(
                self.builder,
                function_type_ref,
                function.as_raw_ref(),
                args.as_mut_ptr(),
                length,
                name.as_ptr()
            )
        };
        value.into()
    }

    unsafe fn build_raw_call(
        &self,
        function_val: LLVMValueRef,
        function_type: LLVMTypeRef,
        args: RawArray<LLVMValueRef>,
        name: RawArray<i8>,
    ) -> LLVMValueRef {
        LLVMBuildCall2(
            self.builder,
            function_type,
            function_val,
            args.as_raw_ptr(),
            args.len() as c_uint,
            name.as_raw_ptr()
        )
    }

    fn build_direct_call(
        &self,
        function: FunctionValue,
        args: Vec<LLVMValueRef>,
        name: CString,
    ) -> LLVMValueRef {
        let args_ptr = args.as_ptr();
        let length = args.len() as u32;
        unsafe {
            LLVMBuildCall2(
                self.builder,
                function.get_type_ref(),
                function.as_raw_ref(),
                args_ptr as *mut _,
                length,
                name.as_ptr() as *const _,
            )
        }
    }

    pub fn build_ret(&self, r: Option<ValueEnum>) {
        unsafe { llvm_sys::core::LLVMBuildRet(self.builder, r
            .map(|x| x.into())
            .unwrap_or(
                std::ptr::null_mut()
            )
        )
        };
    }

    pub fn build_alloca(&self, ty: TypeEnum, name: &str) -> PointerValue {
        let name = to_c_str(name);
        let value = unsafe {
            llvm_sys::core::LLVMBuildAlloca(self.builder, ty.get_type_ref(), name.as_ptr())
        };
        PointerValue::new_llvm_ref(value)
    }

    pub fn build_store(&self, value: ValueEnum, ptr: PointerValue) {
        unsafe { llvm_sys::core::LLVMBuildStore(self.builder, value.into(), ptr.get_llvm_ref()) };
    }

    pub fn build_load(&self, ty: TypeEnum, ptr: PointerValue, name: &str) -> ValueEnum {
        let name = to_c_str(name);
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

    pub fn build_global_string(&self, name: &str, value: &str) -> ValueEnum {
        let name = to_c_str(name);
        let value = to_c_str(value);
        let value = unsafe {
            LLVMBuildGlobalString(self.builder, value.as_ptr(), name.as_ptr())
        };
        value.into()
    }

    pub fn position_at_end(&mut self, basic_block: BasicBlock) {
        unsafe { LLVMPositionBuilderAtEnd(self.builder, basic_block.basic_block) }
    }

    pub fn get_entry_block(&self) -> Option<BasicBlock> {
        self.entry_block
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
