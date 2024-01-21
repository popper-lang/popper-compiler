use llvm_sys::prelude::{
    LLVMTypeRef,
    LLVMValueRef,
};
use llvm_sys::core::{
    LLVMConstInt,
    LLVMConstIntGetZExtValue,
    LLVMTypeOf
};
use crate::types::{int_types, TypeEnum};
use crate::value::{Value, ValueEnum};

#[derive(Debug)]
pub struct IntValue {
    pub(crate) int_value: LLVMValueRef,
    pub(crate) int_type: int_types::IntType,
}

impl IntValue {

    pub fn new_llvm_ref(lref: LLVMValueRef) -> Self {
        let int_type = int_types::IntType::new_with_llvm_ref(unsafe { LLVMTypeOf(lref) });
        Self { int_value: lref, int_type }
    }
    pub fn new_const(value: u32, int_type: int_types::IntType, sign_extend: bool) -> Self {
        let int_value = unsafe { LLVMConstInt(int_type.int_type, value as u64, sign_extend.into()) };
        Self { int_value, int_type }
    }

    pub fn get_int_type(&self) -> int_types::IntType {
        self.int_type.clone()
    }

    pub fn get_value(&self) -> u32 {
        unsafe { LLVMConstIntGetZExtValue(self.int_value) as u32 }
    }
    
    pub fn to_value_enum(self) -> ValueEnum {
        ValueEnum::IntValue(self)
    }
}


impl Value for IntValue {
    fn get_type_ref(&self) -> LLVMTypeRef {
        self.int_type.int_type
    }

    fn get_type(&self) -> TypeEnum {
        TypeEnum::IntType(self.int_type)
    }

    fn as_value_ref(&self) -> LLVMValueRef {
        self.int_value
    }

    fn is_null_or_undef(&self) -> bool {
        false
    }

    fn is_const(&self) -> bool {
        true
    }

    fn is_null(&self) -> bool {
        false
    }

    fn is_undef(&self) -> bool {
        false
    }
}