use crate::context::Context;
use crate::types::array_types::ArrayType;
use crate::types::function_types::FunctionType;
use crate::types::{Type, TypeEnum};
use crate::value::int_value::IntValue;
use llvm_sys::core::{LLVMGetIntTypeWidth, LLVMIntType, LLVMIntTypeInContext};
use llvm_sys::prelude::LLVMTypeRef;

use super::check_same_ty;

macro_rules! impl_into_int_type {
    ($type:ty) => {
        impl From<$type> for IntType {
            fn from(_: $type) -> IntType {
                IntType::new_sized(std::mem::size_of::<$type>() as u32)

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

    fn check_size(size: u32) {
        if size == 0 {
            panic!("Size of int type cannot be 0");
        }

        if size > 8 * std::mem::size_of::<usize>() as u32 {
            panic!("Size of int type cannot be greater than {} bits", 8 * std::mem::size_of::<usize>());
        }
    }

    /// Creates a new `IntType` with the given `LLVMTypeRef`.
    /// # Panics
    /// Panics if the given `LLVMTypeRef` is not an `int` type.
    pub(crate) fn new_llvm_ref(llvm_ty: LLVMTypeRef) -> Self {
        check_same_ty(llvm_ty, "int");
        let size = unsafe { LLVMGetIntTypeWidth(llvm_ty) };
        Self {
            int_type: llvm_ty,
            size,
        }
    }

    /// Creates a new `IntType` with the given size.
    /// # Panics
    /// Panics if the given size is not a valid size for an `int` type.
    pub fn new_sized(size: u32) -> Self {
        Self::check_size(size);
        let int_type = unsafe { LLVMIntType(size) };
        Self { int_type, size }
    }

    /// Creates a new `IntType` with the given size and `Context`.
    /// # Panics
    /// Panics if the given size is not a valid size for an `int` type.
    pub fn new_with_context(size: u32, context: Context) -> Self {
        Self::check_size(size);
        let int_type = unsafe { LLVMIntTypeInContext(context.context, size) };
        Self { int_type, size }
    }

    pub fn get_size(&self) -> u32 {
        self.size
    }

    pub fn int(&self, value: u32, sign_extend: bool) -> IntValue {
        IntValue::new_const(value, *self, sign_extend)
    }

    pub fn array(&self, length: u64) -> ArrayType {
        ArrayType::new(self.to_type_enum(), length)
    }
    pub fn func(&self, args: Vec<TypeEnum>, is_var_args: bool) -> FunctionType {
        FunctionType::new(args, self.to_type_enum(), is_var_args)
    }

    pub fn bool(&self, value: bool) -> IntValue {
        IntValue::new_const(value as u32, *self, false)
    }
    pub fn void(&self) -> IntValue {
        IntValue::new_const(0, *self, false)
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
