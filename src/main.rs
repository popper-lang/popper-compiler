use popper_ast::ast::{LangNodeId, LineInfo};
use popper_ast::file::SourceFileInfo;
use popper_ast::layer::Ast;
use popper_context::Context;
use popper_error_core::{Diagnostics, Error, ErrorInfo};
use popper_parser::error::ParserError;
use popper_parser::Parser;
use popper_semantic_analyzer::SemanticAnalyzerLayer;
use std::env;

fn main() {
    let mut context = Context::new();
    let file_path = env::args()
        .nth(1)
        .unwrap_or_else(|| "hello.pop".to_string());
    let source_file_info = SourceFileInfo::from_file(&file_path).unwrap();
    let file = context
        .file_table_mut()
        .insert(source_file_info.clone(), None);
    let mut parser = Parser::from_source_file(&source_file_info);
    match parser.parse() {
        Ok(ast) => {
            let res = ast.apply_layer(&mut SemanticAnalyzerLayer);
            match res {
                Ok(hir) => {
                    let res = hir.apply_layer(&mut popper_cfte::CFTELayer);
                    let ctx = popper_codegen_llvm::CodegenCtxLLVM::new();
                    let mut codegen = popper_codegen_llvm::PopperCodegenLLVM::new(&ctx);
                    let _ = res.apply_layer(&mut codegen);
                    codegen.state().module.print_to_stderr();

                    codegen.state().module.verify().unwrap();
                    codegen.execute_main_function();
                }
                Err(e) => {
                    let line_info = LineInfo::from_span(e.span(), &source_file_info);
                    let err = Error::new(ErrorInfo::new(line_info, file), e);

                    err.print(context).unwrap();
                }
            }
        }
        Err(ParserError::UnexpectedToken(e)) => {
            let line_info = LineInfo::from_span(e.span, &source_file_info);
            let err = Error::new(ErrorInfo::new(line_info, file), e);

            err.print(context).unwrap();
        }
        Err(e) => {
            println!("Failed to parse statement : {:?}", e);
        }
    }
}
