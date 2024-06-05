use crate::types::{RawType, Type, TypeEnum};
use llvm_sys::core::*;
use llvm_sys::prelude::*;
use crate::context::Context;
use crate::util::{ptr_to_option, to_c_str};
use crate::value::ValueEnum;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct StructType {
    pub(crate) struct_type: RawType,
}

impl StructType {
    pub fn new(element_types: &[TypeEnum], is_packed: bool) -> Self {
        let mut element_types: Vec<LLVMTypeRef> = element_types.iter().map(|ty| ty.as_raw().raw).collect();
        let struct_type = unsafe {
            LLVMStructType(
                element_types.as_mut_ptr(),
                element_types.len() as u32,
                is_packed.into(),
            )
        };
        Self { struct_type: RawType::new(struct_type) }
    }

    pub fn new_named(context: Context, name: &str) -> Self {
        let name = to_c_str(name);
        let struct_type = unsafe { LLVMStructCreateNamed(context.context, name.as_ptr()) };
        Self { struct_type: RawType::new(struct_type) }
    }

    pub fn new_with_llvm_ref(struct_type: LLVMTypeRef) -> Self {
        Self { struct_type: RawType::new(struct_type) }
    }

    pub fn set_body(&self, element_types: &[TypeEnum], is_packed: bool) {
        let mut element_types: Vec<LLVMTypeRef> = element_types.iter().map(|ty| ty.as_raw().raw).collect();
        unsafe {
            LLVMStructSetBody(
                self.struct_type.as_llvm_ref(),
                element_types.as_mut_ptr(),
                element_types.len() as u32,
                is_packed.into(),
            )
        }
    }

    pub fn get_element_types(&self) -> Vec<TypeEnum> {
        let length = unsafe { LLVMCountStructElementTypes(self.struct_type.as_llvm_ref()) };
        let slice: *mut LLVMTypeRef = std::ptr::null_mut();
        unsafe { LLVMGetStructElementTypes(self.struct_type.as_llvm_ref(), slice) };
        let slice = unsafe { std::slice::from_raw_parts(slice, length as usize) };
        slice.iter().map(|&ty| ty.into()).collect()
    }
    
    pub fn nth_element_type(&self, index: u32) -> Option<TypeEnum> {
        unsafe { ptr_to_option(LLVMStructGetTypeAtIndex(self.struct_type.as_llvm_ref(), index)) }
            .map(|x| TypeEnum::from(x))
    }

    pub fn get_name(&self) -> Option<String> {
        let name = ptr_to_option(unsafe { LLVMGetStructName(self.struct_type.as_llvm_ref()) })?;
        unsafe {
            Some(std::ffi::CStr::from_ptr(name)
                .to_str()
                .unwrap()
                .to_string())
        }
    }

    pub fn is_packed(&self) -> bool {
        unsafe { LLVMIsPackedStruct(self.struct_type.as_llvm_ref()) == 1 }
    }
    
    pub fn is_opaque(&self) -> bool {
        unsafe { LLVMIsOpaqueStruct(self.struct_type.as_llvm_ref()) == 1 }
    }
    
    pub fn is_literal(&self) -> bool {
        unsafe { LLVMIsLiteralStruct(self.struct_type.as_llvm_ref()) == 1 }
    }
    
    pub fn const_struct(&self, constant_values: &[ValueEnum]) -> ValueEnum {
        if self.get_name().is_some() {
            let mut constant_values: Vec<LLVMValueRef> = constant_values.iter().map(|val| val.as_raw().as_llvm_ref()).collect();
            let value = unsafe { LLVMConstNamedStruct(self.struct_type.as_llvm_ref(), constant_values.as_mut_ptr(), constant_values.len() as u32) };
            value.into()
        } else {
            let mut constant_values: Vec<LLVMValueRef> = constant_values.iter().map(|val| val.as_raw().as_llvm_ref()).collect();
            let value = unsafe { LLVMConstStruct(constant_values.as_mut_ptr(), constant_values.len() as u32, 0) };
            value.into()
        
        }
    }
}

impl Type for StructType {
    fn is_sized(&self) -> bool {
        true
    }

    fn as_raw(&self) -> RawType {
        self.struct_type
    }

    fn to_type_enum(&self) -> TypeEnum {
        TypeEnum::StructType(*self)
    }
}