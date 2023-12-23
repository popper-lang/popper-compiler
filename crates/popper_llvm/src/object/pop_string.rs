use inkwell::types::ArrayType;
use inkwell::values::ArrayValue;
use inkwell::context::Context;
use std::ffi::CString;

pub struct PopString<'a> {
    pub array_ty: ArrayType<'a>,
    pub array_value: ArrayValue<'a>,
}

impl<'a> PopString<'a> {
    pub fn from_string(context: &'a Context, string: String) -> Self {
        let i8_ty = context.i8_type();
        let cstring = CString::new(string).expect("Cast failed");
        let bytes: &[u8] = cstring.as_bytes_with_nul();
        let array_ty = i8_ty.array_type(bytes.len() as u32);
        let array_value = context.const_string(bytes, false);
        PopString {
            array_ty,
            array_value,
        }
    }

    pub fn from_array_value(array_value: ArrayValue<'a>) -> Self {
        let array_ty = array_value.get_type();
        PopString {
            array_ty,
            array_value,
        }
    }

    pub fn build_string(&self) -> String {
        self.array_value.to_string()
    }
}
