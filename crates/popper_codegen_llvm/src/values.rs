use inkwell::values::{
    AnyValue, AnyValueEnum, BasicMetadataValueEnum, BasicValueEnum, FunctionValue,
};
#[derive(Clone, Debug)]
pub enum LLVMValue<'v> {
    BasicValue(BasicValueEnum<'v>),
    FunctionValue(FunctionValue<'v>),
}

impl<'v> LLVMValue<'v> {
    pub fn as_any_value_enum(&self) -> AnyValueEnum<'v> {
        match self {
            LLVMValue::BasicValue(bv) => bv.as_any_value_enum(),
            LLVMValue::FunctionValue(fv) => fv.as_any_value_enum(),
        }
    }

    pub fn into_function_value(self) -> FunctionValue<'v> {
        match self {
            LLVMValue::FunctionValue(fv) => fv,
            _ => panic!("Not a function value"),
        }
    }

    pub fn into_basic_value(self) -> BasicValueEnum<'v> {
        match self {
            LLVMValue::BasicValue(bv) => bv,
            _ => panic!("Not a basic value"),
        }
    }

    pub fn into_metadata_value(self) -> BasicMetadataValueEnum<'v> {
        match self {
            LLVMValue::BasicValue(bv) => BasicMetadataValueEnum::try_from(bv)
                .unwrap_or_else(|_| panic!("Not a metadata value")),
            _ => panic!("Not a basic value"),
        }
    }

    pub fn is_function_value(&self) -> bool {
        matches!(self, LLVMValue::FunctionValue(_))
    }
}

impl<'f> From<BasicValueEnum<'f>> for LLVMValue<'f> {
    fn from(bv: BasicValueEnum<'f>) -> Self {
        LLVMValue::BasicValue(bv)
    }
}

impl<'f> TryFrom<AnyValueEnum<'f>> for LLVMValue<'f> {
    type Error = &'static str;

    fn try_from(av: AnyValueEnum<'f>) -> Result<Self, Self::Error> {
        if let Ok(r) = BasicValueEnum::try_from(av) {
            Ok(LLVMValue::BasicValue(r))
        } else if let Ok(r) = FunctionValue::try_from(av) {
            Ok(LLVMValue::FunctionValue(r))
        } else {
            Err("Unsupported AnyValueEnum type")
        }
    }
}
impl<'f> From<FunctionValue<'f>> for LLVMValue<'f> {
    fn from(fv: FunctionValue<'f>) -> Self {
        LLVMValue::FunctionValue(fv)
    }
}
