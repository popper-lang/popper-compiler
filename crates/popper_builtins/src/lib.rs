use inkwell::AddressSpace;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{BasicMetadataValueEnum, BasicValue, BasicValueEnum, FunctionValue};
use popper_ast::{Arguments, Function, Argument, Type, TypeKind};

pub trait BuiltinFunctions<'ctx> {
    fn llvm_fn_sign(&self, context: &'ctx Context, module: &inkwell::module::Module<'ctx>, builder: &'ctx Builder) -> FunctionValue<'ctx>;
    fn llvm_body(&self, context: &'ctx Context, module: &inkwell::module::Module<'ctx>, builder: &'ctx Builder, function: FunctionValue) -> Result<(), ()>;
    fn sign_fn(&self) -> Function;
}


pub struct Print;

impl<'ctx> BuiltinFunctions<'ctx> for Print {
    fn llvm_fn_sign(&self, context: &'ctx Context, module: &inkwell::module::Module<'ctx>, builder: &'ctx Builder) -> FunctionValue<'ctx> {
        let i32_type = context.i32_type();
        let i8_type = context.i8_type();
        let param_type = &[i8_type.ptr_type(Default::default()).into()];
        let function_type = i32_type.fn_type(param_type, false);
        let function_value = module.add_function("print", function_type, None);
        builder.position_at_end(function_value.get_last_basic_block().unwrap());
        return function_value.clone();
    }

    fn llvm_body(&self, context: &Context, module: &Module<'ctx>, builder: &'ctx Builder, function: FunctionValue) -> Result<(), ()> {
        let i32_type = context.i32_type();
        let i32_zero = i32_type.const_int(0, false);
        let i8_type = context.i8_type();
        let fmt_string = builder.build_global_string_ptr("%s\n", "fmt").unwrap();
        let ptr = builder.build_alloca(i8_type.ptr_type(Default::default()), "ptr").unwrap();
        builder.build_store(ptr, function.get_nth_param(0).unwrap()).unwrap();
        let _ = builder.build_call(
            module.get_function("printf").unwrap(),
            &[fmt_string.as_basic_value_enum().into(), ptr.as_basic_value_enum().into()],
            "printf",
        ).unwrap();

        builder.build_return(Some(&i32_zero)).unwrap();

        Ok(())
    }

    fn sign_fn(&self) -> Function {
        Function::new(
            "print".to_string(),
            Arguments::new(
                vec![
                    Argument::new("string".to_string(),
                                  Type::new(Default::default(), TypeKind::String(u32::MAX), Default::default()),
                                    Default::default()
                    )
                ],
                Default::default()
            ),
            Type::new(Default::default(), TypeKind::Int, Vec::new()),
            vec![],
            Default::default()
        )
    }
}


pub fn load_builtins<'ctx>() -> Vec<Box<dyn BuiltinFunctions<'ctx>>> {
    vec![
        Box::new(Print)
    ]
}

pub static BUILTINS_NAMES: [&str; 1] = [
    "print"
];

