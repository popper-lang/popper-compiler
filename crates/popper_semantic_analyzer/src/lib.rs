

mod expr_analyzer;
mod stmt_analyzer;


#[cfg(test)]
mod tests;

pub fn analyze(stmts: Vec<popper_ast::Statement>) -> Vec<Result<(), Box<dyn popper_error::Error>>> {
    use popper_flag::Environment;
    use popper_ast::visitor::StmtVisitor;
    use popper_error::Error;
    use popper_flag::SymbolFlags;

    let mut stmt_analyzer = stmt_analyzer::StmtAnalyzer::new(Environment::new());
    stmt_analyzer.init_builtins();
    let mut res: Vec<Result<SymbolFlags, Box<dyn Error>>> = Vec::new();
    for stmt in stmts {
        res.push(stmt_analyzer.visit_stmt(stmt));
    }

    res.into_iter().map(|x| {
        match x {
            Ok(_) => Ok(()),
            Err(err) => Err(err)
        }
    }).collect()
}


