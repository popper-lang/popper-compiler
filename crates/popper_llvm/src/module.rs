use llvm_sys::analysis::LLVMVerifyModule;
use llvm_sys::bit_reader::LLVMParseBitcodeInContext2;
use llvm_sys::core::*;
use llvm_sys::linker::LLVMLinkModules2;
use llvm_sys::prelude::*;
use std::ffi::CString;
use llvm_sys::LLVMModuleFlagBehavior;
use crate::util::to_c_str;

use crate::analysis::FailureAction;
use crate::context::Context;
use crate::metadata::{Metadata, NamedMetadata};
use crate::types::function_types::FunctionType;
use crate::types::{Type, TypeEnum};
use crate::util::ptr_to_option;
use crate::value::function_value::FunctionValue;
use crate::value::{RawValue, Value, ValueEnum};

type LLVMModuleFlagEntryRef = *mut LLVMModuleFlagEntry;

#[derive(Debug, Copy, Clone)]
pub enum ModuleFlagBehavior {
    Error,
    Warning,
    Require,
    Override,
    Append,
    AppendUnique,
}


impl From<LLVMModuleFlagBehavior> for ModuleFlagBehavior {
    fn from(value: LLVMModuleFlagBehavior) -> Self {
        match value {
            LLVMModuleFlagBehavior::LLVMModuleFlagBehaviorError => ModuleFlagBehavior::Error,
            LLVMModuleFlagBehavior::LLVMModuleFlagBehaviorWarning => ModuleFlagBehavior::Warning,
            LLVMModuleFlagBehavior::LLVMModuleFlagBehaviorRequire => ModuleFlagBehavior::Require,
            LLVMModuleFlagBehavior::LLVMModuleFlagBehaviorOverride => ModuleFlagBehavior::Override,
            LLVMModuleFlagBehavior::LLVMModuleFlagBehaviorAppend => ModuleFlagBehavior::Append,
            LLVMModuleFlagBehavior::LLVMModuleFlagBehaviorAppendUnique => ModuleFlagBehavior::AppendUnique,
        }
    }
}


impl From<ModuleFlagBehavior> for LLVMModuleFlagBehavior {
    fn from(value: ModuleFlagBehavior) -> Self {
        match value {
            ModuleFlagBehavior::Error => LLVMModuleFlagBehavior::LLVMModuleFlagBehaviorError,
            ModuleFlagBehavior::Warning => LLVMModuleFlagBehavior::LLVMModuleFlagBehaviorWarning,
            ModuleFlagBehavior::Require => LLVMModuleFlagBehavior::LLVMModuleFlagBehaviorRequire,
            ModuleFlagBehavior::Override => LLVMModuleFlagBehavior::LLVMModuleFlagBehaviorOverride,
            ModuleFlagBehavior::Append => LLVMModuleFlagBehavior::LLVMModuleFlagBehaviorAppend,
            ModuleFlagBehavior::AppendUnique => LLVMModuleFlagBehavior::LLVMModuleFlagBehaviorAppendUnique,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ModuleFlagEntry {
    entry: LLVMModuleFlagEntryRef,
}

impl ModuleFlagEntry {
    pub fn new(entry: LLVMModuleFlagEntryRef) -> Self {
        Self { entry }
    }
    
    pub fn get_flag_behavior(&self, index: u32) -> ModuleFlagBehavior {
        unsafe {
            LLVMModuleFlagEntriesGetFlagBehavior(self.entry, index).into()
        }
    }

    pub fn get_key(&self, index: u32) -> Option<String> {
        unsafe {
            let mut len = 0;
            let c_str = ptr_to_option(
                LLVMModuleFlagEntriesGetKey(self.entry, index, &mut len)
            )?;
            Some(std::ffi::CStr::from_ptr(c_str).to_str().unwrap().to_string())
        }
    }

    pub fn get_meta_data(&self, index: u32) -> Option<Metadata> {
        let meta = unsafe {
            ptr_to_option(
                LLVMModuleFlagEntriesGetMetadata(self.entry, index)
            )?
        };
        Some(Metadata::new(meta))
    }


}

#[derive(Debug, Copy)]
pub struct Module {
    pub(crate) module: LLVMModuleRef,
    pub(crate) context: Context,
}

impl Module {
    pub fn from_bc_file(path: &str, context: Context) -> Self {
        let path = CString::new(path).unwrap();
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

    pub fn get_inline_asm(&self) -> String {
        let zero = std::ptr::null_mut();
        let c_str = unsafe { LLVMGetModuleInlineAsm(self.module, zero) };
        unsafe { std::ffi::CStr::from_ptr(c_str).to_str().unwrap().to_string() }
    }

    pub fn set_inline_asm(&self, asm: &str) {
        let len = asm.len();
        let c_str = to_c_str(asm);
        unsafe {
            LLVMSetModuleInlineAsm2(self.module, c_str.as_ptr(), len);
        }
    }

    pub fn push_asm(&self, asm_code: &str) {
        let len = asm_code.len();
        let c_str = to_c_str(asm_code);
        unsafe {
            LLVMAppendModuleInlineAsm(self.module, c_str.as_ptr(), len);
        }
    }

    pub fn add_function(&self, name: &str, function_type: FunctionType) -> FunctionValue {
        let name = to_c_str(name);
        let function =
            unsafe { LLVMAddFunction(self.module, name.as_ptr(), function_type.as_raw().as_llvm_ref()) };
        unsafe { FunctionValue::new_llvm_ref(function, Some(function_type.as_raw().as_llvm_ref())) }
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

    pub fn verify(&self, action: FailureAction) -> bool {
        unsafe {
            let mut err = std::ptr::null_mut();
            let result = LLVMVerifyModule(
                self.module,
                action.into(),
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
    
    pub fn add_alias(&self, ty: TypeEnum, aliasee: ValueEnum, address_space: u32, name: &str) -> ValueEnum {
        let name = to_c_str(name);
        let value = unsafe {
            LLVMAddAlias2(
                self.module,
                ty.as_raw().as_llvm_ref(),
                address_space,
                aliasee.as_llvm_ref(),
                name.as_ptr()
            )
        };
        ValueEnum::from(value)
    }
    
    pub fn add_global(&self, ty: TypeEnum, name: &str) -> ValueEnum {
        let name = to_c_str(name);
        let value = unsafe {
            LLVMAddGlobal(self.module, ty.as_raw().as_llvm_ref(), name.as_ptr())
        };
        ValueEnum::from(value)
    }

    pub fn print_to_string(&self) -> String {
        unsafe {
            CString::from_raw(LLVMPrintModuleToString(self.module))
                .to_str()
                .unwrap()
                .to_string()
        }
    }

    pub fn set_name(&self, name: &str) {
        let length = name.len();
        let name = to_c_str(name);
        unsafe {
            LLVMSetModuleIdentifier(self.module, name.as_ptr(), length);
        }
    }

    pub fn get_source_file(&self) -> Option<String> {
        let mut length = 0;
        let c_str = ptr_to_option(unsafe { LLVMGetSourceFileName(self.module, &mut length) })?;
        Some(unsafe { std::ffi::CStr::from_ptr(c_str).to_str().unwrap().to_string() })
    }

    pub fn set_source_file(&self, file: &str) {
        let length = file.len();
        let file = to_c_str(file);
        unsafe {
            LLVMSetSourceFileName(self.module, file.as_ptr(), length);
        }
    }
    
    pub fn get_data_layout_str(&self) -> String {
        unsafe {
            let c_str = LLVMGetDataLayoutStr(self.module);
            std::ffi::CStr::from_ptr(c_str).to_str().unwrap().to_string()
        }
    }
    
    pub fn set_data_layout_str(&self, data_layout: &str) {
        let data_layout = to_c_str(data_layout);
        unsafe {
            LLVMSetDataLayout(self.module, data_layout.as_ptr());
        }
    }

    pub fn get_target_triple(&self) -> String {
        unsafe {
            let c_str = LLVMGetTarget(self.module);
            std::ffi::CStr::from_ptr(c_str).to_str().unwrap().to_string()
        }
    }

    pub fn set_target_triple(&self, target_triple: &str) {
        let target_triple = to_c_str(target_triple);
        unsafe {
            LLVMSetTarget(self.module, target_triple.as_ptr());
        }
    }

    pub fn copy_flag_metadata(&self) -> ModuleFlagEntry {
        let zero = std::ptr::null_mut();
        let entry = unsafe { LLVMCopyModuleFlagsMetadata(self.module, zero) };
        ModuleFlagEntry::new(entry)
    }

    pub fn get_flag(&self, key: &str) -> Option<Metadata> {
        let length = key.len();
        let key = to_c_str(key);
        let meta = unsafe { LLVMGetModuleFlag(self.module, key.as_ptr(), length) };
        ptr_to_option(meta)
            .map(Metadata::new)
    }

    pub fn add_flag(&self, key: &str, behavior: ModuleFlagBehavior, meta: Metadata) {
        let length = key.len();
        let key = to_c_str(key);
        unsafe {
            LLVMAddModuleFlag(
                self.module,
                behavior.into(),
                key.as_ptr(),
                length,
                meta.as_llvm_ref(),
            );
        }
    }

    pub fn get_first_named_metadata(&self) -> Option<NamedMetadata> {
        let meta = unsafe { LLVMGetFirstNamedMetadata(self.module) };
        ptr_to_option(meta)
            .map(NamedMetadata::new)
    }

    pub fn get_last_named_metadata(&self) -> Option<NamedMetadata> {
        let meta = unsafe { LLVMGetLastNamedMetadata(self.module) };
        ptr_to_option(meta)
            .map(NamedMetadata::new)
    }

    pub fn get_named_metadata(&self, name: &str) -> Option<NamedMetadata> {
        let length = name.len();
        let name = to_c_str(name);
        let meta = unsafe { LLVMGetNamedMetadata(self.module, name.as_ptr(), length) };
        ptr_to_option(meta)
            .map(NamedMetadata::new)
    }

    pub fn get_or_insert_named_metadata(&self, name: &str) -> NamedMetadata {
        let length = name.len();
        let name = to_c_str(name);
        let meta = unsafe { LLVMGetOrInsertNamedMetadata(self.module, name.as_ptr(), length) };
        NamedMetadata::new(meta)
    }

    pub fn get_num_named_metadata_operand(&self, name: &str) -> u32 {
        let name = to_c_str(name);
        unsafe { LLVMGetNamedMetadataNumOperands(self.module, name.as_ptr()) }
    }

    pub fn get_named_metadata_operand(&self, name: &str) -> RawValue {
        let mut val = std::mem::MaybeUninit::uninit();
        let name = to_c_str(name);
        unsafe { LLVMGetNamedMetadataOperands(self.module, name.as_ptr(), val.as_mut_ptr()) };

        let val = unsafe { val.assume_init() };
        RawValue::new(val)
    }

    pub fn add_named_metadata_operand(&self, name: &str, val: RawValue) {
        let name = to_c_str(name);
        unsafe { LLVMAddNamedMetadataOperand(self.module, name.as_ptr(), val.as_llvm_ref()) };
    }

    pub fn get_first_function(&self) -> Option<FunctionValue> {
        let val = unsafe {
            LLVMGetFirstFunction(self.module)
        };
        
        ptr_to_option(val)
            .map(|x| unsafe { FunctionValue::new_llvm_ref(x, None) })
    }
    
    pub fn get_last_function(&self) -> Option<FunctionValue> {
        let val = unsafe {
            LLVMGetLastFunction(self.module)
        };
        
        ptr_to_option(val)
            .map(|x| unsafe { FunctionValue::new_llvm_ref(x, None) })
    }
    
    pub fn get_next_function(&self, function: FunctionValue) -> Option<FunctionValue> {
        let val = unsafe {
            LLVMGetNextFunction(function
                .as_raw()
                .as_llvm_ref()
            )
        };
        
        ptr_to_option(val)
            .map(|x| unsafe { FunctionValue::new_llvm_ref(x, None) })
    }
    
    pub fn get_previous_function(&self, function: FunctionValue) -> Option<FunctionValue> {
        let val = unsafe {
            LLVMGetPreviousFunction(function
                .as_raw()
                .as_llvm_ref()
            )
        };
        
        ptr_to_option(val)
            .map(|x| unsafe { FunctionValue::new_llvm_ref(x, None) })
    }


}

impl Clone for Module {
    fn clone(&self) -> Self {
        Self {
            context: self.context,
            module: unsafe { LLVMCloneModule(self.module) },
        }
    }
}