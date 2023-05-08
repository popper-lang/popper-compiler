use super::Object;
use crate::interpreter::Interpreter;

#[macro_export]
macro_rules! function {
    ($name:ident) => {
        fn $name(_interpreteur: &mut Interpreter, _this: &mut Object, _args: &mut Vec<Object>) -> Object {
            todo!()
        }
    };
}

pub trait StdLibString {
    function!(push);
    function!(len);
}

pub trait StdLibInt {
    function!(sqrt);
    function!(to_string);
}

pub trait StdLibList {
    function!(push);
    function!(extend);
    function!(to_string);
    function!(nth);
}




#[macro_export]
macro_rules! register_stdlib {
    ($type_:ty, $std_name:ident, { $($std_func_name:expr => $name:ident),* } ) => {
        impl Getter for $type_ {
            fn fetch(&self, interpreteur: &mut Interpreter, obj: &mut Object, name: Expr) -> Option<Object> {
                match *name.expr_type {
                    ExprType::Ident { ident } => {
                        match ident.lexeme.as_str() {
                            $($std_func_name => {
                                Some(BuiltinFunction::new(Rc::new(<$type_ as $std_name>::$name), 1).create_object())
                                }
                            ),*
                            ,
                            _ => None
                        }
                    },
                    ExprType::Call { ref name, args: old_args }  => {
                        let mut args = vec![];
                        for arg in old_args {
                            args.push(arg.accept(interpreteur));
                        }

                        match self.fetch(&mut interpreteur.clone(), &mut obj.clone(), name.clone()) {
                            Some(object) => {
                                match get_impl_if_exist!(Call, object) {
                                    Some(call) => {
                                        Some(call.method(interpreteur, obj, &mut args,  name.file.as_str()))
                                    },
                                    None => {
                                        error!(ErrorType::TypeError, "Expected a function", 0..0, "".to_string());
                                        unreachable!()
                                    }
                                }
                            },
                            None => {
                                panic!("can't get")
                            }
                        }
                    },
                    _ => None
                }
            }
        }
    }
}
