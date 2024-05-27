use llvm_sys::prelude::LLVMTypeRef;

use crate::context::Context;

use super::{Type, TypeEnum};



#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VoidType {
    pub(crate) ty: LLVMTypeRef,
}

impl VoidType {
    pub fn new() -> Self {
        let ty = unsafe { llvm_sys::core::LLVMVoidType() };
        Self { ty }
    }

    pub fn new_with_context(context: Context) -> Self {
        let ty = unsafe { llvm_sys::core::LLVMVoidTypeInContext(context.context) };
        Self { ty }
    }

    pub unsafe fn new_with_llvm_ref(ty: LLVMTypeRef) -> Self {
        Self { ty }
    }

    pub fn get_llvm_ref(&self) -> LLVMTypeRef {
        self.ty
    }
}


impl Type for VoidType {
    fn is_sized(&self) -> bool {
        false
    }

    fn get_type_ref(&self) -> LLVMTypeRef {
        self.get_llvm_ref()
    }

    fn to_type_enum(&self) -> TypeEnum {
        TypeEnum::VoidType(*self)
    }
}
