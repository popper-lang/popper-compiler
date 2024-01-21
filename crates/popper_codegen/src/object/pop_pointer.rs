#![allow(dead_code)]

use inkwell::types::PointerType;
use inkwell::values::PointerValue;

#[derive(Debug, Clone)]
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


    pub fn get_type(&self) -> PointerType<'ctx> {
        self.ty
    }

}

impl<'ctx> Into<PointerValue<'ctx>> for PopPointer<'ctx> {
    fn into(self) -> PointerValue<'ctx> {
        self.value
    }
}

impl<'ctx> Into<PopPointer<'ctx>> for PointerValue<'ctx> {
    fn into(self) -> PopPointer<'ctx> {
        PopPointer::from_value(self)
    }
}
