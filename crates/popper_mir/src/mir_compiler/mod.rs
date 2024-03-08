mod expr_visitor;
mod stmt_visitor;

use crate::mir_ast::{Alloc, Body, BodyFn, Declare, Function, Ir, Label, List, Module, Type as MirType, Value, Variable};
use popper_ast::visitor::StmtVisitor;
use popper_ast::{FunctionSign, Statement, Type, TypeKind};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct MirCompiler {
    pub(crate) ast: Vec<Statement>,
    pub(crate) ir: Module,
    pub(crate) current_label: Option<Label>,
    pub(crate) current_fn: Option<Function>,
    pub(crate) local: HashMap<String, MirType>,
    pub(crate) global: HashMap<String, MirType>,
    pub(crate) var_id: usize,
    pub(crate) label_counter: usize,
    pub(crate) can_alloc: bool,
    let_name: Option<String>,
    is_let_name_used: bool,
    is_returned: bool,
    break_depth: i32,
    loop_depth: i32,
    exit_loop: Option<Label>,
    current_dt: i32,
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
            current_label: None,
            local: HashMap::new(),
            global: HashMap::new(),
            can_alloc: true,
            var_id: 0,
            label_counter: 0,
            let_name: None,
            is_let_name_used: false,
            is_returned: false,
            break_depth: -1,
            loop_depth: 0,
            exit_loop: None,
            current_dt: 2,
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
            TypeKind::Pointer(ty) => MirType::Pointer(Box::new(self.compile_type(*ty))),
            e => panic!("not implemented {:?}", e),
        }
    }

    pub fn compile_fn_sign(&mut self, fn_sign: FunctionSign) {
        let args = fn_sign
            .arguments
            .args
            .iter()
            .map(|arg| self.compile_type(arg.ty.clone()))
            .collect::<Vec<MirType>>();
        let ret = self.compile_type(fn_sign.return_type);
        self.ir.push(Ir::Declare(Declare::new(
            fn_sign.name.clone(),
            List::new(args.clone()),
            ret.clone(),
            fn_sign.is_var_args,
        )));

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
        if self.current_label.is_none() {
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
            let current_label = self.current_label.as_mut().unwrap();
            current_label.push(BodyFn::Alloc(Alloc::new(var.clone(), ty)));
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
            Value::Variable(v) => match v.ty {
                MirType::List(l, _) => Some(*l),
                _ => None,
            },
            Value::Const(_) => val.into_array().map(|x| x.get_minor_type()),
        }
    }

    pub fn new_label(&mut self) -> Label {
        let label = Label::new(format!("L{}", self.label_counter), vec![]);
        self.label_counter += 1;
        label
    }

    pub fn new_labels(&mut self, n: usize) -> Vec<Label> {
        let mut labels = vec![];
        for _ in 0..n {
            labels.push(self.new_label());
        }
        labels
    }

    pub fn add_label(&mut self, label: Label) {
        self.current_fn.as_mut().unwrap().body.push(label);
    }

    pub fn add_current_label(&mut self) {
        self.add_label(self.current_label.clone().unwrap());
    }

    pub fn push_on_label(&mut self, f: BodyFn) {
        if self.is_returned || self.break_depth == self.loop_depth {
            return;
        }
        self.current_label.as_mut().unwrap().push(f);
    }

    pub fn set_current_label(&mut self, label: Label) {
        self.current_label = Some(label);
    }

    pub fn get_module(&self) -> Module {
        self.ir.clone()
    }
}
