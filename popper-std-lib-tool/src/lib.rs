use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

macro_rules! register_method {
    ($name:ident, $method:ident, $fun:ident, { $($key:literal => $value:ident),* $(,)? }) => {
        impl $method for $name {
            fn $fun(&self, interpreteur: &mut Interpreter, name: Expr) -> Option<Object> {
                match *name.expr_type {
                    ExprType::Ident { ident } => {
                        match ident.lexeme {
                            $( $key => Some(self.$value), )*
                            _ => None
                        }
                    },
                    ExprType::Call { ref name, args: old_args }  => {
                        let mut args = vec![];
                        for arg in old_args {
                            args.push(arg.accept(interpreteur));
                        }
                        match self.fetch(&mut interpreteur.clone(), name.clone()) {
                            Some(obj) => {
                                match get_impl_if_exist!(Call, obj) {
                                    Some(call) => Some(call.call(interpreteur, args, name.file.as_str())),
                                    None => {
                                        error!(ErrorType::TypeError, "Expected a function", 0..0, "".to_string());
                                        unreachable!()
                                    }
                                }
                            },
                            None => None
                        }
                    },
                    _ => None
                }
            }
        }
    };
}
/*

register_method!(PopperString, Get, {
    "len" => len,
    "to_string" => to_string,


 */