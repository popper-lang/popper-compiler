use std::collections::HashMap;
use popper_ast::*;

use crate::errors::{AlreadyExist, TypeMismatch};
use popper_flag::{ScopeFlag, VariableFlag, Environment, SymbolFlags, ValueFlag, Flag};
use crate::expr_analyzer::ExprAnalyzer;
use popper_common::error::Error;
use popper_ast::visitor::ExprVisitor;

#[derive(Clone)]
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
    fn visit_expr_stmt(&mut self, expr: Expression) -> Result<Self::Output, Self::Error> {
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
            let_stmt.mutable,
            let_stmt.span
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

        let _body = analyzer.visit_stmt(*while_stmt.body)?;

        Ok(symbol_flag)
    }

    fn visit_if_stmt(&mut self, if_stmt: If) -> Result<Self::Output, Self::Error> {
        let mut analyzer = ExprAnalyzer::new(self.env.clone());
        let symbol_flag = SymbolFlags::new(if_stmt.span);
        let condition = analyzer.visit_expr(if_stmt.condition.clone())?;

        if !condition.is_boolean() {
            return Err(Box::new(
                TypeMismatch::new(
                    (if_stmt.condition.span(), ValueFlag::Boolean.to_string()),
                    (if_stmt.condition.span(), condition.get_value().unwrap().to_string()),
                )
            ))
        }

        let mut analyzer = StmtAnalyzer::new(self.env.clone());

        let _body = analyzer.visit_stmt(*if_stmt.body)?;

        Ok(symbol_flag)
    }

    fn visit_if_else_stmt(&mut self, if_else_stmt: IfElse) -> Result<Self::Output, Self::Error> {

        let mut analyzer = ExprAnalyzer::new(self.env.clone());
        let symbol_flag = SymbolFlags::new(if_else_stmt.span);
        let condition = analyzer.visit_expr(if_else_stmt.condition.clone())?;

        if !condition.is_boolean() {
            return Err(Box::new(
                TypeMismatch::new(
                    (if_else_stmt.condition.span(), ValueFlag::Boolean.to_string()),
                    (if_else_stmt.condition.span(), condition.get_value().unwrap().to_string()),
                )
            ))
        }

        let mut analyzer = StmtAnalyzer::new(self.env.clone());

        let _body = analyzer.visit_stmt(*if_else_stmt.body)?;
        let _else_body = analyzer.visit_stmt(*if_else_stmt.else_body)?;

        Ok(symbol_flag)
    }

    fn visit_function(&mut self, function: Function) -> Result<Self::Output, Self::Error> {
        if let Some(f) = self.env.get_variable(function.name.as_str()) {
            let err = AlreadyExist::new(f.span, (function.name, function.span));
            return Err(Box::new(err));
        }
        let args: HashMap<String, ValueFlag>  = function.arguments.args.iter().map(|arg| {
            let expr_analyser = ExprAnalyzer::new(self.env.clone());
            (arg.name.clone(), expr_analyser.get_type(arg.ty.clone()))
        }).collect();

        let mut stmt_analyser =  StmtAnalyzer::from(self.clone());

        stmt_analyser.current_scope = ScopeFlag::Function;

        for (name, val) in args.clone() {
            let mut symbol_flag = SymbolFlags::new(function.span);
            symbol_flag = symbol_flag.add_flag(Flag::Value(val.clone())).clone();
            let variable = VariableFlag::new(
                name,
                symbol_flag.clone(),
                stmt_analyser.current_scope.clone(),
                false,
                function.span
            );

            stmt_analyser.env.add_variable(variable);
        }

        for stmt in function.body {
            stmt_analyser.visit_stmt(stmt)?;
        }

        let return_type = {
            let expr_analyser = ExprAnalyzer::new(self.env.clone());
            Box::new(expr_analyser.get_type(function.returntype.clone()))
        };

        let symbol_flag = SymbolFlags::new(function.span)
            .set_function(args
                              .values()
                              .cloned()
                              .collect(), *return_type)
            .clone()
        ;


        let function_flag = VariableFlag::new(
            function.name,
            symbol_flag,
            self.current_scope.clone(),
            false,
            function.span
        );

        self.env.add_variable(function_flag);

        Ok(SymbolFlags::new(function.span))
    }

    fn visit_stmt(&mut self, stmt: Statement) -> Result<Self::Output, Self::Error> {
        match stmt {
            Statement::Expression(expr) => self.visit_expr_stmt(expr),
            Statement::Let(let_stmt) => self.visit_let_stmt(let_stmt),
            Statement::Block(block) => self.visit_block(block),
            Statement::While(while_stmt) => self.visit_while_stmt(while_stmt),
            Statement::If(if_stmt) => self.visit_if_stmt(if_stmt),
            Statement::IfElse(if_else_stmt) => self.visit_if_else_stmt(if_else_stmt),
            Statement::Function(fn_stmt) => self.visit_function(fn_stmt)
        }
    }

}

