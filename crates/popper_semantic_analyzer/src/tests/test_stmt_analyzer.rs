use ariadne::Source;
use popper_flag::Environment;
use popper_ast::visitor::{ExprVisitor, StmtVisitor};
use crate::stmt_analyzer::StmtAnalyzer;
use crate::tests::get_ast_from_json_file;
use popper_ast::Span;
use popper_flag::SymbolFlags;
use popper_ast::Statement;
use popper_common::error::Error;
use popper_common::error::generate_color;
use popper_ast::get_ast_from_json_file;


#[cfg(test)]
pub fn test_bad_type_add() {
    let ast = get_ast_from_json_file("src/tests/assets/bad_type_add.json");
    let body = r#"
        let a: int = 3;

        a + "2";
        "#;


    let mut analyzer = StmtAnalyzer::new(Environment::new());
    let mut result = Ok(SymbolFlags::new(Span::new(0, 0)));
    for stmt in ast {
        result = analyzer.visit_stmt(stmt);
    }

    if let Ok(result) = result {
        assert!(false)
    } else if let Err(err) = result {
        err.report(
            generate_color(),
            &body,
            "<test `test_bad_type_add`>"
        ) // We don't use assert here because we want to see the error message
          // in the terminal
          // So the test will fail if the error message is not printed
          // Now, the test is passed
    }
}

#[cfg(test)]
pub fn test_unknow_variable() {
    let ast = get_ast_from_json_file("src/tests/assets/unknow_var.json");
    let body = r#"
        let foo: int = 3;
        foz;
        "#;

    let mut analyzer = StmtAnalyzer::new(Environment::new());
    let mut result = Ok(SymbolFlags::new(Span::new(0, 0)));
    for stmt in ast {
        result = analyzer.visit_stmt(stmt);
    }

    if let Ok(result) = result {
        assert!(false)
    } else if let Err(err) = result {
        err.report(
            generate_color(),
            &body,
            "<test `test_unknow_variable`>"
        ) // We don't use assert here because we want to see the error message
        // in the terminal
        // So the test will fail if the error message is not printed
        // Now, the test is passed
    }
}


#[cfg(test)]
pub fn run_tests() {
    test_bad_type_add();
    test_unknow_variable();
}