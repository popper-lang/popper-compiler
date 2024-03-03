use crate::types;
use crate::value::{Value, ValueEnum};
use llvm_sys::core::{LLVMConstArray2, LLVMTypeOf};
use llvm_sys::prelude::{LLVMTypeRef, LLVMValueRef};

#[derive(Debug, Copy, Clone)]
pub struct ArrayValue {
    pub(crate) array_value: LLVMValueRef,
    pub(crate) array_type: types::array_types::ArrayType,
}

impl ArrayValue {
    pub fn new_const(value: &[ValueEnum], array_type: types::array_types::ArrayType) -> Self {
        let array_value = unsafe {
            LLVMConstArray2(
                array_type.array_type,
                value.as_ptr() as *mut LLVMValueRef,
                value.len() as u64,
            )
        };
        Self {
            array_value,
            array_type,
        }
    }

    pub unsafe fn new_llvm_ref(lref: LLVMValueRef) -> Self {
        let array_type =
            types::array_types::ArrayType::new_with_llvm_ref(unsafe { LLVMTypeOf(lref) });
        Self {
            array_value: lref,
            array_type,
        }
    }

    pub fn get_value(&self) -> LLVMValueRef {
        self.array_value
    }
    pub fn to_value_enum(&self) -> ValueEnum {
        ValueEnum::ArrayValue(*self)
    }
}

impl Value for ArrayValue {
    fn get_type_ref(&self) -> LLVMTypeRef {
        todo!()
    }

    fn get_type(&self) -> types::TypeEnum {
        types::TypeEnum::ArrayType(self.array_type)
    }

    fn as_value_ref(&self) -> LLVMValueRef {
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
