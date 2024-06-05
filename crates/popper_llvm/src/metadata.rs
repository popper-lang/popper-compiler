use llvm_sys::prelude::*;
use llvm_sys::core::*;

#[derive(Debug, Copy, Clone)]
pub struct NamedMetadata {
    named_metadata: LLVMNamedMDNodeRef
}

impl NamedMetadata {
    pub fn new(lref: LLVMNamedMDNodeRef) -> Self {
        NamedMetadata {
            named_metadata: lref
        }
    }
    
    pub fn as_llvm_ref(&self) -> LLVMNamedMDNodeRef {
        self.named_metadata
    }
    
    pub fn get_name(&self) -> String {
        unsafe {
            let c_str = LLVMGetNamedMetadataName(self.named_metadata, std::ptr::null_mut());
            std::ffi::CStr::from_ptr(c_str).to_str().unwrap().to_string()
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Metadata {
    metadata: LLVMMetadataRef
}

impl Metadata {
    pub fn new(lref: LLVMMetadataRef) -> Self {
        Metadata {
            metadata: lref
        }
    }
    
    pub fn as_llvm_ref(&self) -> LLVMMetadataRef {
        self.metadata
    }
}