use crate::compiler::LLVMCompiler;

use crate::object::pop_type::PopType;
use inkwell::types::BasicMetadataTypeEnum;
use inkwell::values::BasicValueEnum;
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

        let func_ty = func_ty.into_int_type().fn_type(args.as_slice(), false);

        let func = self.module.add_function(function.name.as_str(), func_ty, None);

        let block = self.context.append_basic_block(func, "entry");

        self.builder.position_at_end(block);
        for (arg, stmt_arg) in func.get_param_iter().into_iter().zip(
            function.arguments.args.iter()
        ) {
            let ptr_value = match arg {
                BasicValueEnum::PointerValue(ptr) => ptr,
                BasicValueEnum::IntValue(int) => {
                    let ptr = self.builder.build_alloca(int.get_type(), "alloca").unwrap();
                    self.builder.build_store(ptr, int);
                    ptr
                },
                BasicValueEnum::ArrayValue(array) => {
                    let ptr = self.builder.build_alloca(array.get_type(), "alloca").unwrap();
                    self.builder.build_store(ptr, array);
                    ptr
                },
                BasicValueEnum::FloatValue(float) => {
                    let ptr = self.builder.build_alloca(float.get_type(), "alloca").unwrap();
                    self.builder.build_store(ptr, float);
                    ptr
                },
                BasicValueEnum::StructValue(struct_value) => {
                    let ptr = self.builder.build_alloca(struct_value.get_type(), "alloca").unwrap();
                    self.builder.build_store(ptr, struct_value);
                    ptr
                },
                BasicValueEnum::VectorValue(vector) => {
                    let ptr = self.builder.build_alloca(vector.get_type(), "alloca").unwrap();
                    self.builder.build_store(ptr, vector);
                    ptr
                }
                _ => todo!()
            };
            self.env.set(stmt_arg.name.clone(), PopPointer::from_value(ptr_value));
        }
        for stmt in function.body {
            self.compile_stmt(stmt);
        }
        self.current_basic_block = Some(block);
    }
}