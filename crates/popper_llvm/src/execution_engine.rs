use llvm_sys::execution_engine::{
    LLVMCreateExecutionEngineForModule, LLVMDisposeExecutionEngine, LLVMExecutionEngineRef,
    LLVMFindFunction, LLVMGetFunctionAddress, LLVMLinkInMCJIT,
};

use crate::module::Module;
use crate::value::function_value::FunctionValue;
use llvm_sys::target::{LLVM_InitializeNativeAsmPrinter, LLVM_InitializeNativeTarget};

pub trait UnsafeFunctionPtr: Copy {}

pub struct FunctionPtr<T: UnsafeFunctionPtr + Sized> {
    pub(crate) ptr: *const T,
    pub(crate) innner: ExecutionEngine,
}

#[derive(Debug, Clone)]
pub struct ExecutionEngine {
    pub(crate) execution_engine: LLVMExecutionEngineRef,
}

impl ExecutionEngine {
    fn init() {
        unsafe {
            LLVMLinkInMCJIT();
            assert_eq!(LLVM_InitializeNativeTarget(), 0);
            assert_eq!(LLVM_InitializeNativeAsmPrinter(), 0);
        }
    }
    pub fn new(execution_engine: LLVMExecutionEngineRef) -> Self {
        ExecutionEngine::init();
        Self { execution_engine }
    }

    pub fn new_with_module(module: &Module) -> Self {
        ExecutionEngine::init();
        let execution_engine: LLVMExecutionEngineRef;
        unsafe {
            let mut ee = std::mem::MaybeUninit::uninit();
            let mut err = std::mem::zeroed();
            let result =
                LLVMCreateExecutionEngineForModule(ee.as_mut_ptr(), module.module, &mut err);
            if result != 0 {
                assert!(!err.is_null());
                let err = std::ffi::CStr::from_ptr(err).to_str().unwrap();
                panic!("Failed to create execution engine: {}", err);
            }
            execution_engine = ee.assume_init();
        }

        Self { execution_engine }
    }

    pub fn get_execution_engine(&self) -> LLVMExecutionEngineRef {
        self.execution_engine
    }

    pub fn get_function_address(&self, name: &str) -> u64 {
        let name = std::ffi::CString::new(name).unwrap();
        unsafe { LLVMGetFunctionAddress(self.execution_engine, name.as_ptr()) }
    }

    pub fn get_function<T: Copy + Sized>(&self, name: &str) -> T {
        let _ = self
            .find_function(name)
            .unwrap_or_else(|| panic!("Function {} not found", name));
        let address = self.get_function_address(name);
        assert_eq!(
            address % std::mem::align_of::<T>() as u64,
            0,
            "Function pointer alignment mismatch"
        );
        assert_eq!(
            std::mem::size_of::<T>(),
            std::mem::size_of::<u64>(),
            "Function pointer size mismatch"
        );
        let f: T = unsafe { std::mem::transmute_copy(&address) };
        f
    }

    pub fn find_function(&self, name: &str) -> Option<FunctionValue> {
        let mut f = std::mem::MaybeUninit::uninit();
        let name = std::ffi::CString::new(name).unwrap();
        let result =
            unsafe { LLVMFindFunction(self.execution_engine, name.as_ptr(), f.as_mut_ptr()) };
        if result == 0 {
            Some(unsafe { FunctionValue::new_llvm_ref(f.assume_init(), None) })
        } else {
            None
        }
    }
}

impl Drop for ExecutionEngine {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeExecutionEngine(self.execution_engine);
        }
    }
}
