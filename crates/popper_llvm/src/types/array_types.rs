use llvm_sys::core::{
    LLVMArrayType2 as LLVMArrayType, LLVMConstString, LLVMGetArrayLength2 as LLVMGetArrayLength,
    LLVMGetElementType,
};
use llvm_sys::prelude::LLVMTypeRef;
use std::fmt::Debug;
use popper_mem::string::to_c_str;

use crate::types::function_types::FunctionType;
use crate::types::{check_same_ty, RawType, Type, TypeEnum};
use crate::types::int_types::IntType;
use crate::value::array_value::ArrayValue;
use crate::value::ValueEnum;

#[derive(Clone, Copy, PartialEq)]
pub struct ArrayType {
    pub(crate) array_type: RawType,
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

    fn expect_i8(&self) {
        assert_eq!(self.get_element_type(), IntType::new_sized(8).to_type_enum());
    }
    pub fn new(element_type: TypeEnum, size: u64) -> Self {
        check_same_ty(element_type.as_raw().as_llvm_ref(), "array");
        let array_type = unsafe { LLVMArrayType(element_type.as_raw().as_llvm_ref(), size) };
        Self { array_type: RawType::new(array_type), size }
    }

    pub unsafe fn new_with_llvm_ref(array_type: LLVMTypeRef) -> Self {
        Self {
            array_type: RawType::new(array_type),
            size: unsafe { LLVMGetArrayLength(array_type) },
        }
    }

    pub fn const_array(&self, values: &[ValueEnum]) -> ArrayValue {
        ArrayValue::new_const(values, *self)
    }

    pub fn const_string(&self, string: &str) -> ArrayValue {
        self.expect_i8();
        let length = string.len();
        let string = to_c_str(string);
        unsafe {
            let values = LLVMConstString(string.as_ptr(), length as u32, 0);
            ArrayValue::new_llvm_ref(values)
        }
    }

    pub fn get_size(&self) -> u64 {
        self.size
    }

    pub fn get_element_type(&self) -> TypeEnum {
        let element_type = unsafe { LLVMGetElementType(self.array_type.as_llvm_ref()) };
        element_type.into()
    }
}

impl Type for ArrayType {
    fn is_sized(&self) -> bool {
        true
    }
    fn as_raw(&self) -> RawType {
        self.array_type
    }

    fn to_type_enum(&self) -> TypeEnum {
        TypeEnum::ArrayType(*self)
    }
}
