use llvm_sys::core::{LLVMDumpValue, LLVMGetValueName2 as LLVMGetValueName, LLVMPrintValueToString, LLVMReplaceAllUsesWith, LLVMSetValueName2 as LLVMSetValueName, LLVMTypeOf, LLVMInstructionEraseFromParent, LLVMValueAsBasicBlock};
use llvm_sys::prelude::{LLVMTypeRef, LLVMValueRef};

trait UnsignedInt: Sized + Into<u32> {}

impl UnsignedInt for u8 {}
impl UnsignedInt for u16 {}
impl UnsignedInt for u32 {}

macro_rules! values {
    (int($ty:tt) $e:expr) => {
        crate::value::int_value::IntValue::new_const($e, types!($ty), false)
    };
    (int($ty:tt) $e:expr, sign_extend) => {
        crate::value::int_value::IntValue::new_const($e, types!($ty), true)
    };
    (int_ref $e:expr) => {
        crate::value::int_value::IntValue::new_llvm_ref($e)
    };

    (float($ty:tt) $e:expr) => {
        crate::value::float_value::FloatValue::new_const($e, types!($ty))
    };

    (float_ref $e:expr) => {
        crate::value::float_value::FloatValue::new_llvm_ref($e)
    };
}

use crate::types::{int_types, RawType, TypeEnum};
use crate::util::to_c_str;
use crate::value::array_value::ArrayValue;

pub trait Value {
    fn get_type(&self) -> TypeEnum {
        unsafe {
            self
                .as_raw()
                .get_type()
                .as_type_enum()
        }
    }

    fn as_raw(&self) -> RawValue;
    fn is_null_or_undef(&self) -> bool;
    fn is_const(&self) -> bool;
    fn is_null(&self) -> bool;
    fn is_undef(&self) -> bool;
    fn set_name(&self, name: &str) {
        let name = std::ffi::CString::new(name).unwrap();
        unsafe { LLVMSetValueName(self.as_raw().as_llvm_ref(), name.as_ptr(), name.as_bytes().len()) }
    }
    fn get_name(&self) -> Option<String> {
        self.as_raw().get_named_value()
    }
    fn dump(&self) {
        unsafe { LLVMDumpValue(self.as_raw().as_llvm_ref()) }
    }
}

pub mod array_value;
pub mod float_value;
pub mod function_value;
pub mod int_value;
pub mod pointer_value;
pub mod struct_value;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ValueEnum {
    IntValue(int_value::IntValue),
    FloatValue(float_value::FloatValue),
    FunctionValue(function_value::FunctionValue),
    ArrayValue(ArrayValue),
    PointerValue(pointer_value::PointerValue),
    StructValue(struct_value::StructValue),
}

impl ValueEnum {
    pub fn pointer(&self, type_enum: TypeEnum) -> pointer_value::PointerValue {
        pointer_value::PointerValue::new_constant(*self, type_enum)
    }
    pub fn get_type(&self) -> TypeEnum {
        match self {
            ValueEnum::IntValue(int_value) => int_value.get_type(),
            ValueEnum::FloatValue(float_value) => float_value.get_type(),
            ValueEnum::FunctionValue(function_value) => function_value.get_type(),
            ValueEnum::ArrayValue(array_value) => array_value.get_type(),
            ValueEnum::PointerValue(pointer_value) => pointer_value.get_type(),
            ValueEnum::StructValue(struct_value) => struct_value.get_type(),
        }
    }


    pub fn print_to_string(&self) -> String {
        self.as_raw().print_to_string()
    }

    pub fn as_llvm_ref(&self) -> LLVMValueRef {
        self.as_raw().as_llvm_ref()
    }

    pub fn into_int_value(self) -> int_value::IntValue {
        match self {
            ValueEnum::IntValue(int_value) => int_value,
            _ => panic!("Not an int value"),
        }
    }

    pub fn into_ptr_value(self) -> pointer_value::PointerValue {
        match self {
            ValueEnum::PointerValue(ptr_value) => ptr_value,
            _ => panic!("Not a pointer value"),
        }
    }

    pub fn dump(&self) {
        match self {
            ValueEnum::IntValue(int_value) => int_value.dump(),
            ValueEnum::FloatValue(float_value) => float_value.dump(),
            ValueEnum::FunctionValue(function_value) => function_value.dump(),
            ValueEnum::ArrayValue(array_value) => array_value.dump(),
            ValueEnum::PointerValue(pointer_value) => pointer_value.dump(),
            ValueEnum::StructValue(struct_value) => struct_value.dump(),
        }
    }

    pub fn erase_from_parent(&self) {
        unsafe { LLVMInstructionEraseFromParent(self.as_llvm_ref()) }
    }

    pub fn as_raw(&self) -> RawValue {
        match self {
            ValueEnum::IntValue(int_value) => int_value.as_raw(),
            ValueEnum::FloatValue(float_value) => float_value.as_raw(),
            ValueEnum::FunctionValue(function_value) => function_value.as_raw(),
            ValueEnum::ArrayValue(array_value) => array_value.as_raw(),
            ValueEnum::PointerValue(pointer_value) => pointer_value.as_raw(),
            ValueEnum::StructValue(struct_value) => struct_value.as_raw(),

        }
    }
}

impl From<LLVMValueRef> for ValueEnum {
    fn from(value: LLVMValueRef) -> Self {
        unsafe {
            let value_type = LLVMTypeOf(value);
            let value_type_enum = value_type.into();
            match value_type_enum {
                TypeEnum::IntType(_) => ValueEnum::IntValue(
                    int_value::IntValue::new_llvm_ref(value)
                ),
                TypeEnum::FloatType(_) => {
                    ValueEnum::FloatValue(float_value::FloatValue::new_llvm_ref(value))
                }
                TypeEnum::FunctionType(_) => {
                    ValueEnum::FunctionValue(function_value::FunctionValue::new_llvm_ref(value, None))
                }
                TypeEnum::ArrayType(_) => ValueEnum::ArrayValue(ArrayValue::new_llvm_ref(value)),
                TypeEnum::PointerType(_) => {
                    ValueEnum::PointerValue(pointer_value::PointerValue::new_llvm_ref(value))
                },
                TypeEnum::StructType(_) => {
                    ValueEnum::StructValue(struct_value::StructValue::new_llvm_ref(value))
                }
                _ => panic!("Unknown type"),
            }

        }
    }
}



impl From<ValueEnum> for LLVMValueRef {
    fn from(value: ValueEnum) -> LLVMValueRef {
        value.as_raw().as_llvm_ref()
    }
}

pub trait ToValue {
    fn to_value(self) -> ValueEnum;
}

impl ToValue for ValueEnum {
    fn to_value(self) -> ValueEnum {
        self
    }
}

impl ToValue for int_value::IntValue {
    fn to_value(self) -> ValueEnum {
        ValueEnum::IntValue(self)
    }
}

impl ToValue for float_value::FloatValue {
    fn to_value(self) -> ValueEnum {
        ValueEnum::FloatValue(self)
    }
}

impl ToValue for function_value::FunctionValue {
    fn to_value(self) -> ValueEnum {
        ValueEnum::FunctionValue(self)
    }
}

impl ToValue for LLVMValueRef {
    fn to_value(self) -> ValueEnum {
        self.into()
    }
}

impl<T: UnsignedInt> ToValue for T {
    fn to_value(self) -> ValueEnum {
        let ty = int_types::IntType::new_sized(std::mem::size_of::<T>() as u32);
        ty.int(self.into(), false).to_value()
    }
}

impl ToValue for i8 {
    fn to_value(self) -> ValueEnum {
        let ty = int_types::IntType::new_sized(std::mem::size_of::<i8>() as u32);
        ty.int(self as u32, true).to_value()
    }
}

impl ToValue for i16 {
    fn to_value(self) -> ValueEnum {
        let ty = int_types::IntType::new_sized(std::mem::size_of::<i16>() as u32);
        ty.int(self as u32, true).to_value()
    }
}

impl ToValue for i32 {
    fn to_value(self) -> ValueEnum {
        let ty = int_types::IntType::new_sized(std::mem::size_of::<i32>() as u32);
        ty.int(self as u32, true).to_value()
    }
}

impl ToValue for i64 {
    fn to_value(self) -> ValueEnum {
        let ty = int_types::IntType::new_sized(std::mem::size_of::<i64>() as u32);
        ty.int(self as u32, true).to_value()
    }
}

pub fn to_value<T: ToValue>(t: T) -> ValueEnum {
    t.to_value()
}


pub(crate) trait MathValue: Value {}


impl MathValue for int_value::IntValue {}
impl MathValue for float_value::FloatValue {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RawValue {
    raw: LLVMValueRef
}

impl RawValue {
    pub fn new(raw: LLVMValueRef) -> Self {
        RawValue { raw }
    }

    pub fn as_llvm_ref(&self) -> LLVMValueRef {
        self.raw
    }

    pub fn dump(&self) {
        unsafe { LLVMDumpValue(self.raw) }
    }


    pub fn erase_from_parent(&self) {
        unsafe { LLVMInstructionEraseFromParent(self.raw) }
    }

    pub fn print_to_string(&self) -> String {
        unsafe {
            let llvm_str = LLVMPrintValueToString(self.raw);
            let str_slice = std::ffi::CStr::from_ptr(llvm_str)
                .to_str()
                .unwrap();
            str_slice.to_owned()
        }
    }

    pub fn replace_all_uses_with(&self, new_value: RawValue) {
        unsafe { LLVMReplaceAllUsesWith(self.raw, new_value.raw) }
    }

    /// # Safety
    /// ValueEnum::from use LLVMTypeOf which can be invalid if the value is a function
    unsafe fn into_value_enum(self) -> ValueEnum {
        ValueEnum::from(self.raw)
    }

    pub fn get_type(&self) -> RawType {
        unsafe {
            let value_type = LLVMTypeOf(self.raw);
            RawType::new(value_type)
        }
    }


    pub fn get_named_value(&self) -> Option<String> {
        unsafe {
            let mut name_length = 0;
            let name = LLVMGetValueName(self.raw, &mut name_length);
            if name.is_null() {
                None
            } else {
                Some(std::ffi::CStr::from_ptr(name).to_str().unwrap().to_owned())
            }
        }
    }

    pub fn set_name(&self, name: &str) {
        let length = name.len();
        let c_str = to_c_str(name);
        unsafe {
            LLVMSetValueName(self.raw, c_str.as_ptr(), length);
        }
    }
    
    pub fn try_as_basic_block(&self) -> Option<crate::basic_block::BasicBlock> {
        unsafe {
            let value = LLVMValueAsBasicBlock(self.raw);
            if value.is_null() {
                None
            } else {
                Some(crate::basic_block::BasicBlock::new(value))
            }
        }
    }
    
    
    /// # Safety
    /// This function is unsafe because the value may not be a valid debug location
    pub unsafe fn as_debug_loc(&self) -> crate::debug::DebugLoc {
        crate::debug::DebugLoc::new(*self)
    }
    
    /// # Safety
    /// This function is unsafe because the value may not be a valid inline asm
    pub unsafe fn as_inline_asm(&self) -> crate::asm::InlineAsm {
        crate::asm::InlineAsm::new(self.as_llvm_ref())
    }


}
