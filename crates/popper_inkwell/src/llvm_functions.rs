use inkwell::{builder::Builder, context::Context, module::Module, values::FunctionValue};





pub trait LLVMBuiltinFunction {
    fn get_type<'f>(context: &'f Context) -> inkwell::types::FunctionType<'f>;
    fn get_name() -> String;
    fn body<'v>(context: &'v Context, module: &'v Module<'v>, builder: &'v Builder) -> FunctionValue<'v>;
}

#[allow(non_camel_case_types)]
pub struct popper_va_arg_null_check;

impl LLVMBuiltinFunction for popper_va_arg_null_check {
    fn get_type<'f>(context: &'f Context) -> inkwell::types::FunctionType<'f> {
        let i32_type = context.i32_type().ptr_type(Default::default());
        context.void_type().fn_type(&[i32_type.into()], false)
    }

    fn get_name() -> String {
        "popper.va_arg_null_check".to_string()
    }

    fn body<'v>(context: &'v Context, module: &'v Module<'v>, builder: &'v Builder) -> FunctionValue<'v> {
        let function = module.add_function(
            "popper.va_arg_null_check",
            Self::get_type(context),
            None,
        );

        let basic_block = context.append_basic_block(function, "entry");
        builder.position_at_end(basic_block);

        let va_arg = function.get_first_param().unwrap().into_pointer_value();
        let is_null = builder.build_is_null(va_arg, "is_null").unwrap();
        let then_block = context.append_basic_block(function, "then");
        let else_block = context.append_basic_block(function, "else");
        builder.build_conditional_branch(is_null, then_block, else_block);

        builder.position_at_end(then_block);
        builder.build_call(module.get_function("popper.panic").expect("panic not found"), &[], "");

        builder.position_at_end(else_block);
        builder.build_return(None);

        function
    }
}


#[allow(non_camel_case_types)]
pub struct popper_panic;

impl LLVMBuiltinFunction for popper_panic {
    fn get_type<'f>(context: &'f Context) -> inkwell::types::FunctionType<'f> {
        context.void_type().fn_type(&[], false)
    }

    fn get_name() -> String {
        "popper.panic".to_string()
    }

    fn body<'v>(context: &'v Context, module: &'v Module<'v>, builder: &'v Builder) -> FunctionValue<'v> {
        let function = module.add_function(
            "popper.panic",
            Self::get_type(context),
            None,
        );

        let basic_block = context.append_basic_block(function, "entry");
        builder.position_at_end(basic_block);

        let panic_message = context.const_string("panic".as_bytes(), false);
        builder.build_call(
            module.get_function("puts").unwrap(),
            &[panic_message.into()],
            "puts",
        );

        builder.build_call(
            module.get_function("exit").unwrap(),
            &[context.i32_type().const_int(1, false).into()],
            "exit",
        );

        builder.build_unreachable();

        function
    }
}
