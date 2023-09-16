#![allow(dead_code)]

use std::string::ToString;
use popper_ast::{Span, TypeKind};
use popper_asm::ast::{Register, Command, MemoryFetching, Mov, Add, Expr};
use popper_flag::{SymbolFlags, ValueFlag};
use popper_error::{diff_length_of_argument::DiffLengthOfArgument, typemismatch::TypeMismatch,  Error};


pub trait Builtin {

    fn args_type(&self) -> Vec<TypeKind>;
    fn ret_type(&self) -> TypeKind;

    fn load(&self) -> Vec<Command>;

    fn check(&self, arguments: Vec<SymbolFlags>, span: Span) -> Result<(), Box<dyn Error>>;
    fn call(&self) -> Vec<Command>;

    fn lang_name(&self) -> String;

    fn to_value_flag(&self) -> ValueFlag {
        ValueFlag::Function(
            self.args_type().iter().map(|x| ValueFlag::from_ty_kind(x.clone())).collect(),
            Box::new(ValueFlag::from_ty_kind(self.ret_type()))
        )
    }


}


#[derive(Clone)]
struct CustomBuiltin {
    lang_name: String,
    program: Vec<Command>,
    preload_asm: Option<Vec<Command>>,
    argument_type: Vec<TypeKind>,
    ret_type: TypeKind
}

pub type Builtins = Vec<Box<dyn Builtin>>;


fn builtin_sum() -> CustomBuiltin {
    CustomBuiltin {
        lang_name: "sum".to_string(),
        program: vec![
            Command::Add(
                Add(MemoryFetching::Register(Register::R1), Expr::Memory(MemoryFetching::Register(Register::R2)))
            )
        ],
        preload_asm: None,
        argument_type: vec![TypeKind::Int, TypeKind::Int],
        ret_type: TypeKind::Int
    }
}

fn builtin_print() -> CustomBuiltin {
    CustomBuiltin {
        lang_name: "print".to_string(),
        program: vec![
            Command::Mov(
                Mov(MemoryFetching::Addr(500), Expr::Memory(
                    MemoryFetching::Register(Register::R1)
                ))
            )
        ],
        preload_asm: None,
        argument_type: vec![TypeKind::Int],
        ret_type: TypeKind::Unit
    }
}

pub fn builtins() -> Builtins {
    dbg!(vec![
        Box::new(
            builtin_print()
        ),
        Box::new(
            builtin_sum()
        )
    ])
}

pub fn get_builtin(lang_name: &str, builtins: Builtins) -> Option<Box<dyn Builtin>> {
    builtins
        .into_iter()
        .find(|elt| elt.lang_name() == lang_name)
}


impl Builtin for CustomBuiltin {
    fn args_type(&self) -> Vec<TypeKind> {
        self.clone().argument_type
    }

    fn ret_type(&self) -> TypeKind {
        self.clone().ret_type
    }

    fn load(&self) -> Vec<Command> {
        vec![]
    }

    fn check(&self, arguments: Vec<SymbolFlags>, span: Span) -> Result<(), Box<dyn Error>> {
        global_check(arguments, span, self.clone().argument_type)
    }

    fn call(&self) -> Vec<Command> {
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


