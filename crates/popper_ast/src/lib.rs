#![allow(unused)]

pub(crate) mod basic_stmt;
pub(crate) mod constant;
pub(crate) mod expr;
pub(crate) mod function;
pub(crate) mod index;
pub(crate) mod memory;
pub(crate) mod op;
pub(crate) mod span;
pub(crate) mod stmt;
pub(crate) mod struct_stmt;
pub(crate) mod types;

#[cfg(feature = "visitor")]
pub mod visitor;

pub use basic_stmt::*;
pub use constant::*;
pub use expr::*;
pub use function::*;
pub use index::*;
pub use memory::*;
pub use op::*;
pub use span::*;
pub use stmt::*;
pub use struct_stmt::*;
pub use types::*;

#[cfg(feature = "serde")]
pub fn get_ast_from_json_file(file: &str) -> Vec<Statement> {
    let file = std::fs::read_to_string(file).unwrap();
    let ast: Vec<Statement> = serde_json::from_str(&file).unwrap();
    ast
}
