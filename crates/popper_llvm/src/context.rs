use crate::basic_block::BasicBlock;
use crate::builder::Builder;
use llvm_sys::core::*;
use llvm_sys::prelude::*;
use crate::attribute::Attribute;

use crate::module::Module;
use crate::types::{float_types, int_types, RawType, TypeEnum};
use crate::types::struct_type::StructType;
use crate::util::to_c_str;
use crate::value::array_value::ArrayValue;
use crate::value::function_value::FunctionValue;
use crate::value::{Value, ValueEnum};

#[derive(Clone, Copy, Debug)]
pub struct Context {
    pub(crate) context: LLVMContextRef,
}

impl Context {

    pub fn get_global_context() -> Self {
        let context = unsafe { LLVMGetGlobalContext() };
        Self { context }
    }

    pub fn create() -> Self {
        let context = unsafe { LLVMContextCreate() };
        Self { context }
    }
    
    pub fn new(context: LLVMContextRef) -> Self {
        Self { context }
    }
    

    pub fn new_module(&self, name: &str) -> Module {
        Module::new(name, *self)
    }

    pub fn new_builder(&self) -> Builder {
         Builder::new(*self)
    }
    
    pub fn i1_type(&self) -> int_types::IntType {
        int_types::IntType::new_with_context(1, *self)
    }
    pub fn bool_type(&self) -> int_types::IntType {
        self.i1_type()
    }

    pub fn i8_type(&self) -> int_types::IntType {
        int_types::IntType::new_with_context(8, *self)
    }

    pub fn i16_type(&self) -> int_types::IntType {
        int_types::IntType::new_with_context(16, *self)
    }

    pub fn i32_type(&self) -> int_types::IntType {
        int_types::IntType::new_with_context(32, *self)
    }

    pub fn i64_type(&self) -> int_types::IntType {
        int_types::IntType::new_with_context(64, *self)
    }

    pub fn float_type(&self) -> float_types::FloatType {
        float_types::FloatType::new_with_context(*self)
    }

    pub fn const_string(&self, s: &str) -> ValueEnum {
        let s = std::ffi::CString::new(s).unwrap();
        unsafe {
            ValueEnum::ArrayValue(
                ArrayValue::new_llvm_ref(
                    LLVMConstStringInContext(self.context, s.as_ptr(), s.as_bytes().len() as u32, 0)
                )
            )
        }
    }
    
    pub fn struct_type(&self, element_types: &[TypeEnum], is_packed: bool) -> StructType {
        let mut element_types: Vec<LLVMTypeRef> = element_types.iter().map(|ty| ty.as_raw().as_llvm_ref()).collect();
        let struct_type = unsafe {
            LLVMStructTypeInContext(
                self.context,
                element_types.as_mut_ptr(),
                element_types.len() as u32,
                is_packed as i32
            )
        };
        StructType::new_with_llvm_ref(struct_type)
    }
    
    pub fn named_struct_type(&self, name: &str) -> StructType {
        let name = to_c_str(name);
        let struct_type = unsafe { LLVMStructCreateNamed(self.context, name.as_ptr()) };
        StructType::new_with_llvm_ref(struct_type)
    }
    
    pub fn void_type(&self) -> RawType {
        RawType::new(unsafe { LLVMVoidTypeInContext(self.context) })
    }
    
    pub fn label_type(&self) -> RawType {
        RawType::new(unsafe { LLVMLabelTypeInContext(self.context) })
    }
    
    pub fn x86_fp80_type(&self) -> RawType {
        RawType::new(unsafe { LLVMX86FP80TypeInContext(self.context) })
    }
    
    pub fn fp128_type(&self) -> RawType {
        RawType::new(unsafe { LLVMFP128TypeInContext(self.context) })
    }
    
    pub fn ppc_fp128_type(&self) -> RawType {
        RawType::new(unsafe { LLVMPPCFP128TypeInContext(self.context) })
    }
    pub fn target_ext_type(&self, name: &str, params: &[RawType], int_params: &[u32]) -> RawType {
        let name = to_c_str(name);
        let mut params: Vec<LLVMTypeRef> = params.iter().map(|ty| ty.as_llvm_ref()).collect();
        let mut int_params = int_params.to_vec();
        let ty = unsafe {
            LLVMTargetExtTypeInContext(
                self.context,
                name.as_ptr(),
                params.as_mut_ptr(),
                params.len() as u32,
                int_params.as_mut_ptr(),
                int_params.len() as u32
            )
        };
        RawType::new(ty)
    }
    pub fn append_basic_block(&self, name: &str, fn_value: FunctionValue) -> BasicBlock {
        let name = std::ffi::CString::new(name).unwrap();
        let block = unsafe {
            LLVMAppendBasicBlockInContext(self.context, fn_value.as_raw().as_llvm_ref(), name.as_ptr())
        };
        BasicBlock::new(block)
    }
    
    pub fn should_discard_value_names(&self) -> bool {
        unsafe { LLVMContextShouldDiscardValueNames(self.context) == 1 }
    }
    
    pub fn set_discard_value_names(&self, discard: bool) {
        unsafe { LLVMContextSetDiscardValueNames(self.context, discard as i32) }
    }
    
    pub fn get_md_kind_id(&self, name: &str) -> u32 {
        let length = name.len() as u32;
        let name = to_c_str(name);
        unsafe { LLVMGetMDKindIDInContext(self.context, name.as_ptr(), length) }
    }
    
    pub fn create_enum_attribute(&self, kind_id: u32, value: u64) -> Attribute {
        let attr = unsafe { LLVMCreateEnumAttribute(self.context, kind_id, value) };
        Attribute::new(attr)
    }
    
    pub fn create_type_attribute(&self, kind_id: u32, ty: RawType) -> Attribute {
        let attr = unsafe { LLVMCreateTypeAttribute(self.context, kind_id, ty.as_llvm_ref()) };
        Attribute::new(attr)
    }
    
    pub fn create_string_attribute(&self, key: &str, value: &str) -> Attribute {
        let key_length = key.len() as u32;
        let value_length = value.len() as u32;
        let key = to_c_str(key);
        let value = to_c_str(value);
        let attr = unsafe {
            LLVMCreateStringAttribute(
                self.context,
                key.as_ptr(),
                key_length,
                value.as_ptr(),
                value_length
            )
        };
        Attribute::new(attr)
    }
    
    pub fn get_type_by_name(&self, name: &str) -> Option<RawType> {
        let name = to_c_str(name);
        let ty = unsafe { LLVMGetTypeByName2(self.context, name.as_ptr()) };
        if ty.is_null() {
            None
        } else {
            Some(RawType::new(ty))
        }
    }
    
    
}


impl Default for Context {
    fn default() -> Self {
        Self::create()
    }
}
