use llvm_sys::analysis::LLVMVerifyModule;
use llvm_sys::bit_reader::LLVMParseBitcodeInContext2;
use llvm_sys::core::{
    LLVMAddFunction, LLVMAppendModuleInlineAsm, LLVMCreateMemoryBufferWithContentsOfFile,
    LLVMDumpModule, LLVMGetNamedFunction, LLVMModuleCreateWithNameInContext,
    LLVMPrintModuleToString, LLVMDumpType
};
use llvm_sys::linker::LLVMLinkModules2;
use llvm_sys::prelude::{LLVMMemoryBufferRef, LLVMModuleRef};
use std::ffi::CString;
use popper_mem::string::to_c_str;

use crate::context::Context;
use crate::types::function_types::FunctionType;
use crate::types::Type;
use crate::value::function_value::FunctionValue;

#[derive(Debug, Copy, Clone)]
pub struct Module {
    pub(crate) module: LLVMModuleRef,
    pub(crate) context: Context,
}

impl Module {
    pub fn from_bc_file(path: &str, context: Context) -> Self {
        let path = std::ffi::CString::new(path).unwrap();
        let module;
        let mut err = std::ptr::null_mut();
        unsafe {
            let mut buf_uninit = std::mem::MaybeUninit::uninit();
            let mut mod_uninit = std::mem::MaybeUninit::uninit();

            let res_buf = LLVMCreateMemoryBufferWithContentsOfFile(
                path.as_ptr(),
                buf_uninit.as_mut_ptr(),
                &mut err,
            );
            if res_buf != 0 {
                assert!(!err.is_null());
                panic!(
                    "Failed to load bitcode: {}",
                    std::ffi::CStr::from_ptr(err).to_str().unwrap()
                );
            }

            let buf: LLVMMemoryBufferRef = buf_uninit.assume_init();

            let result = LLVMParseBitcodeInContext2(context.context, buf, mod_uninit.as_mut_ptr());
            if result != 0 {
                panic!("Failed to load bitcode");
            }

            module = mod_uninit.assume_init();
        }
        Self { module, context }
    }

    pub fn new(name: &str, context: Context) -> Self {
        let name = to_c_str(name);
        let module = unsafe { LLVMModuleCreateWithNameInContext(name.as_ptr(), context.context) };
        Self { module, context }
    }

    pub fn link(&self, other: &Module) {
        let result = unsafe { LLVMLinkModules2(self.module, other.module) };
        if result != 0 {
            panic!("Failed to link modules");
        }
    }

    pub fn get_context(&self) -> Context {
        self.context
    }

    pub fn dump(&self) {
        unsafe { LLVMDumpModule(self.module) }
    }

    pub fn push_asm(&self, asm_code: String) {
        let asm_code = std::ffi::CString::new(asm_code).unwrap();
        let len = asm_code.clone().into_bytes().len();
        unsafe {
            LLVMAppendModuleInlineAsm(self.module, asm_code.as_ptr(), len);
        }
    }

    pub fn add_function(&self, name: &str, function_type: FunctionType) -> FunctionValue {
        let name = to_c_str(name);
        let function =
            unsafe { LLVMAddFunction(self.module, name.as_ptr(), function_type.get_type_ref()) };
        unsafe { FunctionValue::new_llvm_ref(function, Some(function_type.get_type_ref())) }
    }

    pub fn get_function(&self, name: &str) -> Option<FunctionValue> {
        let name = to_c_str(name);
        let function = unsafe { LLVMGetNamedFunction(self.module, name.as_ptr()) };
        if function.is_null() {
            None
        } else {
            Some(unsafe { FunctionValue::new_llvm_ref(function, None) })
        }
    }

    pub fn verify(&self) -> bool {
        unsafe {
            let mut err = std::ptr::null_mut();
            let result = LLVMVerifyModule(
                self.module,
                llvm_sys::analysis::LLVMVerifierFailureAction::LLVMPrintMessageAction,
                &mut err,
            );
            if result != 0 {
                println!("Error: {}", std::ffi::CStr::from_ptr(err).to_str().unwrap());
                false
            } else {
                true
            }
        }
    }

    pub fn print_to_string(&self) -> String {
        unsafe {
            CString::from_raw(LLVMPrintModuleToString(self.module))
                .to_str()
                .unwrap()
                .to_string()
        }
    }
}
