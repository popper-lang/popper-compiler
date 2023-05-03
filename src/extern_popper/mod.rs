#![allow(unused)]
use crate::value::Type;

#[derive(Clone, Copy, PartialEq, Debug)]
enum RustType {
    RustI32,
    RustF32,
    RustString,
    RustBool
}



fn popper_type_to_rust_type(popper_type: Type) -> RustType {
    match popper_type {
        Type::Int => RustType::RustI32,
        Type::String => RustType::RustString,
        _ => panic!("not yet implemented")
    }
}

fn rust_type_to_popper_type(rust_type: RustType) -> Type {
    match rust_type {
        RustType::RustI32 => Type::Int,
        RustType::RustString => Type::String,
        _ => panic!("not yet implemented")
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_popper_type_to_rust_type() {
        assert_eq!(popper_type_to_rust_type(Type::Int), RustType::RustI32);

    }


}