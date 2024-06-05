#![allow(dead_code)]
#![allow(unused_macros)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::non_canonical_clone_impl)]
#![allow(private_bounds)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]

pub mod basic_block;
pub mod builder;
pub mod context;
pub mod execution_engine;
pub mod module;
pub mod types;
pub mod util;
pub mod value;
pub mod analysis;
pub mod instruction;
pub mod attribute;
pub mod metadata;
pub mod asm;
mod debug;
