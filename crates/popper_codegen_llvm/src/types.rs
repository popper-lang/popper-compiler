use inkwell::types::{
    AnyType, AnyTypeEnum, BasicMetadataTypeEnum, BasicType, BasicTypeEnum, FunctionType, VoidType,
};
use popper_ast::type_::Type;
use std::hash::{Hash, Hasher};

pub fn type_hash(ty: &Type) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    ty.to_string().hash(&mut hasher);
    hasher.finish()
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LLVMType<'t> {
    BasicType(BasicTypeEnum<'t>),
    FunctionType(FunctionType<'t>),
    VoidType(VoidType<'t>),
}

impl<'t> LLVMType<'t> {
    pub fn is_function_type(&self) -> bool {
        matches!(self, LLVMType::FunctionType(_))
    }

    pub fn is_void_type(&self) -> bool {
        matches!(self, LLVMType::VoidType(_))
    }

    pub fn as_any_type_enum(&self) -> AnyTypeEnum<'t> {
        match self {
            LLVMType::BasicType(bt) => bt.as_any_type_enum(),
            LLVMType::FunctionType(ft) => ft.as_any_type_enum(),
            LLVMType::VoidType(vt) => vt.as_any_type_enum(),
        }
    }

    pub fn into_metadata_type(self) -> BasicMetadataTypeEnum<'t> {
        match self {
            LLVMType::BasicType(bt) => BasicMetadataTypeEnum::try_from(bt)
                .unwrap_or_else(|_| panic!("Not a metadata type")),
            _ => panic!("Not a basic type"),
        }
    }
    pub fn into_function_type(self) -> FunctionType<'t> {
        match self {
            LLVMType::FunctionType(ft) => ft,
            _ => panic!("Not a function type"),
        }
    }

    pub fn into_void_type(self) -> VoidType<'t> {
        match self {
            LLVMType::VoidType(vt) => vt,
            _ => panic!("Not a void type"),
        }
    }

    pub fn into_basic_type(self) -> BasicTypeEnum<'t> {
        match self {
            LLVMType::BasicType(bt) => bt,
            e => panic!("Not a basic type: {:?}", e),
        }
    }
}

impl<'t> From<BasicTypeEnum<'t>> for LLVMType<'t> {
    fn from(bt: BasicTypeEnum<'t>) -> Self {
        LLVMType::BasicType(bt)
    }
}

impl<'t> From<FunctionType<'t>> for LLVMType<'t> {
    fn from(ft: FunctionType<'t>) -> Self {
        LLVMType::FunctionType(ft)
    }
}

impl<'t> From<VoidType<'t>> for LLVMType<'t> {
    fn from(vt: VoidType<'t>) -> Self {
        LLVMType::VoidType(vt)
    }
}
