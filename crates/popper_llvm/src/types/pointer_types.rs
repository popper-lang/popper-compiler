use crate::types::{Type, TypeEnum};
use llvm_sys::prelude::LLVMTypeRef;
use llvm_sys::core::*;

use super::RawType;
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PointerTypes {
    pub(crate) ty: RawType,
}

impl PointerTypes {
    pub fn new_const(ty: TypeEnum) -> Self {
        let ty = unsafe { LLVMPointerType(ty.as_raw().as_llvm_ref(), 0) };
        Self { ty: RawType::new(ty) }
    }
    pub fn new_llvm_ref(ty: LLVMTypeRef) -> Self {
        Self { ty: RawType::new(ty) }
    }
    
    pub fn is_opaque(&self) -> bool {
        unsafe { LLVMPointerTypeIsOpaque(self.ty.as_llvm_ref()) == 1 }
    }
    
    pub fn get_address_space(&self) -> u32 {
        unsafe { LLVMGetPointerAddressSpace(self.ty.as_llvm_ref()) }
    }
}

impl Type for PointerTypes {
    fn is_sized(&self) -> bool {
        false
    }

    fn as_raw(&self) -> RawType {
        self.ty
    }

    fn to_type_enum(&self) -> TypeEnum {
        TypeEnum::PointerType(*self)
    }
}
