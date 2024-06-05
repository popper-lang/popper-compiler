use llvm_sys::prelude::LLVMTypeRef;

use crate::context::Context;

use super::{RawType, Type, TypeEnum};



#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VoidType {
    pub(crate) ty: RawType,
}

impl VoidType {
    pub fn new() -> Self {
        let ty = unsafe { llvm_sys::core::LLVMVoidType() };
        Self { ty: RawType::new(ty) }
    }

    pub fn new_with_context(context: Context) -> Self {
        let ty = unsafe { llvm_sys::core::LLVMVoidTypeInContext(context.context) };
        Self { ty: RawType::new(ty) }
    }

    pub unsafe fn new_with_llvm_ref(ty: LLVMTypeRef) -> Self {
        Self { ty: RawType::new(ty) }
    }

}


impl Type for VoidType {
    fn is_sized(&self) -> bool {
        false
    }

    fn as_raw(&self) -> RawType {
        self.ty
    }

    fn to_type_enum(&self) -> TypeEnum {
        TypeEnum::VoidType(*self)
    }
}
