use std::fmt::Debug;
use llvm_sys::prelude::LLVMTypeRef;
use llvm_sys::core::{
    LLVMGetElementType,
    LLVMArrayType2 as LLVMArrayType,
    LLVMGetArrayLength2 as LLVMGetArrayLength
};

use crate::types::{Type, TypeEnum};


#[derive(Clone, Copy)]
pub struct ArrayType {
    pub(crate) array_type: LLVMTypeRef,
    pub(crate) size: u64,
}

impl Debug for ArrayType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ArrayType")
            .field("array_type", &self.array_type)
            .field("size", &self.size)
            .finish()
    }
}

impl ArrayType {
    pub fn new(element_type: TypeEnum, size: u64) -> Self {
        let array_type = unsafe { LLVMArrayType(element_type.get_type_ref(), size) };
        Self { array_type, size }
    }

    pub fn new_with_llvm_ref(array_type: LLVMTypeRef) -> Self {
        Self { array_type, size: unsafe { LLVMGetArrayLength(array_type) }}
    }

    pub fn get_size(&self) -> u64 {
        self.size
    }

    pub fn get_element_type(&self) -> TypeEnum {
        let element_type = unsafe { LLVMGetElementType(self.array_type) };
        element_type.into()
    }
}

impl Type for ArrayType {
    fn is_sized(&self) -> bool {
        true
    }
    fn get_type_ref(&self) -> LLVMTypeRef {
        self.array_type
    }
}