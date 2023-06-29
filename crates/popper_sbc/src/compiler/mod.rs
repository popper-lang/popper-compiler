mod expr_compiler;
mod stmt_compiler;

use crate::ir_sb::SbcIr;

#[derive(Clone)]
pub struct SbCompiler {
    pub ir: SbcIr
}

impl SbCompiler {
    pub fn new() -> Self {
        Self {
            ir: SbcIr::new()
        }
    }
}
