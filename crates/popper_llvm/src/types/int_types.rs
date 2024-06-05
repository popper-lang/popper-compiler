use crate::context::Context;
use crate::types::{Type, TypeEnum};
use crate::value::int_value::IntValue;
use llvm_sys::core::{LLVMGetIntTypeWidth, LLVMIntType, LLVMIntTypeInContext};
use llvm_sys::prelude::LLVMTypeRef;

use super::{check_same_ty, RawType};

macro_rules! impl_into_int_type {
    ($type:ty) => {
        impl From<$type> for IntType {
            fn from(_: $type) -> IntType {
                IntType::new_sized(std::mem::size_of::<$type>() as u32)

            }
        }
    };
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IntType {
    pub(crate) int_type: RawType,
    pub(crate) size: u32,
}

impl IntType {

    fn check_size(size: u32) {
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
            int_type: RawType::new(llvm_ty),
            size,
        }
    }

    /// Creates a new `IntType` with the given size.
    /// # Panics
    /// Panics if the given size is not a valid size for an `int` type.
    pub fn new_sized(size: u32) -> Self {
        Self::check_size(size);
        let int_type = unsafe { LLVMIntType(size) };
        Self { int_type: RawType::new(int_type), size }
    }

    /// Creates a new `IntType` with the given size and `Context`.
    /// # Panics
    /// Panics if the given size is not a valid size for an `int` type.
    pub fn new_with_context(size: u32, context: Context) -> Self {
        Self::check_size(size);
        let int_type = unsafe { LLVMIntTypeInContext(context.context, size) };
        Self { int_type: RawType::new(int_type), size }
    }

    pub fn get_size(&self) -> u32 {
        self.size
    }
    
    pub fn get_int_width(&self) -> u32 {
        unsafe { LLVMGetIntTypeWidth(self.int_type.as_llvm_ref()) }
    }

    pub fn int(&self, value: u32, sign_extend: bool) -> IntValue {
        IntValue::new_const(value, *self, sign_extend)
    }

    pub fn bool(&self, value: bool) -> IntValue {
        IntValue::new_const(value as u32, *self, false)
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

    fn as_raw(&self) -> RawType {
        self.int_type
    }

    fn to_type_enum(&self) -> TypeEnum {
        TypeEnum::IntType(*self)
    }
}
