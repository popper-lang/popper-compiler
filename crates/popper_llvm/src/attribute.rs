use llvm_sys::prelude::*;
use llvm_sys::core::*;
use crate::types::RawType;


#[derive(Debug, Copy, Clone)]
pub struct Attribute {
    attribute: LLVMAttributeRef
}

impl Attribute {
    pub fn new(lref: LLVMAttributeRef) -> Self {
        Attribute {
            attribute: lref
        }
    }
    
    pub fn get_kind(&self) -> u32 {
        unsafe {
            LLVMGetEnumAttributeKind(self.attribute)
        }
    }
    
    pub fn get_value(&self) -> u64 {
        unsafe {
            LLVMGetEnumAttributeValue(self.attribute)
        }
    }
    
    pub fn get_type(&self) -> RawType {
        unsafe {
            RawType::new(
                LLVMGetTypeAttributeValue(self.attribute)
            )
        }
    }
    
    pub fn get_string_kind(&self) -> String {
        unsafe {
            let zero = std::ptr::null_mut();
            let c_str = LLVMGetStringAttributeKind(self.attribute, zero);
            std::ffi::CStr::from_ptr(c_str).to_str().unwrap().to_string()
        }
    }
    
    pub fn get_string_value(&self) -> String {
        unsafe {
            let zero = std::ptr::null_mut();
            let c_str = LLVMGetStringAttributeValue(self.attribute, zero);
            std::ffi::CStr::from_ptr(c_str).to_str().unwrap().to_string()
        }
    }
    
    pub fn is_enum_kind(&self) -> bool {
        unsafe {
            LLVMIsEnumAttribute(self.attribute) == 1
        }
    }
    
    pub fn is_string_kind(&self) -> bool {
        unsafe {
            LLVMIsStringAttribute(self.attribute) == 1
        }
    }
    
    pub fn is_type_kind(&self) -> bool {
        unsafe {
            LLVMIsTypeAttribute(self.attribute) == 1
        }
    }
    
    
    
    
}
