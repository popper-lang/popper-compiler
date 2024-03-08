use popper_ast::*;
use popper_common::name_similarity::find_similar_name;
use popper_error::cantmut::CantMut;
use popper_error::namenotfound::NameNotFound;
use popper_error::notallowed::NotAllowed;
use std::collections::HashMap;

use crate::expr_analyzer::ExprAnalyzer;
use popper_ast::visitor::ExprVisitor;
use popper_error::modulenotfound::ModuleNotFound;
use popper_error::{
    alreadyexist::AlreadyExist, typemismatch::TypeMismatch,
    Error,
};
use popper_flag::{Environment, Flag, ScopeFlag, SymbolFlags, ValueFlag, VariableFlag};

#[derive(Clone)]
pub struct StmtAnalyzer {
    env: Environment,
    current_scope: ScopeFlag,
    is_return: bool,
    return_type: Option<ValueFlag>,
}

impl StmtAnalyzer {
    pub fn new(env: Environment) -> Self {
        Self {
            env,
            current_scope: ScopeFlag::Global,
            return_type: None,
            is_return: false,
        }
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

        let value = if let Some(ref ty) = let_stmt.r#type {
            let r: ValueFlag = ValueFlag::from_ty(ty.clone());
            let mut s = SymbolFlags::new(ty.span);
            s.set_value(r.clone());
            analyzer.set_let_expected_value(s.clone());
            let value = analyzer.visit_expr(let_stmt.value.clone())?;
            let x = value.get_value().unwrap();
            if r != x {
                return Err(Box::new(TypeMismatch::new(
                    (ty.clone().span, ty.type_kind.to_string()),
                    (let_stmt.value.span(), x.to_string()),
                )));
            }

            value
        } else {
            analyzer.visit_expr(let_stmt.value.clone())?
        };

        let variable = VariableFlag::new(
            let_stmt.name.name,
            value.clone(),
            self.current_scope.clone(),
            let_stmt.mutable,
            let_stmt.span,
        );

        self.env.add_variable(variable);

        Ok(value)
    }

    fn visit_assign(&mut self,assign: Assign) -> Result<Self::Output,Self::Error> {

        if !assign.name.is_assignable() {
            return Err(Box::new(NotAllowed::new(
                assign.name.span(),
                "",
                "",
                "expression",
            )));
        }

        let mut analyzer = ExprAnalyzer::new(self.env.clone());
        let name = analyzer.visit_expr(assign.name.clone())?;
        let value = analyzer.visit_expr(assign.value.clone())?;

        if name.get_value().unwrap() != value.get_value().unwrap() {
            return Err(Box::new(TypeMismatch::new(
                (assign.span, name.get_value().unwrap().to_string()),
                (assign.value.span(), value.get_value().unwrap().to_string()),
            )));
        }


        Ok(SymbolFlags::new(assign.span))


    }

    fn visit_block(&mut self, block: Block) -> Result<Self::Output, Self::Error> {
        let mut analyzer = StmtAnalyzer::from(self.clone());

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

        let old_scope = self.current_scope.clone();

        if !condition.is_boolean() {
            return Err(Box::new(TypeMismatch::new(
                (while_stmt.condition.span(), ValueFlag::Boolean.to_string()),
                (
                    while_stmt.condition.span(),
                    condition.get_value().unwrap().to_string(),
                ),
            )));
        }
        self.current_scope = ScopeFlag::Loop;

        let _body = self.visit_stmt(*while_stmt.body)?;

        self.current_scope = old_scope;

        Ok(symbol_flag)
    }

    fn visit_if_stmt(&mut self, if_stmt: If) -> Result<Self::Output, Self::Error> {
        let mut analyzer = ExprAnalyzer::new(self.env.clone());
        let symbol_flag = SymbolFlags::new(if_stmt.span);
        let condition = analyzer.visit_expr(if_stmt.condition.clone())?;

        if !condition.is_boolean() {
            return Err(Box::new(TypeMismatch::new(
                (if_stmt.condition.span(), ValueFlag::Boolean.to_string()),
                (
                    if_stmt.condition.span(),
                    condition.get_value().unwrap().to_string(),
                ),
            )));
        }

        let mut analyzer = StmtAnalyzer::new(self.env.clone());
        analyzer.is_return = self.is_return;
        analyzer.return_type = self.return_type.clone();
        let _body = analyzer.visit_stmt(*if_stmt.body)?;

        Ok(symbol_flag)
    }

    fn visit_if_else_stmt(&mut self, if_else_stmt: IfElse) -> Result<Self::Output, Self::Error> {
        let mut analyzer = ExprAnalyzer::new(self.env.clone());
        let symbol_flag = SymbolFlags::new(if_else_stmt.span);
        let condition = analyzer.visit_expr(if_else_stmt.condition.clone())?;

        if !condition.is_boolean() {
            return Err(Box::new(TypeMismatch::new(
                (
                    if_else_stmt.condition.span(),
                    ValueFlag::Boolean.to_string(),
                ),
                (
                    if_else_stmt.condition.span(),
                    condition.get_value().unwrap().to_string(),
                ),
            )));
        }

        let mut analyzer = StmtAnalyzer::new(self.env.clone());

        let _body = analyzer.visit_stmt(*if_else_stmt.body)?;
        let _else_body = analyzer.visit_stmt(*if_else_stmt.else_body)?;

        Ok(symbol_flag)
    }

    fn visit_break(&mut self, break_stmt: BreakStmt) -> Result<Self::Output, Self::Error> {
        if !self.current_scope.is_loop() {
            return Err(Box::new(NotAllowed::new(
                break_stmt.span,
                "loop",
                "break",
                "keyword"
            )));
        }
        let symbol_flag = SymbolFlags::new(break_stmt.span);
        Ok(symbol_flag)
    }

    fn visit_function(&mut self, function: Function) -> Result<Self::Output, Self::Error> {
        if let Some(f) = self.env.get_variable(function.name.as_str()) {
            let err = AlreadyExist::new(f.span, (function.name, function.span));
            return Err(Box::new(err));
        }
        let mut args = Vec::new();

        for arg in function.arguments.args {
            let mut symbol_flag = SymbolFlags::new(function.span);
            let name = arg.name;
            let val = ExprAnalyzer::new(self.env.clone()).get_type(arg.ty);
            symbol_flag = symbol_flag.add_flag(Flag::Value(val.clone())).clone();
            let variable = VariableFlag::new(
                name,
                symbol_flag.clone(),
                self.current_scope.clone(),
                false,
                function.span,
            );
            self.env.add_variable(variable);
            args.push(val)
        }

        let return_type = {
            let expr_analyser = ExprAnalyzer::new(self.env.clone());
            Box::new(expr_analyser.get_type(function.returntype.clone()))
        };

        self.return_type = Some(*return_type.clone());

        let symbol_flag = SymbolFlags::new(function.span)
            .set_function(args, *return_type.clone(), function.is_var_args)
            .clone();

        let function_flag = VariableFlag::new(
            function.name,
            symbol_flag,
            self.current_scope.clone(),
            false,
            function.span,
        );

        self.env.add_variable(function_flag);

        for stmt in function.body {
            self.visit_stmt(stmt)?;
        }

        if !(self.is_return || self.return_type.is_some() && self.return_type.clone().unwrap().is_same(&ValueFlag::None)) {
            return Err(Box::new(TypeMismatch::new(
                (function.span, return_type.to_string()),
                (function.span, ValueFlag::None.to_string()),
            )));
        }



        Ok(SymbolFlags::new(function.span))
    }

    fn visit_extern(&mut self, extern_stmt: Extern) -> Result<Self::Output, Self::Error> {
        let _analyzer = ExprAnalyzer::new(self.env.clone());

        for sign in &extern_stmt.signs {
            let args: Vec<ValueFlag> = sign
                .arguments
                .args
                .iter()
                .map(|x| {
                    let expr_analyzer = ExprAnalyzer::new(self.env.clone());
                    expr_analyzer.get_type(x.ty.clone())
                })
                .collect();

            let return_type = {
                let expr_analyzer = ExprAnalyzer::new(self.env.clone());
                expr_analyzer.get_type(sign.return_type.clone())
            };

            let var = VariableFlag::new(
                sign.name.clone(),
                SymbolFlags::new(sign.span())
                    .set_function(args, return_type, sign.is_var_args)
                    .clone(),
                ScopeFlag::Global,
                false,
                Default::default(),
            );
            self.env.add_variable(var);
        }

        Ok(SymbolFlags::new(extern_stmt.span()))
    }

    fn visit_return(&mut self, return_expr: Return) -> Result<Self::Output, Self::Error> {
        let mut expr_analyzer = ExprAnalyzer::new(self.env.clone());
        if self.return_type.is_none() {
            return Err(Box::new(NotAllowed::new(return_expr.span, "function", "return", "keyword")));
        }
        let val = return_expr
            .expression
            .map(|x| expr_analyzer.visit_expr(*x).map(|x| x.get_value().unwrap()))
            .unwrap_or(Ok(ValueFlag::None))?;

        if val != self.return_type.clone().unwrap() {
            return Err(Box::new(TypeMismatch::new(
                (
                    return_expr.span,
                    self.return_type.clone().unwrap().to_string(),
                ),
                (return_expr.span, val.to_string()),
            )));
        }

        self.is_return = true;
        Ok(SymbolFlags::new(return_expr.span))
    }

    fn visit_import(&mut self, import: ImportStmt) -> Result<Self::Output, Self::Error> {
        let path = popper_common::ast_path_to_path::ast_path_to_path(import.path.clone());

        if !path.exists() {
            return Err(Box::new(ModuleNotFound::new(
                import.path.to_string(),
                import.path.span(),
            )));
        }

        let mut stmt_analyzer = StmtAnalyzer::new(Environment::new());

        for stmt in import.module_stmts.clone() {
            stmt_analyzer.visit_stmt(stmt)?;
        }

        self.env.extend(&mut stmt_analyzer.env.clone());

        Ok(SymbolFlags::new(import.span()))
    }

    fn visit_stmt(&mut self, stmt: Statement) -> Result<Self::Output, Self::Error> {
        match stmt {
            Statement::Expression(expr) => self.visit_expr_stmt(expr),
            Statement::Let(let_stmt) => self.visit_let_stmt(let_stmt),
            Statement::Block(block) => self.visit_block(block),
            Statement::While(while_stmt) => self.visit_while_stmt(while_stmt),
            Statement::If(if_stmt) => self.visit_if_stmt(if_stmt),
            Statement::IfElse(if_else_stmt) => self.visit_if_else_stmt(if_else_stmt),
            Statement::BreakStmt(b) => self.visit_break(b),
            Statement::Function(fn_stmt) => self.visit_function(fn_stmt),
            Statement::Return(ret_stmt) => self.visit_return(ret_stmt),
            Statement::Import(import) => self.visit_import(import),
            Statement::External(external) => self.visit_external(external),
            Statement::For(for_stmt) => self.visit_for_stmt(for_stmt),
            Statement::Struct(struct_stmt) => self.visit_struct_stmt(struct_stmt),
            Statement::Extern(ext) => self.visit_extern(ext),
            Statement::Assign(a) => self.visit_assign(a),
        }
    }

    fn visit_external(&mut self, external: External) -> Result<Self::Output, Self::Error> {
        let _analyzer = ExprAnalyzer::new(self.env.clone());

        for sign in &external.signs {
            let args: Vec<ValueFlag> = sign
                .arguments
                .args
                .iter()
                .map(|x| {
                    let expr_analyzer = ExprAnalyzer::new(self.env.clone());
                    expr_analyzer.get_type(x.ty.clone())
                })
                .collect();

            let return_type = {
                let expr_analyzer = ExprAnalyzer::new(self.env.clone());
                expr_analyzer.get_type(sign.return_type.clone())
            };

            let var = VariableFlag::new(
                sign.name.clone(),
                SymbolFlags::new(sign.span())
                    .set_function(args, return_type, sign.is_var_args)
                    .clone(),
                ScopeFlag::Global,
                false,
                Default::default(),
            );
            self.env.add_variable(var);
        }

        Ok(SymbolFlags::new(external.span()))
    }

    fn visit_for_stmt(&mut self, _for_stmt: ForStmt) -> Result<Self::Output, Self::Error> {
        todo!()
    }

    fn visit_struct_stmt(&mut self, struct_stmt: StructStmt) -> Result<Self::Output, Self::Error> {
        if self.env.exist(struct_stmt.name.clone()) {
            let s = self.env.get_variable(struct_stmt.name.as_str()).unwrap();
            let err = AlreadyExist::new(s.span, (struct_stmt.name, struct_stmt.span));
            return Err(Box::new(err));
        }
        let field: HashMap<_, _> = struct_stmt
            .fields
            .iter()
            .map(|field| {
                let expr_analyzer = ExprAnalyzer::new(self.env.clone());
                let ty = expr_analyzer.get_type(field.ty.clone());
                (field.name.clone(), ty)
            })
            .collect();

        let symbol_flag = SymbolFlags::new(struct_stmt.span).set_struct(field).clone();
        let variable = VariableFlag::new(
            struct_stmt.name.clone(),
            symbol_flag,
            self.current_scope.clone(),
            false,
            struct_stmt.span,
        );

        self.env.add_variable(variable);

        Ok(SymbolFlags::new(struct_stmt.span))
    }
}
