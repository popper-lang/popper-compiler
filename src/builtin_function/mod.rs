pub mod io;
pub mod cmp;
pub mod list_util;

use crate::interpreter::STD_LIB_PATH;
use std::path::Path;


pub fn panic_if_is_outside_std(path: &str, function_name: &str) {
    let std_path = Path::new(STD_LIB_PATH);
    let path = Path::new(path);

    if ! path.starts_with(std_path) {
        panic!("You can't import the {} function from outside the standard library.", function_name);
    }
}

#[macro_export]
macro_rules! value_to_rs_value {
    ($value:ident) => {
        match $value.value {
            Value::Int(val) => val,
            Value::String(val) => val,
            Value::Bool(val) => val,
            Value::None => (),
            _ => panic!("Unexpected return type for function {}", stringify!($name)),
        }
    };
}

#[macro_export]
macro_rules! rs_type_to_type {
    ($type_:ty) => {
        match <$type_ as Default>::default().into::<Value>() {
            Value::Int(_) => Type::Int,
            Value::String(_) => Type::String,
            Value::Bool(_) => Type::Bool,
            Value::None => Type::None,
            _ => panic!("Unexpected return type for function {}", stringify!($name)),
        }
    };
}

#[macro_export]
macro_rules! type_to_rs_type {
    ($type_:expr) => {
        match $type_ {
            Type::Int => i32,
            Type::String => String,
            Type::Bool => bool,
            Type::None => (),
            _ => panic!("Not implemented yet"),
        }
    };
}

#[macro_export]
macro_rules! create {
    ($funcname:ident) => {
        impl $funcname {
            pub fn create() -> Object {
                 Object {
                    type_: Type::Function,
                    implementations: vec![Implementation::Call(Rc::new($funcname))],
                    value: Value::Function,
                    tags: std::default::Default::default()
                 }
            }
        }
    };
}


#[macro_export]
macro_rules! function_to_rs_fn {
    ($func:expr) => {
        match *$func.declaration.stmt_type {
            $crate::ast::stmt::StmtType::Function { name, args, body } => {
                let name = name.lexeme;

            }
        }
    };
}

#[macro_export]
macro_rules! call_function_with_vec {
    ($func:expr, $($vec:expr),* ) => {
        $func(
            $($vec.into()),*
        );
    };
}

#[macro_export]
macro_rules! build_function {
    ($name:ident, $($args_name:ident : $ty:expr),*) => {
        fn $name($($args_name : $crate::type_to_rs_type!($ty)),*) {

        }
    }
}
#[macro_export]
macro_rules! call_method_with_vec {
    ($func:expr, $this:tt, $($vec:expr),* ) => {
        $func(
            $this.into_mut().unwrap(),
            $($vec.into()),*
        );
    };
}

#[macro_export]
macro_rules! define_function {
    ($name:ident($($arg:ident : $ty:ty),*) $body:block, function_name = $function_name:expr) => {
        pub struct $name;

        impl $name {
            pub fn $name ($($arg : $ty),*) -> Object $body
        }

        impl Callable for $name {
            fn call(&self, _interpreter: &mut Interpreter, args: &mut Vec<Object>, _file: &str) -> Object {
                let mut arg_iter = std::iter::repeat(()).zip(args.iter_mut());
                let mut next_arg = arg_iter.next();

                $(let $arg = loop {
                    let arg = match next_arg {
                        Some(((), arg)) => arg,
                        None => panic!("Missing argument for function {}", stringify!($name)),
                    };
                    let arg_obj = arg.clone();
                    next_arg = arg_iter.next();
                    match arg_obj.into() {
                        Some(val) => break val,
                        None => {},
                    }
                };)*
                if let Some(((), _)) = next_arg {
                    panic!("Too many arguments for function {}", stringify!($name));
                }

                // Call the function and convert the return value to an Object
                let result = call_function_with_vec!($name::$name , $($arg),*);
                result
            }



        }
        create!($name);
    };
}



