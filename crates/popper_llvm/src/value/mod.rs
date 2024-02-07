use llvm_sys::prelude::{
    LLVMTypeRef,
    LLVMValueRef,
};
use llvm_sys::core::{
    LLVMPrintValueToString,
    LLVMReplaceAllUsesWith,
    LLVMDumpValue,
    LLVMTypeOf,
    LLVMGetValueName2 as LLVMGetValueName,
    LLVMSetValueName2 as LLVMSetValueName,
};

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


use crate::types::{int_types, TypeEnum};
use crate::value::array_value::ArrayValue;

pub trait Value {
    fn get_type_ref(&self) -> LLVMTypeRef {
        unsafe { LLVMTypeOf(self.as_value_ref()) }
    }
    fn get_type(&self) -> TypeEnum {
        self.get_type_ref().into()
    }

    fn as_value_ref(&self) -> LLVMValueRef;
    fn is_null_or_undef(&self) -> bool;
    fn is_const(&self) -> bool;
    fn is_null(&self) -> bool;
    fn is_undef(&self) -> bool;
    fn print_to_string(&self) -> String {
        let llvm_str = unsafe { LLVMPrintValueToString(self.as_value_ref()) };
        let str_slice = unsafe { std::ffi::CStr::from_ptr(llvm_str) }.to_str().unwrap();
        let string = str_slice.to_owned();
        string
    }
    fn print_to_stderr(&self) {
        eprintln!("{}", self.print_to_string());
    }
    fn replace_all_uses_with(&self, other: &dyn Value) {
        unsafe { LLVMReplaceAllUsesWith(self.as_value_ref(), other.as_value_ref()) };
    }
    fn set_name(&self, name: &str) {
        let name = std::ffi::CString::new(name).unwrap();
        unsafe { LLVMSetValueName(self.as_value_ref(), name.as_ptr(), name.as_bytes().len()) }
    }
    fn get_name(&self) -> &str {
        let name = unsafe { LLVMGetValueName(self.as_value_ref(), &mut 0) };
        let name = unsafe { std::ffi::CStr::from_ptr(name) };
        let name = name.to_str().unwrap();
        name
    }
    fn dump(&self) {
        unsafe { LLVMDumpValue(self.as_value_ref()) }
    }
}

pub mod int_value;
pub mod float_value;
pub mod function_value;
pub mod array_value;
pub mod pointer_value;


#[derive(Debug, Copy, Clone)]
pub enum ValueEnum {
    IntValue(int_value::IntValue),
    FloatValue(float_value::FloatValue),
    FunctionValue(function_value::FunctionValue),
    ArrayValue(ArrayValue),
    PointerValue(pointer_value::PointerValue),
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
        }
    }

    pub fn print_to_string(&self) -> String {
        match self {
            ValueEnum::IntValue(int_value) => int_value.print_to_string(),
            ValueEnum::FloatValue(float_value) => float_value.print_to_string(),
            ValueEnum::FunctionValue(function_value) => function_value.print_to_string(),
            ValueEnum::ArrayValue(array_value) => array_value.print_to_string(),
            ValueEnum::PointerValue(pointer_value) => pointer_value.print_to_string(),
        }
    }

    pub fn as_llvm_ref(&self) -> LLVMValueRef {
        (*self).into()
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
        }
    }


}

impl Into<ValueEnum> for LLVMValueRef {
    fn into(self) -> ValueEnum {
        let value_type = unsafe { LLVMTypeOf(self) };
        let value_type_enum = value_type.into();
        match value_type_enum {
            TypeEnum::IntType(_) => ValueEnum::IntValue(int_value::IntValue::new_llvm_ref(self)),
            TypeEnum::FloatType(_) => ValueEnum::FloatValue(float_value::FloatValue::new_llvm_ref(self)),
            TypeEnum::FunctionType(_) => ValueEnum::FunctionValue(function_value::FunctionValue::new_llvm_ref(self)),
            TypeEnum::ArrayType(_) => ValueEnum::ArrayValue(ArrayValue::new_llvm_ref(self)),
            TypeEnum::PointerType(_) => ValueEnum::PointerValue(pointer_value::PointerValue::new_llvm_ref(self)),
            _ => panic!("Unknown type"),
        }
    }
}

impl Into<LLVMValueRef> for ValueEnum {
    fn into(self) -> LLVMValueRef {
        match self {
            ValueEnum::IntValue(int_value) => int_value.as_value_ref(),
            ValueEnum::FloatValue(float_value) => float_value.as_value_ref(),
            ValueEnum::FunctionValue(function_value) => function_value.as_value_ref(),
            ValueEnum::ArrayValue(array_value) => array_value.as_value_ref(),
            ValueEnum::PointerValue(pointer_value) => pointer_value.as_value_ref(),
        }
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
