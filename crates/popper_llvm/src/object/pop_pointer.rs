use inkwell::types::PointerType;
use inkwell::values::PointerValue;

pub struct PopPointer<'ctx> {
    pub(crate) ty: PointerType<'ctx>,
    pub(crate) value: PointerValue<'ctx>
}

impl<'ctx> PopPointer<'ctx> {
    pub fn new(ty: PointerType<'ctx>, value: PointerValue<'ctx>) -> Self {
        PopPointer {
            ty,
            value
        }
    }

    pub fn from_value(value: PointerValue<'ctx>) -> Self {
        PopPointer {
            ty: value.get_type(),
            value
        }
    }
}
