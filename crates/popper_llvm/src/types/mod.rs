use llvm_sys::core::{LLVMGetTypeKind, LLVMPrintTypeToString};
use llvm_sys::prelude::LLVMTypeRef;
use llvm_sys::LLVMTypeKind;

pub mod array_types;
pub mod float_types;
pub mod function_types;
pub mod int_types;
mod metadata;
mod pointer_types;

#[macro_export]
macro_rules! types {
    (fn($($t:tt),*) -> $r:tt) => {
        $crate::types::function_types::FunctionType::new(
            vec![$(types!($t)),*],
            types!($r),
            false
        )
    };

    ([$t:tt; $s:expr]) => {
        $crate::types::TypeEnum::ArrayType($crate::types::array_types::ArrayType::new(types!($t), $s))
    };
    (i1) => {
        $crate::types::TypeEnum::IntType($crate::types::int_types::IntType::new_sized(1))
    };

    (i8) => {
        $crate::types::TypeEnum::IntType($crate::types::int_types::IntType::new_sized(8))
    };

    (i16) => {
        $crate::types::TypeEnum::IntType($crate::types::int_types::IntType::new_sized(16))
    };

    (i32) => {
        $crate::types::TypeEnum::IntType($crate::types::int_types::IntType::new_sized(32))
    };

    (i64) => {
        $crate::types::TypeEnum::IntType($crate::types::int_types::IntType::new_sized(64))
    };


}

pub(crate) fn check_same_ty(tref: LLVMTypeRef, tname: &str) {
    let ty = unsafe { LLVMGetTypeKind(tref) };

    match (ty, tname) {
        (LLVMTypeKind::LLVMIntegerTypeKind, "int") => {}
        (LLVMTypeKind::LLVMFloatTypeKind, "float") => {}
        (LLVMTypeKind::LLVMDoubleTypeKind, "double") => {}
        (LLVMTypeKind::LLVMFunctionTypeKind, "function") => {}
        (LLVMTypeKind::LLVMArrayTypeKind, "array") => {}
        (LLVMTypeKind::LLVMPointerTypeKind, "pointer") => {}
        _ => panic!("Type mismatch: expected {} got {:?}", tname, ty),
    }

}

pub trait Type {
    fn is_sized(&self) -> bool;
    fn get_type_ref(&self) -> LLVMTypeRef;

    fn print_to_string(&self) -> String {
        let llvm_str = unsafe { LLVMPrintTypeToString(self.get_type_ref()) };
        let str_slice = unsafe { std::ffi::CStr::from_ptr(llvm_str) }
            .to_str()
            .unwrap();
        str_slice.to_owned()
    }
    fn print_to_stderr(&self) {
        eprintln!("{}", self.print_to_string());
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TypeEnum {
    IntType(int_types::IntType),
    FloatType(float_types::FloatType),
    FunctionType(function_types::FunctionType),
    ArrayType(array_types::ArrayType),
    PointerType(pointer_types::PointerTypes),
}

impl TypeEnum {
    pub fn get_type_ref(&self) -> LLVMTypeRef {
        match self {
            TypeEnum::IntType(t) => t.get_type_ref(),
            TypeEnum::FloatType(t) => t.get_type_ref(),
            TypeEnum::FunctionType(t) => t.get_type_ref(),
            TypeEnum::ArrayType(t) => t.get_type_ref(),
            TypeEnum::PointerType(t) => t.get_type_ref(),
        }
    }

    pub fn func(&self, args: Vec<TypeEnum>, is_var_args: bool) -> function_types::FunctionType {
        match self {
            TypeEnum::IntType(t) => t.func(args, is_var_args),
            TypeEnum::FloatType(t) => t.func(args, is_var_args),
            TypeEnum::ArrayType(t) => t.func(args, is_var_args),
            TypeEnum::FunctionType(t) => t.func(args, is_var_args),
            TypeEnum::PointerType(t) => t.func(args, is_var_args),
        }
    }

    pub fn ptr(&self) -> pointer_types::PointerTypes {
        pointer_types::PointerTypes::new_const(*self)
    }
}


impl From<LLVMTypeRef> for TypeEnum {
    fn from(value: LLVMTypeRef) -> Self {
        let type_ = unsafe { llvm_sys::core::LLVMGetTypeKind(value) };
        match type_ {
            llvm_sys::LLVMTypeKind::LLVMIntegerTypeKind => {
                TypeEnum::IntType(unsafe { int_types::IntType::new_llvm_ref(value) })
            }
            llvm_sys::LLVMTypeKind::LLVMDoubleTypeKind => {
                TypeEnum::FloatType(float_types::FloatType::new_with_llvm_ref(value))
            }
            llvm_sys::LLVMTypeKind::LLVMFunctionTypeKind => {
                TypeEnum::FunctionType(function_types::FunctionType::new_with_llvm_ref(value))
            }
            llvm_sys::LLVMTypeKind::LLVMArrayTypeKind => {
                TypeEnum::ArrayType(unsafe { array_types::ArrayType::new_with_llvm_ref(value) })
            }
            llvm_sys::LLVMTypeKind::LLVMPointerTypeKind => {
                TypeEnum::PointerType(pointer_types::PointerTypes::new_llvm_ref(value))
            }
            _ => panic!("Unknown type"),
        }

    }
}
