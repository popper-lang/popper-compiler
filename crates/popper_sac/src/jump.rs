// use popper_asm::builder::Assembly;
// use popper_sbc::instr::Instruction;
// use crate::bytecode_compiler::Compiler;
// use crate::label::Label;
//
// pub struct Jumper {
//     name: String,
//     op: Assembly,
//     instrs: Vec<Instruction>
// }
//
// impl Jumper {
//     pub fn create_name(compiler: Compiler) -> String {
//         format!("label_{}", compiler.labels.len())
//     }
//
//     pub fn new(name: String, op: Assembly, instrs: Vec<Instruction>) -> Self {
//         Self {
//             name,
//             op,
//             instrs
//         }
//     }
//
//     pub fn build_assembler(&self, compiler: &mut Compiler, is_included: bool) {
//         if compiler.labels.iter().filter(|x| x.label == self.name ).count()  != 0 {
//             compiler.builder.push(self.op.clone());
//             return;
//         }
//
//         let program = {
//             let mut compiler = Compiler::new(self.instrs.clone());
//             compiler.set_labels(compiler.labels.clone());
//             compiler.set_stack(compiler.stack.clone());
//             compiler.compile();
//             if is_included {
//                 compiler.builder.push(self.op.clone());
//             }
//             compiler.build().0
//         };
//         compiler.builder.push(self.op.clone());
//
//         compiler.labels.push(
//             Label::new(self.name.clone(), program)
//         )
//     }
// }