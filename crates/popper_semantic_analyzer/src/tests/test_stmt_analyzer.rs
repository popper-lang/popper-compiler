use popper_flag::Environment;
use popper_ast::visitor::StmtVisitor;
use crate::stmt_analyzer::StmtAnalyzer;
use popper_ast::get_ast_from_json_file;
use popper_ast::Span;
use popper_flag::SymbolFlags;
use popper_common::error::generate_color;


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

    if let Ok(_) = result {
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

    if let Ok(_) = result {
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
pub fn test_already_exist() {
    let ast = get_ast_from_json_file("src/tests/assets/already_exist_fn.json");
    let body = r#"
        func hello(name: string): int {
            4 + 5;
        }

        func hello(name: string): int {
            4 + 5;
        }
        "#;

    let mut analyzer = StmtAnalyzer::new(Environment::new());
    let mut result = Ok(SymbolFlags::new(Span::new(0, 0)));
    for stmt in ast {
        result = analyzer.visit_stmt(stmt);
    }

    if let Ok(_) = result {
        assert!(false)
    } else if let Err(err) = result {
        err.report(
            generate_color(),
            &body,
            "<test `test_already_exist`>"
        ) // We don't use assert here because we want to see the error message
        // in the terminal
        // So the test will fail if the error message is not printed
        // Now, the test is passed
    }
}

#[cfg(test)]
pub fn test_function() {
    let ast = get_ast_from_json_file("src/tests/assets/test_function.json");
    let body = r#"
        func hello(name: string): int {
            5 + nam;
            4 + name;
        }
        "#;

    let mut analyzer = StmtAnalyzer::new(Environment::new());
    let mut result = Ok(SymbolFlags::new(Span::new(0, 0)));
    for stmt in ast {
        result = analyzer.visit_stmt(stmt);
    }

    if let Err(err) = result {
        err.report(
            generate_color(),
            body,
            "<test `test_function`>"
        ) // We don't use assert here because we want to see the error message
        // in the terminal
        // So the test will fail if the error message is not printed
        // Now, the test is passed
    }
}




#[cfg(test)]
pub fn test_call_mismatch_type() {
    let ast = get_ast_from_json_file("src/tests/assets/call_mismatch_type.json");
    let body = r#"
        func add(a: int, b: int): unit {
            return a + b;
        }

        add("hello", 4);
        "#;

    let mut analyzer = StmtAnalyzer::new(Environment::new());
    let mut result = Ok(SymbolFlags::new(Span::new(0, 0)));
    for stmt in ast {
        result = analyzer.visit_stmt(stmt);
    }

    if let Err(err) = result {
        err.report(
            generate_color(),
            body,
            "<test `test_call_mismatch`>"
        ) // We don't use assert here because we want to see the error message
        // in the terminal
        // So the test will fail if the error message is not printed
        // Now, the test is passed
    }
}

#[cfg(test)]
pub fn test_call_diff_length_argument() {
    let ast = get_ast_from_json_file("src/tests/assets/call_diff_length_argument.json");
    let body = r#"
        func add(a: int, b: int): unit {
            return a + b;
        }

        add(2, 4, 1);
        "#;
    let mut analyzer = StmtAnalyzer::new(Environment::new());
    let mut result = Ok(SymbolFlags::new(Span::new(0, 0)));
    for stmt in ast {
        result = analyzer.visit_stmt(stmt);
    }

    if let Err(err) = result {
        err.report(
            generate_color(),
            body,
            "<test `test_diff_length_argument`>"
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
    test_already_exist();
    test_function();
    test_call_mismatch_type();
    test_call_diff_length_argument();
}