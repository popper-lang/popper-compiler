mod stmt_visitor;
mod expr_visitor;

use std::collections::HashMap;
use std::path::Path;
use popper_ast::{FunctionSign, Statement, Type, TypeKind};
use popper_ast::visitor::StmtVisitor;
use crate::mir_ast::{Alloc, Body, BodyFn, Declare, Ir, List, Module, Type as MirType, Value};


#[derive(Debug, Clone)]
pub struct MirCompiler {
    pub(crate) ast: Vec<Statement>,
    pub(crate) ir: Module,
    pub(crate) current_fn: Option<Body>,
    pub(crate) local: HashMap<String, MirType>,
    pub(crate) global: HashMap<String, MirType>,
    pub(crate) var_id: usize,
    pub(crate) can_alloc: bool,
    let_name: Option<String>,
    is_let_name_used: bool
}

impl MirCompiler {
    pub fn new(ast: Vec<Statement>, file_name: String) -> Self {
        let module_name = Path::new(file_name.as_str())
            .with_extension("")
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        Self {
            ast,
            ir: Module::new(module_name, vec![]),
            current_fn: None,
            local: HashMap::new(),
            global: HashMap::new(),
            can_alloc: true,
            var_id: 0,
            let_name: None,
            is_let_name_used: false
        }
    }

    pub fn compile_type(&mut self, ty: Type) -> MirType {
        match ty.type_kind {
            TypeKind::Int => MirType::Int,
            TypeKind::Float => MirType::Float,
            TypeKind::String(e) => MirType::String(e as usize),
            TypeKind::Bool => MirType::Bool,
            TypeKind::Unit => MirType::Void,
            TypeKind::List(ty, len) => MirType::List(Box::new(self.compile_type(*ty)), len),
            _ => unimplemented!()
        }
    }

    pub fn compile_fn_sign(&mut self, fn_sign: FunctionSign)  {
        let args = fn_sign.arguments.args.iter().map(|arg| self.compile_type(arg.ty.clone())).collect::<Vec<MirType>>();
        let ret = self.compile_type(fn_sign.return_type);
        self.ir.push(
            Ir::Declare(
                Declare::new(fn_sign.name.clone(), List::new(args.clone()), ret.clone())
            )
        );

        self.global.insert(fn_sign.name, ret);

    }

    pub fn get(&self, name: &str) -> Option<MirType> {
        if let Some(ty) = self.local.get(name) {
            return Some(ty.clone());
        }
        if let Some(ty) = self.global.get(name) {
            return Some(ty.clone());
        }
        None
    }

    pub fn compile(&mut self) -> Module {
        for stmt in self.ast.clone() {
            self.visit_stmt(stmt).unwrap();
        }
        self.ir.clone()
    }

    pub fn new_var_id(&mut self, ty: MirType) -> Result<String, ()> {
        if self.current_fn.is_none() {
            return Err(());
        }

        let var = if let Some(let_name) = self.let_name.clone() {
            self.is_let_name_used = true;
            let_name
        } else {
            let e = format!("__{}", self.var_id);
            self.var_id += 1;
            e
        };
        if self.can_alloc {
            let current_fn = self.current_fn.as_mut().unwrap();
            current_fn.push(
                BodyFn::Alloc(
                    Alloc::new(
                        var.clone(),
                        ty
                    )
                )
            );
        }
        Ok(var.clone())
    }

    pub fn new_var_id_no_alloc(&mut self, ty: MirType) -> Result<String, ()> {
        self.can_alloc = false;
        let var = self.new_var_id(ty);
        self.can_alloc = true;
        var
    }

    pub fn get_minor_type_from_list(&self, val: Value) -> Option<MirType> {
        match val {
            Value::Variable(v) => {
                match v.ty {
                    MirType::List(l, _) => Some(*l),
                    _ => None
                }
            },
            Value::Const(_) => {
                val.into_array().map(|x| x.get_minor_type())
            }

        }
    }

    pub fn get_module(&self) -> Module {
        self.ir.clone()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use popper_ast::{Argument, Arguments, Expression, Function, Int, Return, Statement, Type, TypeKind, Constant};
    use crate::mir_ast::{Body, Declare, Function as MirFunction, Ir, List, Module, Type as MirType};

    #[test]
    fn test_function() {
        let ast = vec![
            Statement::Function(
                Function::new(
                    "main".to_string(),
                    Arguments::new(
                        vec![
                            Argument::new(
                                "a".to_string(),
                                Type::new(
                                    Default::default(),
                                    TypeKind::Int,
                                    Default::default()
                                ),
                                Default::default()
                            )
                        ],
                        Default::default()
                    ),
                    Type::new(
                        Default::default(),
                        TypeKind::Int,
                        Default::default()
                    ),
                    vec![
                        Statement::Return(
                            Return::new(
                                Some(
                                    Expression::Constant(
                                        Constant::Int(
                                            Int::new(
                                                Default::default(),
                                                1
                                            )
                                        )
                                    )
                                ),
                                Default::default()
                            )
                        )
                    ],
                    Default::default()
                )
            )
        ];

        let mut compiler = MirCompiler::new(ast, "test".to_string());

        let ir = compiler.compile();

        assert_eq!(ir, Module::new("ss".to_string(), vec![]))
    }
}
