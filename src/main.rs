use popper_ast::ast::{LangNodeId, LineInfo};
use popper_ast::file::SourceFileInfo;
use popper_ast::layer::Ast;
use popper_context::Context;
use popper_error_core::{Diagnostics, Error, ErrorInfo};
use popper_parser::error::ParserError;
use popper_parser::Parser;
use popper_semantic_analyzer::SemanticAnalyzerLayer;

fn main() {
    let mut context = Context::new();
    let source_file_info = SourceFileInfo::from_file("hello.pop").unwrap();
    let file = context
        .file_table_mut()
        .insert(source_file_info.clone(), None);
    let mut parser = Parser::from_source_file(source_file_info);
    match parser.parse() {
        Ok(ast) => {
            let res = ast.apply_layer(&mut SemanticAnalyzerLayer);
            match res {
                Ok(_) => {
                    println!("Check successfully!");
                    // Here you can do something with the AST, like printing it
                    // println!("{:#?}", ast);
                }
                Err(e) => {
                    let line_info = LineInfo::from_span(e.span(), 1);
                    let err = Error::new(ErrorInfo::new(line_info, file), e);

                    err.print(context).unwrap();
                }
            }
        }
        Err(ParserError::UnexpectedToken(e)) => {
            let line_info = LineInfo::from_span(e.span, 1);
            let err = Error::new(ErrorInfo::new(line_info, file), e);

            err.print(context).unwrap();
        }
        Err(e) => {
            println!("Failed to parse statement : {:?}", e);
        }
    }
}
