use popper_ast::{Block, Expression, External, ForStmt, Function, If, IfElse, ImportStmt, LetStmt, Return, Statement, StructStmt, While};
use popper_ast::visitor::{ExprVisitor, StmtVisitor};
use crate::mir_ast::{Alloc, BodyFn, Ir, MirString, Store, Return as MirReturn, Value, Const, Body, Function as MirFunction, Type, Arguments, Argument};
use crate::mir_compiler::MirCompiler;

impl StmtVisitor for MirCompiler {
    type Output = ();
    type Error = ();

    fn visit_expr_stmt(&mut self, expr: Expression) -> Result<Self::Output, Self::Error> {
        self.visit_expr(expr)?;
        Ok(())
    }

    fn visit_let_stmt(&mut self, let_stmt: LetStmt) -> Result<Self::Output, Self::Error> {
        if self.current_fn.is_none() {
            return Err(());
        }
        let expr = self.visit_expr(let_stmt.value)?;
        let ty = self.compile_type(let_stmt.r#type.unwrap());
        self.env.insert(let_stmt.name.name.clone(), ty.clone());
        let body = self.current_fn.as_mut().unwrap();
        body.push(
            BodyFn::Alloc(
                Alloc::new(let_stmt.name.name.clone(), ty)
            )
        );

        body.push(
            BodyFn::Store(
                Store::new(
                    let_stmt.name.name,
                    expr
                )
            )
        );


        Ok(())


    }

    fn visit_stmt(&mut self, stmt: Statement) -> Result<Self::Output, Self::Error> {
        match stmt {
            Statement::Expression(expr) => {
                self.visit_expr_stmt(expr)
            },
            Statement::Let(let_stmt) => {
                self.visit_let_stmt(let_stmt)
            },
            Statement::Block(block) => {
                self.visit_block(block)
            },
            Statement::While(while_stmt) => {
                self.visit_while_stmt(while_stmt)
            },
            Statement::If(if_stmt) => {
                self.visit_if_stmt(if_stmt)
            },
            Statement::IfElse(if_else_stmt) => {
                self.visit_if_else_stmt(if_else_stmt)
            },
            Statement::Function(function) => {
                self.visit_function(function)
            },
            Statement::Return(return_expr) => {
                self.visit_return(return_expr)
            },
            Statement::Import(import) => {
                self.visit_import(import)
            },
            Statement::External(external) => {
                self.visit_external(external)
            },
            Statement::For(for_stmt) => {
                self.visit_for_stmt(for_stmt)
            },
            Statement::Struct(struct_stmt) => {
                self.visit_struct_stmt(struct_stmt)
            },
        }
    }

    fn visit_block(&mut self, block: Block) -> Result<Self::Output, Self::Error> {
        todo!()
    }

    fn visit_while_stmt(&mut self, while_stmt: While) -> Result<Self::Output, Self::Error> {
        todo!()
    }

    fn visit_if_stmt(&mut self, if_stmt: If) -> Result<Self::Output, Self::Error> {
        todo!()
    }

    fn visit_if_else_stmt(&mut self, if_else_stmt: IfElse) -> Result<Self::Output, Self::Error> {
        todo!()
    }

    fn visit_function(&mut self, function: Function) -> Result<Self::Output, Self::Error> {
        let name = function.name.clone();
        let args = function
            .arguments
            .args
            .iter()
            .map(|arg|
                (arg.name.clone(),
                 self.compile_type(
                     arg.ty.clone()
                 )
                )
            )
            .map(|x| Argument::new(x.0, x.1))
            .collect::<Vec<Argument>>();

        args.iter().for_each(|arg| {
            self.env.insert(arg.name.clone(), arg.ty.clone());
        });
        let ret = self.compile_type(function.returntype);

        let body = Body::new(Vec::new());
        self.current_fn = Some(body);
        for stmt in function.body {
            self.visit_stmt(stmt)?;
        }

        self.env.clear();
        self.env.insert(name.clone(), ret.clone());
        let function = MirFunction::new(name, Arguments::new(args), ret, self.current_fn.clone().unwrap());

        self.current_fn = None;

        self.ir.push(
            Ir::Function(
                function
            )
        );

        self.current_fn = None;

        Ok(())
    }

    fn visit_return(&mut self, return_expr: Return) -> Result<Self::Output, Self::Error> {
        if self.current_fn.is_none() {
            return Err(());
        }
        if let Some(expr) = return_expr.expression {
            let expr = self.visit_expr(*expr)?;
            self.current_fn.as_mut().unwrap().push(
                BodyFn::Return(
                    MirReturn::new(expr)
                )
            );
        } else {
            self.current_fn.as_mut().unwrap().push(
                BodyFn::Return(
                    MirReturn::new(Value::Const(Const::Void))
                )
            );
        }

        Ok(())
    }

    fn visit_import(&mut self, import: ImportStmt) -> Result<Self::Output, Self::Error> {
        let path = import.path;

        // convert stmt path to path

        let path = path.segments
            .iter()
            .map(|segment| segment.name.clone())
            .collect::<Vec<String>>()
            .join("/");


        self.ir.push(
            Ir::LoadModule(
                MirString::new(path)
            )
        );

        Ok(())
    }

    fn visit_external(&mut self, external: External) -> Result<Self::Output, Self::Error> {
        let file = external.file;
        self.ir.push(
            Ir::LoadExternal(
                MirString::new(file)
            )
        );

        for fn_sign in external.signs {
            self.compile_fn_sign(fn_sign);
        }

        Ok(())

    }

    fn visit_for_stmt(&mut self, for_stmt: ForStmt) -> Result<Self::Output, Self::Error> {
        todo!()
    }

    fn visit_struct_stmt(&mut self, struct_stmt: StructStmt) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}