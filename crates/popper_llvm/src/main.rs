use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicType, FunctionType};
use inkwell::values::{FunctionValue, IntValue, PointerValue};
use std::ptr;

fn main() {
    let context = Context::create();
    let module = context.create_module("print_module");

    // Déclaration de la fonction de print
    let i32_type = context.i32_type();
    let printf_type = i32_type.fn_type(&[i32_type.into()], true);
    let printf_func = module.add_function("printf", printf_type, None);

    // Déclaration de la fonction de print dans votre langage
    let print_type = i32_type.fn_type(&[i32_type.into()], false);
    let print_func = module.add_function("print", print_type, None);

    // Définition du corps de la fonction de print
    let entry_block = context.append_basic_block(print_func, "entry");
    let builder = context.create_builder();
    builder.position_at_end(entry_block);

    let format_string = context.const_string("%d\n".to_string().into_bytes().as_slice(), false);

    let arg_value = print_func.get_first_param().unwrap();
    builder.build_call(printf_func, &[format_string.into(), arg_value.into()], "printf_call");

    builder.build_return(None);

    // Code pour imprimer la fonction générée
    module.print_to_file("output.ll").expect("Erreur lors de l'impression du module LLVM");


}
