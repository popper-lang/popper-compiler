use crate::compiler::LLVMCompiler;

use popper_ast::Constant;
use crate::object::pop_object::PopObject;
use crate::object::pop_string::PopString;


impl<'ctx> LLVMCompiler<'ctx> {
    pub fn compile_int(&self, value: i32) -> PopObject {
        let ty = self.context.i32_type();
        let int_value = ty.const_int(value as u64, false);
        PopObject::Int(ty, int_value)
    }

    pub fn compile_float(&self, value: f32) -> PopObject {
        let ty = self.context.f32_type();
        let float_value = ty.const_float(value as f64);
        PopObject::Float(ty, float_value)
    }

    pub fn compile_string(&self, value: String) -> PopObject {
        let cstring = std::ffi::CString::new(value).expect("Cast failed");
        let bytes: &[u8] = cstring.as_bytes_with_nul();
        let array_value = self.context.const_string(bytes, false);
        PopObject::String(PopString::from_array_value(array_value))
    }

    pub fn compile_bool(&self, value: bool) -> PopObject {
        let ty = self.context.bool_type();
        let bool_value = ty.const_int(value as u64, false);
        PopObject::Bool(ty, bool_value)
    }

    pub fn compile_constant(&self, constant: Constant) -> PopObject {
        match constant {
            Constant::Int(int) => self.compile_int(int.value as i32),
            Constant::Float(float) => self.compile_float(float.value as f32),
            Constant::StringLiteral(string) => self.compile_string(string.value),
            Constant::Bool(boolean) => self.compile_bool(boolean.value),
            Constant::Ident(ident) => {
                let ptr = self.env.get(ident.name).unwrap();
                let ptr = ptr.value;
                let ty = ptr.get_type();
                let val = self.builder.build_load(ty, ptr, "load").unwrap();
                PopObject::from_basic_value_enum(val)
            },
            _ => todo!("Constant not implemented")
        }
    }
}