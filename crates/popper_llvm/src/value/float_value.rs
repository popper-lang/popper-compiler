use crate::types;
use crate::types::TypeEnum;
use crate::value::{RawValue, Value, ValueEnum};
use llvm_sys::core::{LLVMConstReal, LLVMConstRealGetDouble, LLVMTypeOf};
use llvm_sys::prelude::{LLVMValueRef};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FloatValue {
    pub(crate) float_value: RawValue,
    pub(crate) float_type: types::float_types::FloatType,
}

impl FloatValue {
    /// # Safety
    /// This function is unsafe because it does not check if the LLVMValueRef is a valid float value.
    pub unsafe fn new_llvm_ref(lref: LLVMValueRef) -> Self {
        let float_type =
            types::float_types::FloatType::new_with_llvm_ref( LLVMTypeOf(lref));
        Self {
            float_value: RawValue::new(lref),
            float_type,
        }
    }
    pub fn new_const(value: f64, float_type: types::float_types::FloatType) -> Self {
        let float_value = unsafe { LLVMConstReal(float_type.float_type.as_llvm_ref(), value) };
        Self {
            float_value: RawValue::new(float_value),
            float_type,
        }
    }

    pub fn get_value(&self) -> f64 {
        let double = unsafe { LLVMConstRealGetDouble(self.float_value.as_llvm_ref(), &mut 0) };
        double as f64
    }

    pub fn to_value_enum(&self) -> ValueEnum {
        ValueEnum::FloatValue(*self)
    }
}

impl Value for FloatValue {
    
    fn get_type(&self) -> TypeEnum {
        TypeEnum::FloatType(self.float_type)
    }

    fn as_raw(&self) -> RawValue {
        self.float_value
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
