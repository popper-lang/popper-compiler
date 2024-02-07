use llvm_sys::prelude::{
    LLVMValueRef,
    LLVMTypeRef
};
use llvm_sys::core::{
    LLVMTypeOf,
    LLVMGetParam,
    LLVMGetValueName2 as LLVMGetValueName
};
use crate::module::Module;

use crate::types::{function_types, TypeEnum};

use crate::value::{Value, ValueEnum};

#[derive(Debug, Copy, Clone)]
pub struct FunctionValue {
    pub(crate) function_value: LLVMValueRef,
    pub(crate) function_type: function_types::FunctionType,
}

impl FunctionValue {

    pub fn new_llvm_ref(lref: LLVMValueRef) -> Self {
        let function_type = function_types::FunctionType::new_with_llvm_ref(unsafe { LLVMTypeOf(lref) });
        Self { function_value: lref, function_type }
    }
    pub fn new_constant(function_type: function_types::FunctionType, module: Module, name: &str) -> Self {
        let function_value = unsafe {
            llvm_sys::core::LLVMAddFunction(
                module.module,
                name.as_ptr() as *const i8,
                function_type.function_type,
            )
        };
        Self {
            function_value,
            function_type,
        }
    }

    pub fn get_name(&self) -> &str {
        use std::ffi::CStr;
        use std::str;

        unsafe {
            let ptr = LLVMGetValueName(self.function_value, std::ptr::null_mut());
            let cstr = CStr::from_ptr(ptr);
            str::from_utf8_unchecked(cstr.to_bytes())
        }
    }

    pub fn get_nth_param(&self, index: u32) -> Option<ValueEnum> {
        let param = unsafe { LLVMGetParam(self.function_value, index) };
        if param.is_null() {
            None
        } else {
            Some(param.into())
        }
    }

    pub fn verify(&self) -> bool {
        let result = unsafe { llvm_sys::analysis::LLVMVerifyFunction(self.function_value, llvm_sys::analysis::LLVMVerifierFailureAction::LLVMPrintMessageAction) };
        result == 0
    }

}

impl Value for FunctionValue {
    fn get_type_ref(&self) -> LLVMTypeRef {
        self.function_type.function_type
    }

    fn get_type(&self) -> TypeEnum {
        TypeEnum::FunctionType(self.function_type)
    }

    fn as_value_ref(&self) -> LLVMValueRef {
        self.function_value
    }

    fn is_null_or_undef(&self) -> bool {
        false
    }

    fn is_const(&self) -> bool {
        false
    }

    fn is_null(&self) -> bool {
        false
    }

    fn is_undef(&self) -> bool {
        false
    }
}