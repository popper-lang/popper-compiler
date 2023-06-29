#![feature(vec_into_raw_parts)]

pub mod ir_sb;
pub mod compiler;
pub mod instr;
pub mod value;
pub mod debug;

#[cfg(test)]
mod tests;


pub fn compile_to_bytecode(ast: Vec<popper_ast::Statement>) -> crate::ir_sb::SbcIr {
    use popper_ast::visitor::StmtVisitor;

    let mut compiler = compiler::SbCompiler::new();
    for stmt in ast {
        let _ = compiler.visit_stmt(stmt);
    }

    compiler.ir


}
