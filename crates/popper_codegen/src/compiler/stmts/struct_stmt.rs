use popper_ast::{StructInstance, StructStmt};
use std::collections::HashMap;
use crate::compiler::LLVMCompiler;
use crate::object::pop_object::PopObject;
use crate::object::pop_type::PopType;

impl<'ctx> LLVMCompiler<'ctx> {
    pub fn compile_struct(&mut self, struct_stmt: StructStmt) {
        let field_ty = struct_stmt
            .fields
            .iter()
            .map(|x|
                PopType::from_ty_ast(x.ty.type_kind.clone())
                    .to_llvm_type(self.context)
            ).collect::<Vec<_>>();

        let hashmap = struct_stmt
            .fields
            .iter()
            .enumerate()
            .map(|(i, x)| (x.name.clone(), i as u8))
            .collect::<HashMap<_, _>>();

        self.structs.insert(struct_stmt.name.clone(), hashmap);

        let struct_ty = self.context.opaque_struct_type(&struct_stmt.name);
        struct_ty.set_body(&field_ty, false);
        self.env.set(struct_stmt.name, PopObject::new_struct(struct_ty.const_zero()));
    }

    pub fn get_struct_field_index(&self, struct_name: &str, field_name: &str) -> u8 {
        self.structs.get(struct_name).unwrap().get(field_name).unwrap().clone()
    }

    pub fn compile_struct_instance(&self, struct_instance: StructInstance) -> PopObject<'_> {
        let struct_ty = self.env.get(struct_instance.name.clone()).unwrap();
        let struct_ty = struct_ty.clone().get_type().into_struct_type();
        let mut fields = Vec::new();

        for field in struct_instance.fields {
            fields.push(self.compile_expr(field.value).to_basic_value_enum().clone());
        }

        PopObject::new_struct(struct_ty.const_named_struct(&fields))

    }
}