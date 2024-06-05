use std::mem::MaybeUninit;
use crate::types::{Type, TypeEnum};
use llvm_sys::core::*;
use llvm_sys::prelude::LLVMTypeRef;

use super::RawType;
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FunctionType {
    pub(crate) function_type: RawType,
}

impl FunctionType {
    pub fn new(args_ty: Vec<TypeEnum>, return_ty: TypeEnum, is_var_args: bool) -> Self {
        let mut args_ty: Vec<LLVMTypeRef> =
            args_ty.into_iter().map(|ty| ty.as_raw().raw).collect();
        let function_type = unsafe {
            LLVMFunctionType(
                return_ty.as_raw().raw,
                args_ty.as_mut_ptr(),
                args_ty.len() as u32,
                is_var_args.into(),
            )
        };
        Self { function_type: RawType::new(function_type) }
    }

    pub fn new_with_llvm_ref(function_type: LLVMTypeRef) -> Self {
        Self { function_type: RawType::new(function_type) }
    }
    
    pub fn is_var_args(&self) -> bool {
        unsafe { LLVMIsFunctionVarArg(self.function_type.as_llvm_ref()) == 1 }
    }
    
    pub fn get_return_type(&self) -> TypeEnum {
        unsafe { LLVMGetReturnType(self.function_type.as_llvm_ref()) }.into()
    }
    
    pub fn count_param_types(&self) -> u32 {
        unsafe { LLVMCountParamTypes(self.function_type.as_llvm_ref()) }
    }
    
    pub fn get_param_types(&self) -> Vec<TypeEnum> {
        let length = self.count_param_types();
        let slice: *mut LLVMTypeRef = std::ptr::null_mut();
        unsafe { LLVMGetParamTypes(self.function_type.as_llvm_ref(), slice) };
        let slice = unsafe { std::slice::from_raw_parts(slice, length as usize) };
        slice.iter().map(|&ty| ty.into()).collect()
    }


}

impl Type for FunctionType {
    fn is_sized(&self) -> bool {
        true
    }
    fn as_raw(&self) -> RawType {
        self.function_type
    }
    fn to_type_enum(&self) -> TypeEnum {
        TypeEnum::FunctionType(*self)
    }
}
