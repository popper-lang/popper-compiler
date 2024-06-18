use std::collections::HashMap;
use std::rc::Rc;
use popper_ast::{BinOpKind, Constant, Expression as AstExpression, Statement as AstStatement, StructStmt as AstStructStmt, Type as AstType, TypeKind as AstTypeKind};
use crate::builder::Builder;
use crate::consts::{ConstKind, Ident, TypeId};
use crate::debug::VarDebugKind;
use crate::expr::Expr;
use crate::function::Function;
use crate::marks::MarkKind;
use crate::types::Types;

mod const_table;

#[derive(Debug, Clone)]
pub struct Compiler {
    builder: Builder,
    program: Vec<AstStatement>,
    env: HashMap<String, Ident>,
    ty_env: HashMap<String, AstStructStmt>,
    function_env: HashMap<String, Function>,
    const_table: const_table::ConstTable,
    const_id: usize,
    gep_table: HashMap<String, Ident>,
    is_already_returned: bool,
    no_load: bool

}

impl Compiler {
    pub fn new(program: Vec<AstStatement>) -> Self {
        Self {
            builder: Builder::new(),
            program,
            env: HashMap::new(),
            function_env: HashMap::new(),
            ty_env: HashMap::new(),
            const_table: const_table::ConstTable::new(),
            const_id: 0,
            gep_table: HashMap::new(),
            is_already_returned: false,
            no_load: false
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

    pub fn use_id(&mut self, id: Ident, n: i64) {
        for _ in 0..n {
            self.builder.use_ident(id.clone());
        }
    }
    pub fn new_const(&mut self, c: ConstKind) -> Ident {
        let id = self.new_internal_ident(c.get_type());
        self.builder.build_const_command(id.clone(), c.clone());
        self.const_table.insert(id.clone(), c.clone());
        id
    }
    
    pub fn new_const_with_type(&mut self, c: ConstKind, ty: Types) -> Ident {
        let id = self.new_internal_ident(ty.clone());
        self.builder.build_const_command(id.clone(), c.clone());
        self.const_table.insert(id.clone(), c.clone());
        id
    }

    pub fn get_type_from_ast(&self, ast: AstType) -> Types {
        match ast.type_kind {
            AstTypeKind::Int => Types::Int,
            AstTypeKind::Bool => Types::Bool,
            AstTypeKind::Float => Types::Float,
            AstTypeKind::String(l) => Types::String(l as usize),
            AstTypeKind::List(t, len) => Types::List(Box::new(self.get_type_from_ast(*t)), len),
            AstTypeKind::Pointer(t) => Types::Ptr(Box::new(self.get_type_from_ast(*t))),
            AstTypeKind::Struct(s) => {
                Types::TypeId(s)
            },
            AstTypeKind::Pointer(p) => Types::Ptr(Box::new(self.get_type_from_ast(*p))),
            AstTypeKind::Unit => Types::Unit,
            e => todo!("{:?}", e)
        }
    }
    
    pub fn no_load<T>(&mut self, f: impl Fn(&mut Self) -> T)  -> T {
        self.no_load = true;
        let res = f(self);
        self.no_load = false;
        res
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
                    let f = self.builder.build_external_function(
                        &sign.name,
                        sign.arguments.args
                            .iter()
                            .map(|x| self.get_type_from_ast(x.ty.clone()))
                            .collect(),
                        self.get_type_from_ast(sign.return_type
                            .clone()
                        )
                            ,
                        sign.is_var_args
                    );

                    self.function_env.insert(sign.name.clone(), f.into());
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
                self.is_already_returned = true;
                self.builder.build_ret_command(expr);
            },
            AstStatement::Struct(s) => {
                self.ty_env.insert(s.name.clone(), s.clone());
                let mut fields = Vec::new();
                for field in s.fields.iter() {
                    fields.push(
                        self.get_type_from_ast(
                            field.ty.clone()
                        )
                    );
                }
                self.builder.build_type_decl(TypeId::new(s.name.clone()), Types::Struct(s.name.clone(), fields));
            },
            AstStatement::Assign(a) => {
                let ident = self.no_load(|s| 
                    s.compile_expression(a.name.clone()).expect_ident()
                );
                self.builder.marks_ident(ident.clone(), MarkKind::Ptr);
                let value = self.compile_expression(a.value.clone());
                self.builder.build_write(ident, value);
            },
            _ => todo!()
        }
    }

    fn compile_function(&mut self, f: &popper_ast::Function) {
        let ret = self.get_type_from_ast(f.returntype.clone());
        self.builder.build_function(&f.name, f
            .arguments
            .args
            .iter()
            .map(|x| 
                self.get_type_from_ast(x.ty.clone()))
            .collect(), self.get_type_from_ast(f.returntype.clone()));
        for arg in f.arguments.args.iter() {
            let id = self.builder.new_ident(
                self.get_type_from_ast(arg.ty.clone())
            );
            self.env.insert(arg.name.clone(), id.clone());
            self.debug_var(id, &arg.name);
        }
        for stmt in f.body.iter() {
            self.compile_stmt(stmt);
        }
        
        if !self.is_already_returned {
            self.builder.build_ret_command(Expr::Const(ConstKind::Null));
        }
        let func = self.builder.end_function();
        self.function_env.insert(f.name.clone(), func);
        self.gep_table.clear();
        self.is_already_returned = false;
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
                        let name = self.env.get(&i.name).unwrap();
                        Expr::Ident(
                            name.clone()
                        )
                    },
                    Constant::Int(i) => {
                        let res = self.new_const(ConstKind::Int(i.value));
                        Expr::Ident(res)
                    },
                    Constant::Bool(b) => {
                        let res = self.new_const(ConstKind::Bool(b.value));
                        Expr::Ident(res)
                    },
                    Constant::StringLiteral(s) => {
                        let res = self.new_const(ConstKind::Str(s.value));
                        Expr::Ident(res)
                    },
                    Constant::Float(f) => {
                        let res = self.new_const(ConstKind::Float(f.value));
                        Expr::Ident(res)
                    },
                    Constant::Null(_) => {
                        let res = self.new_const(ConstKind::Null);
                        Expr::Ident(res)
                    },
                    Constant::List(l) => {
                        let mut list = Vec::new();
                        for e in l.value.iter() {
                            list.push(self.compile_expression(e.clone()));
                        }
                        let res = self.new_const(ConstKind::List(list));
                        Expr::Ident(res)
                    },
                    _ => todo!()
                }
            },
            AstExpression::Call(call) => {
                let func = self.function_env.get(&call.name).unwrap().clone();
                let mut args = vec![];
                for arg in call.arguments.iter() {
                    args.push(self.compile_expression(arg.clone()));
                }
                if func.ret == Types::Unit {
                    self.builder.build_single_call_command(call.name, args);
                    return Expr::Const(ConstKind::Null);
                }
                let res = self.new_internal_ident(func.ret.clone());
                self.builder.build_call_command(call.name, args, res.clone());
                Expr::Ident(res)
            },
            AstExpression::StructInstance(s) => {
                let mut fields = Vec::new();
                for field in s.fields.iter() {
                    fields.push(self.compile_expression(field.clone().value));
                }
                let const_ = ConstKind::Struct(TypeId::new(s.name.clone()), fields);
                let res = self.new_const_with_type(const_, Types::TypeId(s.name.clone()));
                Expr::Ident(res)
            },
            AstExpression::StructFieldAccess(s) => {
                let struct_ = self.compile_expression(*s.name);
                let struct_id = struct_.clone().expect_ident();
                let struct_ty = struct_id.get_type();
                let ptr;
                let struct_name;
                (ptr, struct_name) = if let Some(s_name) = struct_ty.get_type_id() {
                    let ptr = if let Some(e) = self.gep_table.get(&s_name) {
                        e.clone()
                    } else {
                        let ptr_struct_ty = Types::Ptr(Box::new(struct_ty.clone()));
                        let ptr = self.new_internal_ident(ptr_struct_ty.clone());
                        self.builder.build_llvm_store_command(ptr.clone(), struct_id.clone(), struct_ty.clone());
                        self.gep_table.insert(s_name.clone(), ptr.clone());
                        ptr
                    };
                    (ptr, s_name) 
                } else if let Types::Ptr(e) = struct_ty {
                    let s_name = e.expect_type_id();
                    (struct_id, s_name)
                } else {
                    todo!()
                };

                let ty = self.ty_env.get(&struct_name).unwrap();
                let s = ty.fields.iter().position(|x| x.name == s.field).unwrap();
                let ty: Types = self.get_type_from_ast(ty.fields[s].ty.clone());
                let res_type = Types::Ptr(Box::new(ty.clone()));
                let res = self.new_internal_ident(res_type.clone());
                self.builder.build_gep_struct_command(res.clone(), ptr, res_type, Expr::Const(ConstKind::Int(s as i64)), TypeId::new(struct_name));
                Expr::Ident(res)
            },
            AstExpression::Reference(r) => {
                let id = self.compile_expression(*r.expr).expect_ident();
                let ty = id.get_type();
                let res = self.new_internal_ident(Types::Ptr(Box::new(ty.clone())));
                self.builder.build_llvm_store_command(res.clone(), id, ty);
                Expr::Ident(res)
            },
            AstExpression::Deref(d) => {
                if self.no_load {
                    return self.compile_expression(*d.expr);
                }
                let id = self.compile_expression(*d.expr);
                let ty = id.get_type();
                let ty = ty.get_ptr_inner_type();
                let res = self.new_internal_ident(ty.clone());
                self.builder.build_llvm_load_ptr_command(res.clone(), id.expect_ident(), ty);
                Expr::Ident(res)
            },
            _ => todo!()
        }

    }

    pub fn get_builder(self) -> Builder {
        self.builder
    }
}

