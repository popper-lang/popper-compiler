use llvm_sys::core::{
    LLVMArrayType2 as LLVMArrayType, LLVMConstString, LLVMGetArrayLength2 as LLVMGetArrayLength,
    LLVMGetElementType,
};
use llvm_sys::prelude::LLVMTypeRef;
use std::fmt::Debug;

use crate::types::function_types::FunctionType;
use crate::types::{Type, TypeEnum};
use crate::value::array_value::ArrayValue;
use crate::value::ValueEnum;

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

    pub unsafe fn new_with_llvm_ref(array_type: LLVMTypeRef) -> Self {
        Self {
            array_type,
            size: unsafe { LLVMGetArrayLength(array_type) },
        }
    }

    pub fn const_array(&self, values: &[ValueEnum]) -> ArrayValue {
        ArrayValue::new_const(values, *self)
    }

    pub fn const_string(&self, string: &str) -> ArrayValue {
        let values =
            unsafe { LLVMConstString(string.as_ptr() as *const i8, string.len() as u32, 0) };
        ArrayValue::new_llvm_ref(values)
    }

    pub fn func(&self, args: Vec<TypeEnum>, is_var_args: bool) -> FunctionType {
        FunctionType::new(args, self.to_type_enum(), is_var_args)
    }

    pub fn to_type_enum(&self) -> TypeEnum {
        TypeEnum::ArrayType(*self)
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
