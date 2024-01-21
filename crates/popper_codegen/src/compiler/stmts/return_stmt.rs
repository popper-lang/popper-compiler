use inkwell::types::BasicType;

use crate::compiler::LLVMCompiler;
use popper_ast::Return;
use crate::object::pop_object::PopObject;

impl<'ctx> LLVMCompiler<'ctx> {
    pub fn compile_return(&mut self, return_stmt: Return) {
        let value = return_stmt.expression.and_then(|x| Some(self.compile_expr(*x)));
        let value = value.unwrap_or(PopObject::new_int(&self.context, 0));
        let ptr_ty = value.clone().get_type().ptr_type(Default::default());
        let ptr = self.builder.build_alloca(ptr_ty, "return").unwrap();
        self.builder.build_store(ptr, value.clone().to_basic_value_enum()).expect("Failed to build store");
        self.builder.build_return(
            Some(&ptr)
        ).unwrap();
    }
}