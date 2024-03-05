use crate::mir_ast::{
    Alloc, Argument, Arguments, Body, BodyFn, CJump, Function as MirFunction, Ir, Jump, Label, MirString, Return as MirReturn, Store
};
use crate::mir_compiler::MirCompiler;
use popper_ast::visitor::{ExprVisitor, StmtVisitor};
use popper_ast::{
    Block, Expression, External, ForStmt, Function, If, IfElse, ImportStmt, LetStmt, Return,
    Statement, StructStmt, While,
};

impl StmtVisitor for MirCompiler {
    type Output = ();
    type Error = ();

    fn visit_expr_stmt(&mut self, expr: Expression) -> Result<Self::Output, Self::Error> {
        self.visit_expr(expr)?;
        Ok(())
    }

    fn visit_let_stmt(&mut self, let_stmt: LetStmt) -> Result<Self::Output, Self::Error> {
        if self.current_label.is_none() {
            return Err(());
        }
        let name = let_stmt.name.name.clone();
        self.let_name = Some(name.clone());
        let expr = self.visit_expr(let_stmt.value)?;
        self.let_name = None;
        let ty = expr.get_type();
        self.local.insert(name.clone(), ty.clone());
        if !self.is_let_name_used {
            let current_fn = self.current_label.as_mut().unwrap();
            current_fn.push(BodyFn::Alloc(Alloc::new(name.clone(), ty)));

            current_fn.push(BodyFn::Store(Store::new(name, expr)))
        } else {
            self.is_let_name_used = false
        }

        Ok(())
    }

    fn visit_stmt(&mut self, stmt: Statement) -> Result<Self::Output, Self::Error> {
        match stmt {
            Statement::Expression(expr) => self.visit_expr_stmt(expr),
            Statement::Let(let_stmt) => self.visit_let_stmt(let_stmt),
            Statement::Block(block) => self.visit_block(block),
            Statement::While(while_stmt) => self.visit_while_stmt(while_stmt),
            Statement::If(if_stmt) => self.visit_if_stmt(if_stmt),
            Statement::IfElse(if_else_stmt) => self.visit_if_else_stmt(if_else_stmt),
            Statement::Function(function) => self.visit_function(function),
            Statement::Return(return_expr) => self.visit_return(return_expr),
            Statement::Import(import) => self.visit_import(import),
            Statement::External(external) => self.visit_external(external),
            Statement::For(for_stmt) => self.visit_for_stmt(for_stmt),
            Statement::Struct(struct_stmt) => self.visit_struct_stmt(struct_stmt),
            Statement::Extern(enum_stmt) => self.visit_extern(enum_stmt),
            Statement::BreakStmt(break_stmt) => self.visit_break(break_stmt),
        }
    }

    fn visit_block(&mut self, block: Block) -> Result<Self::Output, Self::Error> {
        for stmt in block.statements {
            self.visit_stmt(stmt)?;
        }
        Ok(())
    }

    fn visit_while_stmt(&mut self, while_stmt: While) -> Result<Self::Output, Self::Error> {
        let labels = self.new_labels(3);
        let cond_label = labels[0].clone();
        let loop_label = labels[1].clone();
        let end_label = labels[2].clone();

        self.push_on_label(BodyFn::Jump(
            Jump::new(cond_label.name.clone())
        ));
        self.add_current_label();

        self.set_current_label(cond_label.clone());

        let condition = self.visit_expr(while_stmt.condition)?;

        self.push_on_label(BodyFn::CJump(
            CJump::new(condition, loop_label.name.clone(), end_label.name.clone())
        ));

        self.add_current_label();

        self.set_current_label(loop_label.clone());
        self.exit_loop = Some(end_label.clone());
        self.loop_depth += 1;
        self.visit_stmt(*while_stmt.body)?;

        self.push_on_label(BodyFn::Jump(
            Jump::new(cond_label.name.clone())
        ));

        self.exit_loop = None;
        self.loop_depth -= 1;

        self.add_current_label();
        self.set_current_label(end_label.clone());

        Ok(())

    }

    fn visit_break(&mut self, _: popper_ast::BreakStmt) -> Result<Self::Output, Self::Error> {
        let current_fn = self.current_label.as_mut().unwrap();
        current_fn.push(BodyFn::Jump(
            Jump::new(self.exit_loop.as_ref().unwrap().name.clone())
        ));
        self.break_depth = self.loop_depth;
        Ok(())

    }

    fn visit_if_stmt(&mut self, if_stmt: If) -> Result<Self::Output, Self::Error> {
        let cond = self.visit_expr(if_stmt.condition)?;
        let labels = self.new_labels(2);
        let then_label = labels[0].clone();
        let else_label = labels[1].clone();
        self.push_on_label(BodyFn::CJump(
            CJump::new(cond, then_label.name.clone(), else_label.name.clone())
        ));
        self.add_current_label();

        self.set_current_label(then_label.clone());

        self.visit_stmt(*if_stmt.body)?;



        self.push_on_label(BodyFn::Jump(
            Jump::new(else_label.clone().name)
        ));
        self.add_current_label();
        self.set_current_label(else_label.clone());
        Ok(())

    }

    fn visit_if_else_stmt(&mut self, if_else_stmt: IfElse) -> Result<Self::Output, Self::Error> {
        let cond = self.visit_expr(if_else_stmt.condition)?;
        let labels = self.new_labels(3);
        let then_label = labels[0].clone();
        let else_label = labels[1].clone();
        let end_label = labels[2].clone();
        self.push_on_label(BodyFn::CJump(
            CJump::new(cond, then_label.name.clone(), else_label.name.clone())
        ));
        self.add_current_label();
        self.set_current_label(then_label.clone());
        self.visit_stmt(*if_else_stmt.body)?;
        self.push_on_label(BodyFn::Jump(
            Jump::new(end_label.clone().name)
        ));
        self.add_current_label();
        self.set_current_label(else_label.clone());
        self.visit_stmt(*if_else_stmt.else_body)?;
        self.push_on_label(BodyFn::Jump(
            Jump::new(end_label.clone().name)
        ));
        self.add_current_label();
        self.set_current_label(end_label.clone());
        Ok(())
    }

    fn visit_function(&mut self, function: Function) -> Result<Self::Output, Self::Error> {
        let name = function.name.clone();
        let args = function
            .arguments
            .args
            .iter()
            .map(|arg| (arg.name.clone(), self.compile_type(arg.ty.clone())))
            .map(|x| Argument::new(x.0, x.1))
            .collect::<Vec<Argument>>();

        args.iter().for_each(|arg| {
            self.local.insert(arg.name.clone(), arg.ty.clone());
        });
        let ret = self.compile_type(function.returntype);
        let label = Label::new("entry".to_string(), vec![]);
        let func = MirFunction::new(
            name.clone(),
            Arguments::new(args),
            ret.clone(),
            function.is_var_args,
            Body::new(vec![]),
        );
        self.global.insert(name.clone(), ret.clone());

        self.current_fn = Some(func);
        self.current_label = Some(label);
        for stmt in function.body {
            self.visit_stmt(stmt)?;
        }

        self.current_fn.as_mut().unwrap().body.body.push(self.current_label.as_ref().unwrap().clone());



        self.local.clear();



        self.current_label = None;

        self.ir.push(Ir::Function(self.current_fn.as_ref().unwrap().clone()));
        self.current_fn = None;

        Ok(())
    }

    fn visit_return(&mut self, return_expr: Return) -> Result<Self::Output, Self::Error> {
        if self.current_label.is_none() {
            return Err(());
        }
        self.is_returned = true;
        if let Some(expr) = return_expr.expression {
            let expr = self.visit_expr(*expr)?;
            self.current_label
                .as_mut()
                .unwrap()
                .push(BodyFn::Return(MirReturn::new(Some(expr))));
        } else {
            self.current_label
                .as_mut()
                .unwrap()
                .push(BodyFn::Return(MirReturn::new(None)));
        }

        Ok(())
    }

    fn visit_import(&mut self, import: ImportStmt) -> Result<Self::Output, Self::Error> {
        let path = import.path;

        // convert stmt path to path

        let path = path
            .segments
            .iter()
            .map(|segment| segment.name.clone())
            .collect::<Vec<String>>()
            .join("/");

        let path = format!("{}.pop", path);

        let path = std::path::Path::new(path.as_str());

        let mut compiler =
            MirCompiler::new(import.module_stmts, path.to_str().unwrap().to_string());

        compiler.compile();

        let module = compiler.get_module();
        self.ir.push(Ir::LoadModule(module));

        self.global.extend(compiler.global.clone());

        Ok(())
    }

    fn visit_external(&mut self, external: External) -> Result<Self::Output, Self::Error> {
        let file = external.file;
        self.ir.push(Ir::LoadExternal(MirString::new(file)));

        for fn_sign in external.signs {
            self.compile_fn_sign(fn_sign);
        }

        Ok(())
    }

    fn visit_extern(
        &mut self,
        extern_stmt: popper_ast::Extern,
    ) -> Result<Self::Output, Self::Error> {
        for fn_sign in extern_stmt.signs {
            self.compile_fn_sign(fn_sign);
        }

        Ok(())
    }

    fn visit_for_stmt(&mut self, _for_stmt: ForStmt) -> Result<Self::Output, Self::Error> {
        todo!()
    }

    fn visit_struct_stmt(&mut self, _struct_stmt: StructStmt) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}
