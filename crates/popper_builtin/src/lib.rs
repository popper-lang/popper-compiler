#![allow(dead_code)]

use std::string::ToString;
use popper_ast::{Span, TypeKind};
use popper_asm::asm_value::AsmValue;
use popper_asm::builder::{Assembly, Program};
use popper_asm::register::Register;
use popper_flag::{SymbolFlags, ValueFlag};
use popper_error::{diff_length_of_argument::DiffLengthOfArgument, typemismatch::TypeMismatch,  Error};

pub trait Builtin {

    fn args_type(&self) -> Vec<TypeKind>;
    fn ret_type(&self) -> TypeKind;

    fn load(&self) -> Program;

    fn check(&self, arguments: Vec<SymbolFlags>, span: Span) -> Result<(), Box<dyn Error>>;
    fn call(&self) -> Program;

    fn lang_name(&self) -> String;

    fn to_value_flag(&self) -> ValueFlag {
        ValueFlag::Function(
            self.args_type().iter().map(|x| ValueFlag::from_ty_kind(x.clone())).collect(),
            Box::new(ValueFlag::from_ty_kind(self.ret_type()))
        )
    }


}
#[derive(Clone)]
struct BuiltinFunctionFromCStdLib {
    lang_name: String,
    c_name: String,
    argument_type: Vec<TypeKind>,
    ret_type: TypeKind
}

#[derive(Clone)]
struct CustomBuiltin {
    lang_name: String,
    program: Program,
    preload_asm: Option<Program>,
    argument_type: Vec<TypeKind>,
    ret_type: TypeKind
}

pub type Builtins = Vec<Box<dyn Builtin>>;


fn builtin_print() -> BuiltinFunctionFromCStdLib {
    BuiltinFunctionFromCStdLib {
        lang_name: "print".to_string(),
        c_name: "_printf".to_string(),
        argument_type: vec![TypeKind::String],
        ret_type: TypeKind::Unit
    }
}

fn builtin_sum() -> CustomBuiltin {
    CustomBuiltin {
        lang_name: "sum".to_string(),
        program: vec![
            Assembly::IAdd(Register::R1, Box::new(AsmValue::Register(Register::R2))),
            Assembly::Ret
        ],
        preload_asm: None,
        argument_type: vec![TypeKind::Int, TypeKind::Int],
        ret_type: TypeKind::Int
    }
}

pub fn builtins() -> Builtins {
    vec![
        Box::new(
            builtin_print()
        ),
        Box::new(
            builtin_sum()
        )
    ]
}

pub fn get_builtin(lang_name: &str, builtins: Builtins) -> Option<Box<dyn Builtin>> {
    builtins
        .into_iter()
        .find(|elt| elt.lang_name() == lang_name)
}

impl Builtin for BuiltinFunctionFromCStdLib {
    fn args_type(&self) -> Vec<TypeKind> {
        self.clone().argument_type
    }

    fn ret_type(&self) -> TypeKind {
        self.clone().ret_type
    }
    fn load(&self) -> Program {
        vec![] // return nothing because C builtin is by default loaded
    }

    fn check(&self, arguments: Vec<SymbolFlags>, span: Span) -> Result<(), Box<dyn Error>> {
        global_check(arguments, span, self.clone().argument_type)
    }

    fn call(&self) -> Program {
        vec![
            Assembly::Call(self.c_name.clone()),
            Assembly::Ret
        ]
    }

    fn lang_name(&self) -> String {
        self.lang_name.clone()
    }
}

impl Builtin for CustomBuiltin {
    fn args_type(&self) -> Vec<TypeKind> {
        self.clone().argument_type
    }

    fn ret_type(&self) -> TypeKind {
        self.clone().ret_type
    }

    fn load(&self) -> Program {
        vec![]
    }

    fn check(&self, arguments: Vec<SymbolFlags>, span: Span) -> Result<(), Box<dyn Error>> {
        global_check(arguments, span, self.clone().argument_type)
    }

    fn call(&self) -> Program {
        self.clone().program
    }

    fn lang_name(&self) -> String {
        self.lang_name.clone()
    }

}


fn global_check(arguments: Vec<SymbolFlags>, span: Span, argument_model: Vec<TypeKind>) -> Result<(), Box<dyn Error>> {
    let args_expected: Vec<ValueFlag> = argument_model.iter().map(|x| ValueFlag::from_ty_kind(x.clone())).collect();

    if args_expected.len() != arguments.len() {
        return Err(
            Box::new(
                DiffLengthOfArgument::new(args_expected.len(), arguments.len(), span)
            )
        );
    }

    for (arg_expected, arg_got) in args_expected.iter().zip(arguments) {
        let value = arg_got.get_value().unwrap();
        if arg_expected.clone() !=  value {
            return Err(
                Box::new(
                    TypeMismatch::new((Span::new(0, 0), arg_expected.to_string()), (span, value.to_string()))
                )
            )
        }
    }

    Ok(())
}


