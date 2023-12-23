

mod object;
pub mod compiler;

use std::any::Any;
pub use inkwell::context::Context;
use popper_ast::visitor::{ExprVisitor, StmtVisitor};
