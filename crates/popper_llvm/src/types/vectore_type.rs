use crate::types::{RawType, Type, TypeEnum};
use llvm_sys::core::*;
use llvm_sys::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VectoreType {
    pub(crate) vectore_type: RawType,
}

impl VectoreType {
    pub fn new(element_type: TypeEnum, size: u32) -> Self {
        let vectore_type = unsafe { LLVMVectorType(element_type.as_raw().as_llvm_ref(), size) };
        Self { vectore_type: RawType::new(vectore_type) }
    }
    
    pub fn new_llvm_ref(vectore_type: LLVMTypeRef) -> Self {
        Self { vectore_type: RawType::new(vectore_type) }
    }

    pub fn get_size(&self) -> u32 {
        unsafe { LLVMGetVectorSize(self.vectore_type.raw) }
    }

    pub fn get_element_type(&self) -> RawType {
        let element_type = unsafe { LLVMGetElementType(self.vectore_type.raw) };
        RawType::new(element_type)
    }
}

impl Type for VectoreType {
    fn is_sized(&self) -> bool {
        true
    }

    fn as_raw(&self) -> RawType {
        self.vectore_type
    }

    fn to_type_enum(&self) -> TypeEnum {
        TypeEnum::VectoreType(*self)
    }
}