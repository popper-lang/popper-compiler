use crate::types::{Type, TypeEnum};
use llvm_sys::core::*;
use llvm_sys::prelude::LLVMTypeRef;
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FunctionType {
    pub(crate) function_type: LLVMTypeRef,
}

impl FunctionType {
    pub fn new(args_ty: Vec<TypeEnum>, return_ty: TypeEnum, is_var_args: bool) -> Self {
        let mut args_ty: Vec<LLVMTypeRef> =
            args_ty.into_iter().map(|ty| ty.get_type_ref()).collect();
        let function_type = unsafe {
            LLVMFunctionType(
                return_ty.get_type_ref(),
                args_ty.as_mut_ptr(),
                args_ty.len() as u32,
                is_var_args.into(),
            )
        };
        Self { function_type }
    }

    pub fn new_with_llvm_ref(function_type: LLVMTypeRef) -> Self {
        Self { function_type }
    }

    pub fn print_to_string(&self) -> String {
        unsafe {
            dbg!();
            let return_ty = LLVMGetReturnType(self.function_type);
            dbg!();
            let return_c_str = LLVMPrintTypeToString(return_ty) ;
            let return_ty_str = std::ffi::CStr::from_ptr(return_c_str).to_str().unwrap();
            let mut args_str = String::new();
            let num_args = LLVMCountParamTypes(self.function_type);
            for i in 0..num_args {
                let arg_ty: *mut LLVMTypeRef = std::ptr::null_mut();
                LLVMGetParamTypes(self.function_type, arg_ty);
                let arg_c_str = LLVMPrintTypeToString(*arg_ty);
                let arg_ty_str = std::ffi::CStr::from_ptr(arg_c_str).to_str().unwrap();
                args_str.push_str(arg_ty_str);
                if i != num_args - 1 {
                    args_str.push_str(", ");
                }
            }
            dbg!();

            let var_args = LLVMIsFunctionVarArg(self.function_type);
            let var_args_str = if var_args == 1 {
                "varargs"
            } else {
                "no varargs"
            };
            format!("function {} ({}) {}", return_ty_str, args_str, var_args_str)
        }

    }


}

impl Type for FunctionType {
    fn is_sized(&self) -> bool {
        true
    }
    fn get_type_ref(&self) -> LLVMTypeRef {
        self.function_type
    }
    fn to_type_enum(&self) -> TypeEnum {
        TypeEnum::FunctionType(*self)
    }
}
