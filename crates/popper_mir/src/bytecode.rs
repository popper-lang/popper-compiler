// mod consts;

// use consts::*;

// use crate::function::Function;
// use crate::stmt::StmtKind;
// use crate::Program;
// use crate::program::ProgramSection;
// use crate::types::Types;



// #[derive(Debug, Clone)]
// pub struct Bytecode {
//     pub program: Program,
//     pub bytecode: Vec<u8>
// }

// impl Bytecode {
//     pub fn new(program: Program) -> Self {
//         Bytecode {
//             program,
//             bytecode: vec![]
//         }
//     }

//     pub fn compile(&mut self) {
//         for section in self.program.programs.clone().into_iter() {
//             self.compile_section(&section);
//         }
//     }

//     fn compile_section(&mut self, section: &ProgramSection) {
//         match section {
//             ProgramSection::Function(func) => {
//                 self.compile_function(func);
//             },
//             ProgramSection::ExternalFunction(func) => {
//                 self.compile_external_function(func);
//             }
//         }
//     }

//     fn emit_bytecode(&mut self, byte: u8) {
//         self.bytecode.push(byte);
//     }

//     fn emit_bytecodes(&mut self, bytes: Vec<u8>) {
//         self.bytecode.extend(bytes);
//     }

//     fn compile_function(&mut self, func: &Function) {
//         self.emit_bytecode(OP_FUNCTION);
//         self.emit_bytecode(func.name.len() as u8);
//         for byte in func.name.bytes() {
//             self.emit_bytecode(byte);
//         }
//         self.emit_bytecode(func.args.len() as u8);
//         for arg in &func.args {
//             self.emit_bytecodes(arg.to_bytes())
//         }
//         self.emit_bytecodes(func.ret.to_bytes());
//         for stmt in &func.stmts {
//             self.compile_statement(stmt.kind);
//         }
//     }

//     fn compile_statement(&mut self, stmt: StmtKind) {
//         match stmt {
//             StmtKind::LetDecl(l) => {
//                 self.emit_bytecode(OP_LET_DECL);
//                 self.emit_bytecode(l.ident.get_index());
//                 for byte in l.name.bytes() {
//                     self.emit_bytecode(byte);
//                 }
//                 self.emit_bytecodes(l.ty.to_bytes());
//             },
//         }



// }
