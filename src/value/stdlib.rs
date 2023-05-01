use super::Object;
use crate::interpreter::Interpreter;


pub trait StdLibString {
    fn len(&self) -> Object;
}

pub trait StdLibInt {
    fn sqrt(interpreteur: &mut Interpreter, args: Vec<Object>, file: &str) -> Object;
}

#[macro_export]
macro_rules! register_stdlib {
    ($type_:ty, $std_name:ident, $($std_func_name:expr => $std_function:ident),* ) => {
        impl Getter for $type_ {
            fn fetch(&self, interpreteur: &mut Interpreter, obj: Object, name: Expr) -> Option<Object> {
                match *name.expr_type {
                    ExprType::Ident { ident } => {
                        match ident.lexeme.as_str() {
                            $($std_func_name => Some(BuiltinFunction::new(Rc::new(<$type_ as $std_name>::$std_function), 1).create_object())),*
                            ,
                            _ => None
                        }
                    },
                    ExprType::Call { ref name, args: old_args }  => {
                        let mut args = vec![obj.clone()];
                        for arg in old_args {
                            args.push(arg.accept(interpreteur));
                        }

                        match self.fetch(&mut interpreteur.clone(), obj.clone(), name.clone()) {
                            Some(obj) => {
                                match get_impl_if_exist!(Call, obj) {
                                    Some(call) => Some(call.call(interpreteur, args,  name.file.as_str())),
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