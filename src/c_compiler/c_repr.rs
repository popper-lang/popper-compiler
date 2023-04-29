
use crate::ast::expr::{LiteralType};

use crate::value::Type;


pub fn c_function(name: String, args: Vec<String>, body: String, return_type: String) -> String {
    format!("{} {}({}) {{\n{}}}", return_type, name, args.join(", "), body)
}

pub fn c_call(name: String, args: Vec<String>) -> String {
    format!("{}({});", name, args.join(", "))
}

pub fn c_typed_args(args: Vec<(String, String)>, op: &str) -> String {
    args.iter().map(|(name, type_)| format!("{} {}", type_, name)).collect::<Vec<String>>().join(op)
}

pub fn c_int(value: String) -> String {
    format!("(int) {}", value)
}

pub fn c_asm(value: String) -> String {
    format!("asm(\"{}\")", value)
}

pub fn c_asm_vol(value: String) -> String {
    format!("asm volatile(\"{}\")", value)
}

pub fn literal(literal: &LiteralType) -> String {
    match literal {
        LiteralType::Number (value) => value.to_string(),
        LiteralType::String(value) => format!("\"{}\"", value),
        LiteralType::Bool (value) => value.to_string(),
    }
}

pub fn c_if(cond: String, then: String) -> String {
    format!("if ({}) {{\n{}\n}}", cond, then)
}

pub fn c_if_else(cond: String, then: String, else_: String) -> String {
    format!("if ({}) {} else {}", cond, then, else_)
}

pub fn c_block(body: String) -> String {
    format!("{{\n{}\n}}", body)
}

pub fn c_while(cond: String, body: String) -> String {
    format!("while ({}) {{\n{}\n}}", cond, body)
}

pub fn c_for(name: String, iter: String, body: String) -> String {
    format!("for (int {} = 0; {} < {}; {}++) {{\n{}\n}}", name, name, iter, name, body)
}

pub fn c_init_var(type_: String, name: String, value: String) -> String {
    format!("{} {} = {};", type_, name, value)
}

pub fn c_struct(name: String, fields: String) -> String {
    format!("struct {} {{\n{}\n}};", name, fields)
}

pub fn c_type(type_: Type) -> String {

    match type_ {
        Type::Int => "int".to_string(),
        Type::String => "char*".to_string(),
        Type::Bool => "int".to_string(),
        Type::Struct(name) => name,
        Type::List => panic!("List type is not supported in C"),
        Type::Func => panic!("Func type is not supported in C"),
        Type::Range => panic!("Range type is not supported in C"),
        Type::Type(_) => panic!("Type type is not supported in C"),
        Type::Any => "void*".to_string(),
        Type::None => "void".to_string(),
        Type::Function => "void*".to_string(),
        Type::Instance(_) => panic!("Instance type is not supported in C"),
        Type::Class(_) => panic!("Class type is not supported in C"),
        Type::Namespace => panic!("Namespace type is not supported in C"),
        Type::InstanceStruct => panic!("InstanceStruct type is not supported in C")
    }
}

pub fn c_expr(expr: String) -> String {
    expr
}

pub fn c_const(type_: String, name: String, value: String) -> String {
    format!("const {} {} = {};", type_, name, value)
}

pub fn c_binop(left: String, op: String, right: String) -> String {
    format!("{} {} {}", left, op, right)
}

pub fn c_get(name: String, attr: String) -> String {
    format!("{}.{}", name, attr)
}

pub fn c_assign(name: String, value: String) -> String {
    format!("{} = {};\n", name, value)
}

pub fn c_group(expr: String) -> String {
    format!("({})", expr)
}

pub fn c_cmpop(left: String, op: String, right: String) -> String {
    format!("{} {} {}", left, op, right)
}

pub fn c_index(name: String, index: String) -> String {
    format!("{}[{}]", name, index)
}

pub fn c_iop(name: String, op: String, value: String) -> String {
    format!("{} {} {}", name, op, value)
}

pub fn c_to(type_: String, value: String) -> String {
    format!("({}) {}", type_, value)
}