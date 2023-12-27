use inkwell::types::ArrayType;
use inkwell::values::{ArrayValue, BasicValue, GlobalValue};
use inkwell::context::Context;
use std::ffi::CString;

#[derive(Debug, Clone)]
pub struct PopString<'a> {
    pub global_value: GlobalValue<'a>,
}

impl<'a> PopString<'a> {
    pub fn new(global_value: GlobalValue<'a>) -> Self {
        Self {
            global_value
        }
    }
}