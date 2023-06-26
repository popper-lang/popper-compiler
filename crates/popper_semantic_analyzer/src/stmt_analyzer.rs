

use popper_ast::*;

use crate::errors::TypeMismatch;
use popper_flag::{ScopeFlag, Flag, TypeFlag, VariableFlag, Environment, SymbolFlags, ValueFlag};
use crate::expr_analyzer::ExprAnalyzer;
use popper_common::error::Error;
use popper_ast::visitor::ExprVisitor;


pub struct StmtAnalyzer {
    env: Environment,
    current_scope: ScopeFlag,
}

impl StmtAnalyzer {
    pub fn new(env: Environment) -> Self {
        Self { env , current_scope: ScopeFlag::Global }
    }
}

impl visitor::StmtVisitor for StmtAnalyzer {
    type Output = SymbolFlags;
    type Error = Box<dyn Error>;
    fn visit_expr(&mut self, expr: Expression) -> Result<Self::Output, Self::Error> {
        let mut analyzer = ExprAnalyzer::new(self.env.clone());

        analyzer.visit_expr(expr)

    }

    fn visit_let_stmt(&mut self, let_stmt: LetStmt) -> Result<Self::Output, Self::Error> {
        let mut analyzer = ExprAnalyzer::new(self.env.clone());

        let value = analyzer.visit_expr(let_stmt.value)?;


        let variable = VariableFlag::new(
            let_stmt.name.name,
            value.clone(),
            self.current_scope.clone(),
            let_stmt.mutable
        );

        self.env.add_variable(variable);

        Ok(value)
    }

    fn visit_block(&mut self, block: Block) -> Result<Self::Output, Self::Error> {
        let mut analyzer = StmtAnalyzer::new(self.env.clone());

        let mut result = SymbolFlags::new(block.span());

        for stmt in block.statements {
            result = analyzer.visit_stmt(stmt)?;
        }

        Ok(result)
    }

    fn visit_while_stmt(&mut self, while_stmt: While) -> Result<Self::Output, Self::Error> {
        let mut analyzer = ExprAnalyzer::new(self.env.clone());
        let symbol_flag = SymbolFlags::new(while_stmt.span());
        let condition = analyzer.visit_expr(while_stmt.condition.clone())?;

        if !condition.is_boolean() {
            return Err(Box::new(
                TypeMismatch::new(
                    (while_stmt.condition.span(), ValueFlag::Boolean.to_string()),
                    (while_stmt.condition.span(), condition.get_value().unwrap().to_string()),
                )
            ))
        }

        let mut analyzer = StmtAnalyzer::new(self.env.clone());

        let body = analyzer.visit_stmt(*while_stmt.body)?;

        Ok(symbol_flag)
    }

    fn visit_stmt(&mut self, stmt: Statement) -> Result<Self::Output, Self::Error> {
        match stmt {
            Statement::Expression(expr) => self.visit_expr(expr),
            Statement::Let(let_stmt) => self.visit_let_stmt(let_stmt),
            Statement::Block(block) => self.visit_block(block),
            Statement::While(while_stmt) => self.visit_while_stmt(while_stmt),
        }
    }

}

