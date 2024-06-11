use popper_llvm::types::TypeEnum;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Type {
    ty: TypeEnum,
    real_ty: TypeEnum
}

impl Type {
    
    pub fn from_type(ty: TypeEnum) -> Self {
        Self { ty, real_ty: ty }
    }
    pub fn new(ty: TypeEnum, real_ty: TypeEnum) -> Self {
        Self { ty, real_ty }
    }

    pub fn get_type(&self) -> TypeEnum {
        self.ty
    }

    pub fn get_real_type(&self) -> TypeEnum {
        self.real_ty
    }
}