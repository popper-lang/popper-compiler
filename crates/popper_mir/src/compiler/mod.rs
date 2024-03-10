use std::collections::HashMap;
use popper_ast::{BinOpKind, Constant, Expression as AstExpression, Statement as AstStatement, Type as AstType, TypeKind as AstTypeKind};
use crate::builder::Builder;
use crate::consts::{ConstKind, Ident};
use crate::debug::VarDebugKind;
use crate::expr::Expr;
use crate::function::Function;
use crate::types::Types;

#[derive(Debug, Clone)]
pub struct Compiler {
    builder: Builder,
    program: Vec<AstStatement>,
    env: HashMap<String, Ident>,
    function_env: HashMap<String, Function>,
    current_var: Option<Ident>
}

impl Compiler {
    pub fn new(program: Vec<AstStatement>) -> Self {
        Self {
            builder: Builder::new(),
            program,
            env: HashMap::new(),
            current_var: None,
            function_env: HashMap::new()
        }
    }
    
    pub fn remove_debug(&mut self, id: Ident) {
        self.builder.remove_debug_info(id);
    }

    pub fn debug_var(&mut self, id: Ident, name: &str) {
        self.builder.set_debug_info(id, VarDebugKind::Var(name.to_string()));
    }

    pub fn debug_internal(&mut self, id: Ident) {
        self.builder.set_debug_info(id, VarDebugKind::Internal);
    }

    pub fn new_internal_ident(&mut self, ty: Types) -> Ident {
        if let Some(id) = self.current_var.clone() {
            let mut id = id;
            self.current_var = None;
            id.set_type(ty.clone());
            self.env.iter_mut().filter(|x| x.1 == &id).for_each(|x| {
                x.1.set_type(ty.clone());
            });
            return id;
        }
        let id =  self.builder.build_let_decl(ty);
        self.debug_internal(id.clone());
        id
    }

    pub fn new_var(&mut self, ty: Types, name: &str) -> Ident {
        let id = self.builder.build_let_decl(ty);
        self.env.insert(name.to_string(), id.clone());
        self.debug_var(id.clone(), name);
        id
    }

    pub fn compile(&mut self) {
        for stmt in self.program.clone() {
            self.compile_stmt(&stmt);
        }
    }

    fn compile_stmt(&mut self, stmt: &AstStatement) {
        match stmt {
            AstStatement::Function(f) => {
                self.compile_function(f);
            },
            AstStatement::Extern(ex) => {
                for sign in ex.signs.clone() {
                    self.builder.build_external_function(&sign.name, sign.arguments.args.iter().map(|x| x.ty.clone().into()).collect(), sign.return_type.clone().into(), sign.is_var_args);
                }
            },
            AstStatement::Expression(expr) => {
                self.compile_expression(expr.clone());
            },
            AstStatement::Let(l) => {
                let expr = self.compile_expression(l.value.clone());
                let id = expr.expect_ident();
                self.env.insert(l.name.name.clone(), id.clone());
                self.remove_debug(id.clone());
                self.debug_var(id, &l.name.name);
            },
            AstStatement::Return(r) => {
                let expr = if let Some(ret) = r.expression.clone() {
                    self.compile_expression(*ret)
                } else {
                    Expr::Const(ConstKind::Null)
                };
                self.builder.build_ret_command(expr);
            },
            _ => todo!()
        }
    }

    fn compile_function(&mut self, f: &popper_ast::Function) {
        self.builder.build_function(&f.name, f
            .arguments
            .args
            .iter()
            .map(|x| x.ty.clone().into())
            .collect(), f.returntype.clone().into());


        for stmt in f.body.iter() {
            self.compile_stmt(stmt);
        }
        let func = self.builder.end_function();

        self.function_env.insert(f.name.clone(), func);
    }

    pub fn compile_expression(&mut self, expr: AstExpression) -> Expr {
        match expr {
            AstExpression::BinOp(binop) => {
                let lhs = self.compile_expression(*binop.lhs);
                let rhs = self.compile_expression(*binop.rhs);
                let res_ty = if binop.op.is_arithmetic() {
                    lhs.get_type()
                } else {
                    Types::Bool
                };

                let res = self.new_internal_ident(res_ty.clone());
                match binop.op {
                    BinOpKind::Add => {
                        self.builder.build_add_command(res.clone(), lhs, rhs);
                        Expr::Ident(res)
                    },
                    BinOpKind::Eq => {
                        self.builder.build_cmp_eq_command(res.clone(), lhs, rhs);
                        Expr::Ident(res)
                    },
                    _ => todo!()
                }

            },
            AstExpression::Constant(e) => {
                match e {
                    Constant::Ident(i) => {
                        Expr::Ident(
                            self.env.get(&i.name).unwrap().clone()
                        )
                    },
                    Constant::Int(i) => {
                        let res = self.new_internal_ident(Types::Int);
                        self.builder.build_const_command(res.clone(), ConstKind::Int(i.value));
                        Expr::Ident(res)
                    },
                    Constant::Bool(b) => {
                        let res = self.new_internal_ident(Types::Bool);
                        self.builder.build_const_command(res.clone(), ConstKind::Bool(b.value));
                        Expr::Ident(res)
                    },
                    Constant::StringLiteral(s) => {
                        let res = self.new_internal_ident(Types::String(s.value.len()));
                        self.builder.build_const_command(res.clone(), ConstKind::Str(s.value));
                        Expr::Ident(res)
                    },
                    Constant::Float(f) => {
                        let res = self.new_internal_ident(Types::Float);
                        self.builder.build_const_command(res.clone(), ConstKind::Float(f.value));
                        Expr::Ident(res)
                    },
                    Constant::Null(_) => {
                        let res = self.new_internal_ident(Types::Unit);
                        self.builder.build_const_command(res.clone(), ConstKind::Null);
                        Expr::Ident(res)
                    },
                    Constant::List(l) => {
                        let mut list = Vec::new();
                        for e in l.value.iter() {
                            list.push(self.compile_expression(e.clone()));
                        }
                        let res = self.new_internal_ident(Types::List(Box::new(list[0].get_type()), l.value.len()));
                        self.builder.build_const_command(res.clone(), ConstKind::List(list.to_vec()));
                        Expr::Ident(res)
                    },
                    _ => todo!()
                }
            },
            AstExpression::Call(call) => {
                let args = call.arguments.iter().map(|x| self.compile_expression(x.clone())).collect();
                let func = self.function_env.get(&call.name).unwrap();
                let res = self.new_internal_ident(func.ret.clone());
                self.builder.build_call_command(call.name, args, res.clone());
                Expr::Ident(res)
            }
            _ => todo!()
        }

    }

    pub fn get_builder(self) -> Builder {
        self.builder
    }
}

impl Into<Types> for AstType {
    fn into(self) -> Types {
        match self.type_kind {
            AstTypeKind::Int => Types::Int,
            AstTypeKind::Bool => Types::Bool,
            AstTypeKind::Unit => Types::Unit,
            AstTypeKind::String(e) => Types::String(e as usize),
            AstTypeKind::List(e, l) => Types::List(Box::new((*e).into()), l),
            AstTypeKind::Pointer(p) => Types::Ptr(Box::new((*p).into())),
            _ => unimplemented!()

        }
    }
}