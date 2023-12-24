
pub(crate) fn _llvm_ir_to_inkwell_fn(func: llvm_ir::function::Function, module: inkwell::module::Module) -> inkwell::values::FunctionValue {
    let arguments = func.parameters.iter().map(|x| {
        (_llvm_ir_to_inkwell_ty(x.ty, context).into(), x.name.clone())
    }).collect::<Vec<(BasicMetadataTypeEnum, String)>>();

    let func_ty = _llvm_ir_to_inkwell_ty(func.return_type, context).into_int_type().fn_type(
        &arguments.iter().map(|x| x.0.clone()).collect::<Vec<BasicMetadataTypeEnum>>(),
        func.is_var_arg
    );

    let func = module.add_function(func.name.as_str(), func_ty, None);

    return func
}

pub(crate) fn _llvm_ir_to_inkwell_ty(ty: llvm_ir::types::TypeRef, context: inkwell::context::Context) -> inkwell::types::BasicTypeEnum {
    match ty.as_ref() {
        llvm_ir::types::Type::VoidType => inkwell::types::BasicTypeEnum::IntType(context.i8_type()),
        llvm_ir::types::Type::IntegerType { bits } => inkwell::types::BasicTypeEnum::IntType(context.i32_type()),
        llvm_ir::types::Type::PointerType { addr_space } => {
            inkwell::types::BasicTypeEnum::PointerType(context.i32_type().ptr_type(inkwell::AddressSpace::from(addr_space)))
        },
        llvm_ir::types::Type::ArrayType { element_type, .. } => {
            inkwell::types::BasicTypeEnum::ArrayType(_llvm_ir_to_inkwell_ty(*element_type, context).into_array_type())
        },
        llvm_ir::types::Type::FuncType { result_type, param_types, is_var_arg } => {
            inkwell::types::BasicTypeEnum::FunctionType(
                _llvm_ir_to_inkwell_ty(*result_type, context).into_int_type().fn_type(
                    &param_types.iter().map(|x| _llvm_ir_to_inkwell_ty(*x, context)).collect::<Vec<inkwell::types::BasicTypeEnum>>(),
                    *is_var_arg
                )
            )
        },
        _ => todo!("Type not implemented")
    }
}

pub fn _llvm_ir_to_inkwell_basic_block(basic_block: llvm_ir::basicblock::BasicBlock, context: inkwell::context::Context) -> inkwell::basic_block::BasicBlock {
