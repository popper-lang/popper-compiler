use inkwell::values::BasicValue;
use crate::compiler::LLVMCompiler;
use popper_ast::LetStmt;
use crate::object::pop_object::PopObject;
use crate::object::pop_pointer::PopPointer;
use crate::object::pop_type::PopType;



impl<'ctx> LLVMCompiler<'ctx> {
    pub fn compile_let(&mut self, let_stmt: LetStmt) {
        let ty = PopType::from_ty_ast(let_stmt.r#type.unwrap().type_kind);
        let llvm_ty = ty.to_llvm_type(&self.context);
        let value = self.compile_expr(let_stmt.value);
        let basic_value = value.clone().to_basic_value_enum();
        let ptr = self.builder.build_alloca(llvm_ty, format!("let_{}", let_stmt.name.name).as_str()).unwrap();
        let store = self.builder.build_store(ptr, basic_value).expect("Failed to build store");
        let basic_val = store.get_operand(0).unwrap().unwrap_left();

        self.env.set(let_stmt.name.name, value);

    }


}