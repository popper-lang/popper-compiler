use inkwell::types::BasicType;
use inkwell::types::BasicMetadataTypeEnum;
use crate::compiler::LLVMCompiler;
use crate::object::pop_type::PopType;


impl<'ctx> LLVMCompiler<'ctx> {
    pub fn compile_external(&mut self, external: popper_ast::External) {
        for sign in external.signs {

            let fn_name = sign.name.clone();

            let fn_args = sign
                .arguments
                .args
                .iter()
                .map(|x| x.ty.clone())
                .map(|x| PopType::from_ty_ast(x.type_kind))
                .map(|x| x.to_llvm_type(self.context))
                .map(|x| x.into())
                .collect::<Vec<BasicMetadataTypeEnum>>();

            let fn_return_type = PopType::from_ty_ast(sign.return_type.type_kind).to_llvm_type(self.context);

            let fn_type = fn_return_type.fn_type(&fn_args, false);
            self.module.add_function(fn_name.as_str(), fn_type, None);
            self.compile_from_rust_to_dylib(external.file.clone());
        }



    }
}