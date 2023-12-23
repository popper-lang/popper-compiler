use inkwell::basic_block::BasicBlock;
use crate::compiler::LLVMCompiler;
use popper_ast::Block;

impl<'ctx> LLVMCompiler<'ctx> {
    pub fn compile_block(&mut self, block: Block) -> BasicBlock {
        let old_basic_block = self.current_basic_block;
        let llvm_block = self.context.append_basic_block(self.module.get_function(self.current_function).unwrap(),
                                                         format!(
                                                             "{}_block_{}",
                                                             self.current_function,
                                                             self.module.get_function(self.current_function).unwrap().get_basic_blocks().len()
                                                         ).as_str());
        self.builder.position_at_end(llvm_block);
        self.current_basic_block = Some(llvm_block);
        for stmt in block.statements {
            self.compile_stmt(stmt);
        }
        self.current_basic_block = old_basic_block;
        llvm_block
    }
}