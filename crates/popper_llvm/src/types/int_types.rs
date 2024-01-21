use llvm_sys::prelude::LLVMTypeRef;
use llvm_sys::core::{
    LLVMIntTypeInContext,
    LLVMGetIntTypeWidth,
    LLVMIntType,
};
use crate::context::Context;
use crate::types::{Type, TypeEnum};
use crate::types::array_types::ArrayType;
use crate::types::function_types::FunctionType;
use crate::value::int_value::IntValue;

macro_rules! impl_into_int_type {
    ($type:ty) => {
        impl Into<IntType> for $type {
            fn into(self) -> IntType {
                IntType::new_sized(std::mem::size_of::<Self>() as u32)
            }
        }
    };
}

#[derive(Clone, Copy, Debug)]
pub struct IntType {
    pub(crate) int_type: LLVMTypeRef,
    pub(crate) size: u32,
}

impl IntType {

    pub fn new_with_llvm_ref(llvm_ty: LLVMTypeRef) -> Self {
        let size = unsafe { LLVMGetIntTypeWidth(llvm_ty) };
        Self { int_type: llvm_ty, size }
    }

    pub fn new_sized(size: u32) -> Self {
        let int_type = unsafe { LLVMIntType(size) };
        Self { int_type, size }
    }


    pub fn new_with_context(size: u32, context: Context) -> Self {
        let int_type = unsafe { LLVMIntTypeInContext(context.context, size) };
        Self { int_type, size }
    }

    pub fn get_size(&self) -> u32 {
        self.size
    }

    pub fn int(&self, value: u32, sign_extend: bool) -> IntValue {
        IntValue::new_const(value, self.clone(), sign_extend)
    }

    pub fn array(&self, length: u64) -> ArrayType {
        ArrayType::new(self.to_type_enum(), length)
    }
    pub fn func(&self, args: Vec<TypeEnum>, is_var_args: bool) -> FunctionType {
        FunctionType::new(args, self.to_type_enum(), is_var_args)
    }

    pub fn to_type_enum(self) -> TypeEnum {
        TypeEnum::IntType(self)
    }

}

impl_into_int_type!(u8);
impl_into_int_type!(u16);
impl_into_int_type!(u32);
impl_into_int_type!(u64);




impl Type for IntType {
    fn is_sized(&self) -> bool {
        true
    }

    fn get_type_ref(&self) -> LLVMTypeRef {
        self.int_type
    }

}