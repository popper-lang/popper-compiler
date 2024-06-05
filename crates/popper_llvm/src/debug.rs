use crate::value::RawValue;
use llvm_sys::core::*;

pub struct DebugLoc {
    debug_loc: RawValue
}

impl DebugLoc {
    pub fn new(debug_loc: RawValue) -> Self {
        DebugLoc {
            debug_loc
        }
    }
    
    pub fn get_line(&self) -> u32 {
        unsafe {
            LLVMGetDebugLocLine(self.debug_loc.as_llvm_ref())
        }
    }
    
    pub fn get_column(&self) -> u32 {
        unsafe {
            LLVMGetDebugLocColumn(self.debug_loc.as_llvm_ref())
        }
    }
    
    pub fn get_directory(&self) -> String {
        unsafe {
            let zero = std::ptr::null_mut();
            let c_str = LLVMGetDebugLocDirectory(self.debug_loc.as_llvm_ref(), zero);
            std::ffi::CStr::from_ptr(c_str).to_str().unwrap().to_string()
        }
    }
    
    pub fn get_filename(&self) -> String {
        unsafe {
            let zero = std::ptr::null_mut();
            let c_str = LLVMGetDebugLocFilename(self.debug_loc.as_llvm_ref(), zero);
            std::ffi::CStr::from_ptr(c_str).to_str().unwrap().to_string()
        }
    }

    pub fn as_raw(&self) -> RawValue {
        self.debug_loc
    }
}