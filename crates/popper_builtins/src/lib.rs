use inkwell::builder::Builder;
use inkwell::context::Context;
use popper_ast::{Arguments, Function, Argument, Type, TypeKind};

pub trait BuiltinFunctions<'ctx> {
    fn llvm_fn_sign(&self) -> Function;
    fn sign_fn(&self) -> Function;
}


pub struct Print;

impl<'ctx> BuiltinFunctions<'ctx> for Print {
    fn llvm_fn_sign(&self) -> Function {
        Function::new(
            "print".to_string(),
            Arguments::new(
                vec![
                    Argument::new("string".to_string(),
                                  Type::new(Default::default(), TypeKind::Pointer(
                                      Box::new(Type::new(Default::default(), TypeKind::String(1), vec![]))
                                  ), Default::default()),
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

    fn sign_fn(&self) -> Function {
        Function::new(
            "print".to_string(),
            Arguments::new(
                vec![
                    Argument::new("string".to_string(),
                                  Type::new(Default::default(), TypeKind::String(1), vec![]),
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

