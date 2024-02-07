use popper_llvm::value::{
    int_value,
    function_value,
    float_value,
    ValueEnum,
    ToValue,
};

use popper_llvm::types::{
    int_types,
    float_types,
    function_types,
    TypeEnum,
};



#[derive(Debug, Clone, Copy)]
pub struct PopObject {
    pub(crate) value: ValueEnum,
    pub(crate) type_enum: TypeEnum,
}

impl PopObject {

    pub fn from(value: ValueEnum) -> Self {
        Self {
            value,
            type_enum: value.get_type()
        }
    }
    pub fn new(value: ValueEnum, type_enum: TypeEnum) -> Self {
        Self {
            value,
            type_enum
        }
    }

}