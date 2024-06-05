use llvm_sys::core::*;
use llvm_sys::prelude::*;
use llvm_sys::LLVMTypeKind;
use crate::context::Context;
use crate::util::ptr_to_option;

pub mod array_types;
pub mod float_types;
pub mod function_types;
pub mod int_types;
pub mod pointer_types;
pub mod void_type;
pub mod struct_type;
pub mod vectore_type;

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

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TypeKind {
    Void,
    Half,
    Float,
    Double,
    X86_FP80,
    FP128,
    PPC_FP128,
    Label,
    Integer,
    Function,
    Struct,
    Array,
    Pointer,
    Vector,
    Metadata,
    X86_MMX,
    Token,
    ScalableVector,
    BFloat,
    X86_AMX,
    TargetExt,
}


impl From<LLVMTypeKind> for TypeKind {
    fn from(kind: LLVMTypeKind) -> Self {
        match kind {
            LLVMTypeKind::LLVMVoidTypeKind => TypeKind::Void,
            LLVMTypeKind::LLVMHalfTypeKind => TypeKind::Half,
            LLVMTypeKind::LLVMFloatTypeKind => TypeKind::Float,
            LLVMTypeKind::LLVMDoubleTypeKind => TypeKind::Double,
            LLVMTypeKind::LLVMX86_FP80TypeKind => TypeKind::X86_FP80,
            LLVMTypeKind::LLVMFP128TypeKind => TypeKind::FP128,
            LLVMTypeKind::LLVMPPC_FP128TypeKind => TypeKind::PPC_FP128,
            LLVMTypeKind::LLVMLabelTypeKind => TypeKind::Label,
            LLVMTypeKind::LLVMIntegerTypeKind => TypeKind::Integer,
            LLVMTypeKind::LLVMFunctionTypeKind => TypeKind::Function,
            LLVMTypeKind::LLVMStructTypeKind => TypeKind::Struct,
            LLVMTypeKind::LLVMArrayTypeKind => TypeKind::Array,
            LLVMTypeKind::LLVMPointerTypeKind => TypeKind::Pointer,
            LLVMTypeKind::LLVMVectorTypeKind => TypeKind::Vector,
            LLVMTypeKind::LLVMMetadataTypeKind => TypeKind::Metadata,
            LLVMTypeKind::LLVMX86_MMXTypeKind => TypeKind::X86_MMX,
            LLVMTypeKind::LLVMTokenTypeKind => TypeKind::Token,
            LLVMTypeKind::LLVMScalableVectorTypeKind => TypeKind::ScalableVector,
            LLVMTypeKind::LLVMBFloatTypeKind => TypeKind::BFloat,
            LLVMTypeKind::LLVMX86_AMXTypeKind => TypeKind::X86_AMX,
            LLVMTypeKind::LLVMTargetExtTypeKind => TypeKind::TargetExt,
        }
    }
}

impl From<TypeKind> for LLVMTypeKind {
    fn from(kind: TypeKind) -> Self {
        match kind {
            TypeKind::Void => LLVMTypeKind::LLVMVoidTypeKind,
            TypeKind::Half => LLVMTypeKind::LLVMHalfTypeKind,
            TypeKind::Float => LLVMTypeKind::LLVMFloatTypeKind,
            TypeKind::Double => LLVMTypeKind::LLVMDoubleTypeKind,
            TypeKind::X86_FP80 => LLVMTypeKind::LLVMX86_FP80TypeKind,
            TypeKind::FP128 => LLVMTypeKind::LLVMFP128TypeKind,
            TypeKind::PPC_FP128 => LLVMTypeKind::LLVMPPC_FP128TypeKind,
            TypeKind::Label => LLVMTypeKind::LLVMLabelTypeKind,
            TypeKind::Integer => LLVMTypeKind::LLVMIntegerTypeKind,
            TypeKind::Function => LLVMTypeKind::LLVMFunctionTypeKind,
            TypeKind::Struct => LLVMTypeKind::LLVMStructTypeKind,
            TypeKind::Array => LLVMTypeKind::LLVMArrayTypeKind,
            TypeKind::Pointer => LLVMTypeKind::LLVMPointerTypeKind,
            TypeKind::Vector => LLVMTypeKind::LLVMVectorTypeKind,
            TypeKind::Metadata => LLVMTypeKind::LLVMMetadataTypeKind,
            TypeKind::X86_MMX => LLVMTypeKind::LLVMX86_MMXTypeKind,
            TypeKind::Token => LLVMTypeKind::LLVMTokenTypeKind,
            TypeKind::ScalableVector => LLVMTypeKind::LLVMScalableVectorTypeKind,
            TypeKind::BFloat => LLVMTypeKind::LLVMBFloatTypeKind,
            TypeKind::X86_AMX => LLVMTypeKind::LLVMX86_AMXTypeKind,
            TypeKind::TargetExt => LLVMTypeKind::LLVMTargetExtTypeKind,
        }
    }
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
    fn as_raw(&self) -> RawType;

    fn print_to_string(&self) -> String {
        self.as_raw().print_to_string()
    }
    fn print_to_stderr(&self) {
        eprintln!("{}", self.print_to_string());
    }

    fn to_type_enum(&self) -> TypeEnum;
}

pub trait TypeBuilder {
    fn func(&self, args: Vec<TypeEnum>, is_var_args: bool) -> function_types::FunctionType;
    fn array(&self, length: u64) -> array_types::ArrayType;
    fn ptr(&self) -> pointer_types::PointerTypes;
}

impl<T: Type> TypeBuilder for T {
    fn func(&self, args: Vec<TypeEnum>, is_var_args: bool) -> function_types::FunctionType {
        function_types::FunctionType::new(args, self.to_type_enum(), is_var_args)
    }

    fn array(&self, length: u64) -> array_types::ArrayType {
        array_types::ArrayType::new(self.to_type_enum(), length)
    }

    fn ptr(&self) -> pointer_types::PointerTypes {
        pointer_types::PointerTypes::new_const(self.to_type_enum())
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TypeEnum {
    IntType(int_types::IntType),
    FloatType(float_types::FloatType),
    FunctionType(function_types::FunctionType),
    ArrayType(array_types::ArrayType),
    PointerType(pointer_types::PointerTypes),
    StructType(struct_type::StructType),
    VectoreType(vectore_type::VectoreType),
    VoidType(void_type::VoidType),
}

impl TypeEnum {
    pub fn as_raw(&self) -> RawType {
        match self {
            TypeEnum::IntType(t) => t.as_raw(),
            TypeEnum::FloatType(t) => t.as_raw(),
            TypeEnum::FunctionType(t) => t.as_raw(),
            TypeEnum::ArrayType(t) => t.as_raw(),
            TypeEnum::PointerType(t) => t.as_raw(),
            TypeEnum::VoidType(t) => t.as_raw(),
            TypeEnum::StructType(t) => t.as_raw(),
            TypeEnum::VectoreType(t) => t.as_raw(),
        }
    }

    pub fn func(&self, args: Vec<TypeEnum>, is_var_args: bool) -> function_types::FunctionType {
        match self {
            TypeEnum::IntType(t) => t.func(args, is_var_args),
            TypeEnum::FloatType(t) => t.func(args, is_var_args),
            TypeEnum::ArrayType(t) => t.func(args, is_var_args),
            TypeEnum::FunctionType(t) => t.func(args, is_var_args),
            TypeEnum::PointerType(t) => t.func(args, is_var_args),
            TypeEnum::VoidType(t) => t.func(args, is_var_args),
            TypeEnum::StructType(t) => t.func(args, is_var_args),
            TypeEnum::VectoreType(t) => t.func(args, is_var_args),
        }
    }

    pub fn array(&self, length: u64) -> array_types::ArrayType {
        match self {
            TypeEnum::IntType(t) => t.array(length),
            TypeEnum::FloatType(t) => t.array(length),
            TypeEnum::ArrayType(t) => t.array(length),
            TypeEnum::FunctionType(t) => t.array(length),
            TypeEnum::PointerType(t) => t.array(length),
            TypeEnum::VoidType(t) => t.array(length),
            TypeEnum::StructType(t) => t.array(length),
            TypeEnum::VectoreType(t) => t.array(length),
        }
    }

    pub fn ptr(&self) -> pointer_types::PointerTypes {
        pointer_types::PointerTypes::new_const(*self)
    }

    pub fn into_function_type(self) -> function_types::FunctionType {
        match self {
            TypeEnum::FunctionType(t) => t,
            _ => panic!("Expected FunctionType, got {:?}", self),
        }
    }
    
    pub fn into_pointer_type(self) -> pointer_types::PointerTypes {
        match self {
            TypeEnum::PointerType(t) => t,
            _ => panic!("Expected PointerType, got {:?}", self),
        }
    }
    
    pub fn into_struct_type(self) -> struct_type::StructType {
        match self {
            TypeEnum::StructType(t) => t,
            _ => panic!("Expected StructType, got {:?}", self),
        }
    }
}


impl From<LLVMTypeRef> for TypeEnum {
    fn from(value: LLVMTypeRef) -> Self {
        let type_ = unsafe { LLVMGetTypeKind(value) };
        match type_ {
            LLVMTypeKind::LLVMIntegerTypeKind => {
                TypeEnum::IntType(int_types::IntType::new_llvm_ref(value))
            }
            LLVMTypeKind::LLVMDoubleTypeKind => {
                TypeEnum::FloatType(float_types::FloatType::new_with_llvm_ref(value))
            }
            LLVMTypeKind::LLVMFunctionTypeKind => {
                TypeEnum::FunctionType(function_types::FunctionType::new_with_llvm_ref(value))
            }
            LLVMTypeKind::LLVMArrayTypeKind => {
                TypeEnum::ArrayType(unsafe { array_types::ArrayType::new_with_llvm_ref(value) })
            }
            LLVMTypeKind::LLVMPointerTypeKind => {
                TypeEnum::PointerType(pointer_types::PointerTypes::new_llvm_ref(value))
            },
            LLVMTypeKind::LLVMVoidTypeKind => {
                TypeEnum::VoidType(unsafe { void_type::VoidType::new_with_llvm_ref(value) })
            },
            LLVMTypeKind::LLVMStructTypeKind => {
                TypeEnum::StructType(struct_type::StructType::new_with_llvm_ref(value))
            },
            LLVMTypeKind::LLVMVectorTypeKind => {
                TypeEnum::VectoreType(vectore_type::VectoreType::new_llvm_ref(value))
            },
            
            _ => panic!("Unknown type"),
        }

    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RawType {
    raw: LLVMTypeRef,
}


impl RawType {
    
    pub fn void_type() -> Self {
        Self::new(unsafe { LLVMVoidType() })
    }
    
    pub fn label_type() -> Self {
        Self::new(unsafe { LLVMLabelType() })
    }
    
    pub fn x86_mmx_type() -> Self {
        Self::new(unsafe { LLVMX86MMXType() })
    }
    
    pub fn x86_amx_type() -> Self {
        Self::new(unsafe { LLVMX86AMXType() })
    }
    
    
    pub fn new(raw: LLVMTypeRef) -> Self {
        Self { raw }
    }
    
    pub unsafe fn as_type_enum(&self) -> TypeEnum {
        TypeEnum::from(self.raw)
    }

    pub fn print_to_string(&self) -> String {
        let llvm_str = unsafe { LLVMPrintTypeToString(self.raw) };
        let str_slice = unsafe { std::ffi::CStr::from_ptr(llvm_str) }
            .to_str()
            .unwrap();
        str_slice.to_owned()
    }
    
    pub fn as_llvm_ref(&self) -> LLVMTypeRef {
        self.raw
    }
    
    pub fn get_type_kind(&self) -> TypeKind {
        let kind = unsafe { LLVMGetTypeKind(self.raw) };
        kind.into()
    }
    
    pub fn is_sized(&self) -> bool {
        unsafe { LLVMTypeIsSized(self.raw) == 1 }
    }
    
    pub fn get_context(&self) -> Option<Context> {
        let res = unsafe { LLVMGetTypeContext(self.raw) };
        
        ptr_to_option(res)
            .map(Context::new)
    }
    
    pub fn get_element_type(&self) -> Option<TypeEnum> {
        let res = unsafe { LLVMGetElementType(self.raw) };
        ptr_to_option(res)
            .map(TypeEnum::from)
    }

}
