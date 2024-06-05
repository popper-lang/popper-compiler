#![allow(clippy::borrowed_box)]

mod flag;
mod scope_flag;
mod symbol_table;
mod value_flag;
mod variable_flag;

pub use flag::Flag;
pub use scope_flag::ScopeFlag;
pub use symbol_table::SymbolFlags;
pub use value_flag::ValueFlag;
pub use variable_flag::Environment;
pub use variable_flag::VariableFlag;
