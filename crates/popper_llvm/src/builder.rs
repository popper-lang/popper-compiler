use crate::basic_block::BasicBlock;
use crate::context::Context;
use crate::types::TypeEnum;
use crate::value::function_value::FunctionValue;
use crate::value::int_value::IntValue;
use crate::value::pointer_value::PointerValue;
use crate::value::{Value, ValueEnum};
use llvm_sys::core::{LLVMArrayType2, LLVMBuildAdd, LLVMBuildCall2, LLVMBuildFAdd, LLVMBuildFDiv, LLVMBuildFMul, LLVMBuildFSub, LLVMBuildGlobalString, LLVMBuildGlobalStringPtr, LLVMBuildMul, LLVMBuildNSWAdd, LLVMBuildNSWMul, LLVMBuildNSWSub, LLVMBuildNUWAdd, LLVMBuildNUWMul, LLVMBuildNUWSub, LLVMBuildPointerCast, LLVMBuildSub, LLVMCreateBuilderInContext, LLVMIntType, LLVMIntTypeInContext, LLVMPointerType, LLVMPositionBuilderAtEnd, LLVMPrintTypeToString, LLVMPrintValueToString, LLVMTypeOf};
use llvm_sys::prelude::{LLVMBuilderRef, LLVMTypeRef, LLVMValueRef};
use std::ffi::{c_char, c_uint, CString};
use crate::value::float_value::FloatValue;
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
                LLVMBuildNSWAdd(self.builder, lhs.as_raw().as_llvm_ref(), rhs.as_raw().as_llvm_ref(), name.as_ptr()).into()
            } else if math_op_type == MathOpType::NUW {
                LLVMBuildNUWAdd(self.builder, lhs.as_raw().as_llvm_ref(), rhs.as_raw().as_llvm_ref(), name.as_ptr()).into()
            } else {
                LLVMBuildAdd(self.builder, lhs.as_raw().as_llvm_ref(), rhs.as_raw().as_llvm_ref(), name.as_ptr()).into()
            }
        }
    }

    pub fn build_float_add(&self,
                           lhs: FloatValue,
                           rhs: FloatValue,
                           name: &str) -> ValueEnum {
        let name = CString::new(name).unwrap();
        unsafe {
            LLVMBuildFAdd(self.builder, lhs.as_raw().as_llvm_ref(), rhs.as_raw().as_llvm_ref(), name.as_ptr()).into()
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
                LLVMBuildNSWSub(self.builder, lhs.as_raw().as_llvm_ref(), rhs.as_raw().as_llvm_ref(), name.as_ptr()).into()
            } else if math_op_type == MathOpType::NUW {
                LLVMBuildNUWSub(self.builder, lhs.as_raw().as_llvm_ref(), rhs.as_raw().as_llvm_ref(), name.as_ptr()).into()
            } else {
                LLVMBuildSub(self.builder, lhs.as_raw().as_llvm_ref(), rhs.as_raw().as_llvm_ref(), name.as_ptr()).into()
            }
        }
    }

    pub fn build_float_sub(&self,
                           lhs: FloatValue,
                           rhs: FloatValue,
                           name: &str) -> ValueEnum {
        let name = CString::new(name).unwrap();
        unsafe {
            LLVMBuildFSub(self.builder, lhs.as_raw().as_llvm_ref(), rhs.as_raw().as_llvm_ref(), name.as_ptr()).into()
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
                LLVMBuildNSWMul(self.builder, lhs.as_raw().as_llvm_ref(), rhs.as_raw().as_llvm_ref(), name.as_ptr()).into()
            } else if math_op_type == MathOpType::NUW {
                LLVMBuildNUWMul(self.builder, lhs.as_raw().as_llvm_ref(), rhs.as_raw().as_llvm_ref(), name.as_ptr()).into()
            } else {
                LLVMBuildMul(self.builder, lhs.as_raw().as_llvm_ref(), rhs.as_raw().as_llvm_ref(), name.as_ptr()).into()
            }
        }
    }

    pub fn build_float_mul(&self,
                           lhs: FloatValue,
                           rhs: FloatValue,
                           name: &str) -> ValueEnum {
        let name = to_c_str(name);
        unsafe {
            LLVMBuildFMul(self.builder, lhs.as_raw().as_llvm_ref(), rhs.as_raw().as_llvm_ref(), name.as_ptr()).into()
        }
    }

    pub fn build_float_div(&self, lhs: &FloatValue, rhs: &FloatValue, name: &str) -> FloatValue {
        let name = to_c_str(name);
        let value =
            unsafe { LLVMBuildFDiv(self.builder, lhs.float_value.as_llvm_ref(), rhs.float_value.as_llvm_ref(), name.as_ptr()) };
        unsafe { FloatValue::new_llvm_ref(value) }
    }



    pub fn build_call(
        &self,
        function: FunctionValue,
        args: &[ValueEnum],
        name: &str,
    ) -> ValueEnum {
        let mut args = args
            .iter().map(|x: &ValueEnum| x.as_raw().as_llvm_ref()).by_ref().collect::<Vec<LLVMValueRef>>();
        let function_type_ref = function.get_raw_function_type().unwrap();
        let length = args.len() as u32;
        let name = to_c_str(name);
        let value = unsafe {
            LLVMBuildCall2(
                self.builder,
                function_type_ref,
                function.as_raw().as_llvm_ref(),
                args.as_mut_ptr(),
                length,
                name.as_ptr()
            )
        };
        value.into()
    }
    
    pub fn build_ret(&self, r: Option<ValueEnum>) {
        unsafe { llvm_sys::core::LLVMBuildRet(self.builder, r
            .map(|x| x.as_raw().as_llvm_ref())
            .unwrap_or(
                std::ptr::null_mut()
            )
        )
        };
    }

    pub fn build_alloca(&self, ty: TypeEnum, name: &str) -> PointerValue {
        let name = to_c_str(name);
        let value = unsafe {
            llvm_sys::core::LLVMBuildAlloca(self.builder, ty.as_raw().as_llvm_ref(), name.as_ptr())
        };
        PointerValue::new_llvm_ref(value)
    }

    pub fn build_store(&self, value: ValueEnum, ptr: PointerValue) {
        unsafe { llvm_sys::core::LLVMBuildStore(self.builder, value.into(), ptr.as_raw().as_llvm_ref()) };
    }

    pub fn build_load(&self, ty: TypeEnum, ptr: PointerValue, name: &str) -> ValueEnum {
        let name = to_c_str(name);
        let value = unsafe {
            llvm_sys::core::LLVMBuildLoad2(
                self.builder,
                ty.as_raw().as_llvm_ref(),
                ptr.as_raw().as_llvm_ref(),
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
    
    pub fn build_get_element_ptr(&self, ty: TypeEnum, ptr: PointerValue, indices: &[IntValue], name: &str) -> ValueEnum {
        let name = to_c_str(name);
        let mut indices = indices
            .iter().map(|x: &IntValue| x.as_raw().as_llvm_ref()).by_ref().collect::<Vec<LLVMValueRef>>();
        let value = unsafe {
            llvm_sys::core::LLVMBuildGEP2(
                self.builder,
                ty.as_raw().as_llvm_ref(),
                ptr.as_raw().as_llvm_ref(),
                indices.as_mut_ptr(),
                indices.len() as u32,
                name.as_ptr(),
            )
        };
        ValueEnum::from(value)
    }
    
    pub fn build_inbound_get_element_ptr(&self, ty: TypeEnum, ptr: PointerValue, indices: &[IntValue], name: &str) -> ValueEnum {
        let name = to_c_str(name);
        let mut indices = indices
            .iter().map(|x: &IntValue| x.as_raw().as_llvm_ref()).by_ref().collect::<Vec<LLVMValueRef>>();
        let value = unsafe {
            llvm_sys::core::LLVMBuildInBoundsGEP2(
                self.builder,
                ty.as_raw().as_llvm_ref(),
                ptr.as_raw().as_llvm_ref(),
                indices.as_mut_ptr(),
                indices.len() as u32,
                name.as_ptr(),
            )
        };
        ValueEnum::from(value)
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
