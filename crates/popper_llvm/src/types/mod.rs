use llvm_sys::prelude::LLVMTypeRef;
use llvm_sys::core::LLVMPrintTypeToString;

mod metadata;
pub mod int_types;
pub mod float_types;
pub mod function_types;
pub mod array_types;

#[macro_export]
macro_rules! types {
    (fn($($t:tt),*) -> $r:tt) => {
        crate::types::function_types::FunctionType::new(
            vec![$(types!($t)),*],
            types!($r),
            false
        )
    };

    ([$t:tt; $s:expr]) => {
        crate::types::TypeEnum::ArrayType(crate::types::array_types::ArrayType::new(types!($t), $s))
    };
    (i1) => {
        crate::types::TypeEnum::IntType(crate::types::int_types::IntType::new_sized(1))
    };

    (i8) => {
        crate::types::TypeEnum::IntType(crate::types::int_types::IntType::new_sized(8))
    };

    (i16) => {
        crate::types::TypeEnum::IntType(crate::types::int_types::IntType::new_sized(16))
    };

    (i32) => {
        crate::types::TypeEnum::IntType(crate::types::int_types::IntType::new_sized(32))
    };

    (i64) => {
        crate::types::TypeEnum::IntType(crate::types::int_types::IntType::new_sized(64))
    };


}



pub trait Type {
    fn is_sized(&self) -> bool;
    fn get_type_ref(&self) -> LLVMTypeRef;

    fn print_to_string(&self) -> String {
        let llvm_str = unsafe { LLVMPrintTypeToString(self.get_type_ref()) };
        let str_slice = unsafe { std::ffi::CStr::from_ptr(llvm_str) }.to_str().unwrap();
        let string = str_slice.to_owned();
        string
    }
    fn print_to_stderr(&self) {
        eprintln!("{}", self.print_to_string());
    }
}

pub enum TypeEnum {
    IntType(int_types::IntType),
    FloatType(float_types::FloatType),
    FunctionType(function_types::FunctionType),
    ArrayType(array_types::ArrayType),
}

impl TypeEnum {
    pub fn get_type_ref(&self) -> LLVMTypeRef {
        match self {
            TypeEnum::IntType(t) => t.get_type_ref(),
            TypeEnum::FloatType(t) => t.get_type_ref(),
            TypeEnum::FunctionType(t) => t.get_type_ref(),
            TypeEnum::ArrayType(t) => t.get_type_ref(),
        }
    }

}


impl Into<TypeEnum> for LLVMTypeRef {
    fn into(self) -> TypeEnum {
        let type_ = unsafe { llvm_sys::core::LLVMGetTypeKind(self) };
        match type_ {
            llvm_sys::LLVMTypeKind::LLVMIntegerTypeKind => {
                TypeEnum::IntType(int_types::IntType::new_with_llvm_ref(self))
            }
            llvm_sys::LLVMTypeKind::LLVMDoubleTypeKind => {
                TypeEnum::FloatType(float_types::FloatType::new_with_llvm_ref(self))
            }
            llvm_sys::LLVMTypeKind::LLVMFunctionTypeKind => {
                TypeEnum::FunctionType(function_types::FunctionType::new_with_llvm_ref(self))
            }
            llvm_sys::LLVMTypeKind::LLVMArrayTypeKind => {
                TypeEnum::ArrayType(array_types::ArrayType::new_with_llvm_ref(self))
            }
            _ => panic!("Unknown type"),
        }
    }
}