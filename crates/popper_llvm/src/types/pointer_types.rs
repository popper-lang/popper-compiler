use crate::types::function_types::FunctionType;
use crate::types::{Type, TypeEnum};
use llvm_sys::prelude::LLVMTypeRef;

#[derive(Debug, Clone, Copy)]
pub struct PointerTypes {
    pub(crate) ty: LLVMTypeRef,
}

impl PointerTypes {
    pub fn new_const(ty: TypeEnum) -> Self {
        let ty = unsafe { llvm_sys::core::LLVMPointerType(ty.get_type_ref(), 0) };
        Self { ty }
    }
    pub fn new_llvm_ref(ty: LLVMTypeRef) -> Self {
        Self { ty }
    }

    pub fn func(&self, args: Vec<TypeEnum>, is_var_args: bool) -> FunctionType {
        FunctionType::new(args, self.to_type_enum(), is_var_args)
    }

    pub fn to_type_enum(self) -> TypeEnum {
        TypeEnum::PointerType(self)
    }

    pub fn get_llvm_ref(&self) -> LLVMTypeRef {
        self.ty
    }
}

impl Type for PointerTypes {
    fn is_sized(&self) -> bool {
        false
    }

    fn get_type_ref(&self) -> LLVMTypeRef {
        self.get_llvm_ref()
    }
}
