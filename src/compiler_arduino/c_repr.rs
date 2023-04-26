use crate::ast::expr::LiteralType;

pub fn c_function(name: String, args: Vec<String>, body: String, return_type: String) -> String {
    format!("{} {}({}) {{\n{}}}", return_type, name, args.join(", "), body)
}

pub fn c_call(name: String, args: Vec<String>) -> String {
    format!("{}({})", name, args.join(", "))
}

pub fn c_typed_args(args: Vec<(String, String)>) -> Vec<String> {
    args.iter().map(|(name, type_)| format!("{} {}", type_, name)).collect()
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
    format!("if ({}) {{\n{}\n}} else {{\n{}\n}}", cond, then, else_)
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