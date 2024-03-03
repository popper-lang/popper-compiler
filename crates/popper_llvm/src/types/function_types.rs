use crate::types::{Type, TypeEnum};
use llvm_sys::core::LLVMFunctionType;
use llvm_sys::prelude::LLVMTypeRef;
#[derive(Debug, Copy, Clone)]
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

    pub fn func(&self, args: Vec<TypeEnum>, is_var_args: bool) -> Self {
        FunctionType::new(args, self.to_type_enum(), is_var_args)
    }

    pub fn new_with_llvm_ref(function_type: LLVMTypeRef) -> Self {
        Self { function_type }
    }

    pub fn get_type_ref(&self) -> LLVMTypeRef {
        self.function_type
    }

    pub fn to_type_enum(&self) -> TypeEnum {
        TypeEnum::FunctionType(*self)
    }
}

impl Type for FunctionType {
    fn is_sized(&self) -> bool {
        true
    }
    fn get_type_ref(&self) -> LLVMTypeRef {
        self.function_type
    }
}
