use crate::compiler::LLVMCompiler;

use crate::object::pop_type::PopType;
use inkwell::types::BasicMetadataTypeEnum;
use inkwell::values::BasicValueEnum;
use inkwell::types::BasicType;

use crate::object::pop_object::PopObject;
use crate::object::pop_pointer::PopPointer;

impl<'ctx> LLVMCompiler<'ctx> {
    pub fn compile_function(&mut self, function: popper_ast::Function) {
        let args = function
            .arguments
            .args
            .iter()
            .map(|x|
                PopType::from_ty_ast(x.ty.clone().type_kind).to_basic_metadata_type(&self.context)
            )
            .collect::<Vec<BasicMetadataTypeEnum>>()
            ;
        let func_ty = PopType::from_ty_ast(function.returntype.type_kind).to_llvm_type(&self.context);
        let ret_ptr = func_ty.ptr_type(Default::default());
        let func_ty = ret_ptr.fn_type(args.as_slice(), false);

        let func = self.module.add_function(function.name.as_str(), func_ty, None);

        let block = self.context.append_basic_block(func, "entry");

        self.builder.position_at_end(block);
        for (i, (arg, stmt_arg)) in func.get_param_iter().into_iter().zip(
            function.arguments.args.iter()
        ).enumerate() {
            // get the arg to a pointer
            self.env.set(stmt_arg.name.clone(), PopObject::from_basic_value_enum(arg));
        }

        for stmt in function.body {
            self.compile_stmt(stmt);
        }
        self.current_basic_block = Some(block);
    }
}