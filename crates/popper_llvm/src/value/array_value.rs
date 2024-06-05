use crate::types;
use crate::value::{Value, ValueEnum};
use llvm_sys::core::{LLVMConstArray2, LLVMTypeOf};
use llvm_sys::prelude::{LLVMValueRef};


use super::RawValue;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ArrayValue {
    pub(crate) array_value: RawValue,
    pub(crate) array_type: types::array_types::ArrayType,
}

impl ArrayValue {
    pub fn new_const(value: &[ValueEnum], array_type: types::array_types::ArrayType) -> Self {
        let mut value = value.iter().map(|v| v.as_raw().as_llvm_ref()).collect::<Vec<_>>();
        let array_value = unsafe {
            LLVMConstArray2(
                array_type.array_type.as_llvm_ref(),
                value.as_mut_ptr(),
                value.len() as u64,
            )
        };
        Self {
            array_value: RawValue::new(array_value),
            array_type,
        }
    }

    /// # Safety
    /// This function is unsafe because it does not check if the LLVMValueRef is a valid array value.
    pub unsafe fn new_llvm_ref(lref: LLVMValueRef) -> Self {
        let array_type =
            types::array_types::ArrayType::new_with_llvm_ref(unsafe { LLVMTypeOf(lref) });
        Self {
            array_value: RawValue::new(lref),
            array_type,
        }
    }

    pub fn get_value(&self) -> RawValue {
        self.array_value
    }
    pub fn to_value_enum(&self) -> ValueEnum {
        ValueEnum::ArrayValue(*self)
    }
}

impl Value for ArrayValue {
    fn get_type(&self) -> types::TypeEnum {
        types::TypeEnum::ArrayType(self.array_type)
    }

    fn as_raw(&self) -> RawValue {
        self.array_value
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
