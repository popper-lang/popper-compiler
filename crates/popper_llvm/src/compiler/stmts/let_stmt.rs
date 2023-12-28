use crate::compiler::LLVMCompiler;
use popper_ast::LetStmt;
use crate::object::pop_object::PopObject;
use crate::object::pop_type::PopType;



impl LLVMCompiler<'_> {
    pub fn compile_let(&mut self, let_stmt: LetStmt) {
        let ty = PopType::from_ty_ast(let_stmt.r#type.unwrap().type_kind);
        let value = &self.compile_expr(let_stmt.value);
        let basic_value = value.to_basic_value_enum();
        let ptr = self.builder.build_alloca(ty.to_llvm_type(&self.context), format!("let_{}", let_stmt.name.name).as_str()).unwrap();
        let _store = self.builder.build_store(ptr, basic_value).expect("Failed to build store");


        self.env.set(let_stmt.name.name, PopObject::Ptr(ptr.get_type(), ptr));

    }
}