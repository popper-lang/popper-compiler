use crate::context::Context;
use crate::types::function_types::FunctionType;
use crate::types::{Type, TypeEnum};
use crate::value::float_value::FloatValue;
use llvm_sys::core::LLVMDoubleTypeInContext;
use llvm_sys::prelude::LLVMTypeRef;
use crate::types::array_types::ArrayType;

use super::RawType;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FloatType {
    pub(crate) float_type: RawType,
}

impl FloatType {
    pub fn new_with_llvm_ref(float_type: LLVMTypeRef) -> Self {
        Self { float_type: RawType::new(float_type) }
    }
    pub fn new_with_context(context: Context) -> Self {
        let float_type = unsafe { LLVMDoubleTypeInContext(context.context) };
        Self { float_type: RawType::new(float_type) }
    }

    pub fn float(&self, value: f64) -> FloatValue {
        FloatValue::new_const(value, *self)
    }


}

impl Type for FloatType {
    fn is_sized(&self) -> bool {
        true
    }

    fn as_raw(&self) -> RawType {
        self.float_type
    }
    fn to_type_enum(&self) -> TypeEnum {
        TypeEnum::FloatType(*self)
    }
}
