use popper_ast::ast::{
    Const, Expr, Ident, LangNode, LangNodeId, LangNodeKind, Let, MacroDef, ParamDef, Span,
    SymbolId, TypeDeclKind, UnaryOpKind,
};
use popper_ast::layer::{Ast, Layer};
use popper_ast::type_::{Fields, Type, TypeInfo};
use popper_semantic_analyzer::hir::{Hir, HirNode, HirNodeId};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Bool(bool),
    String(String),
    Char(char),
    Float(f64),
    Void,
    List(Vec<Value>),
    Struct(HashMap<SymbolId, Value>),
    Type(popper_ast::type_::Type),
}

impl Value {
    pub fn to_expr(&self, ast: &mut Hir) -> Expr {
        match self {
            Value::Int(i) => Expr::Int(*i),
            Value::Bool(b) => Expr::Bool(*b),
            Value::String(s) => Expr::String(s.clone()),
            Value::Char(c) => Expr::Char(*c),
            Value::Float(f) => Expr::Float(*f),
            Value::Void => panic!("Void value cannot be converted to Expr"),
            Value::Type(t) => Expr::Type(t.clone()),
            Value::List(l) => {
                let nodes = l
                    .iter()
                    .map(|v| {
                        let expr = v.to_expr(ast);
                        let descriptor = ast.add_descriptor(
                            popper_semantic_analyzer::hir::NodeDescriptor::default(),
                        );
                        ast.add(HirNode {
                            node: LangNode {
                                kind: LangNodeKind::Expr(expr),
                                span: Span::new(0, 0),
                            },
                            descriptor,
                        })
                        .into()
                    })
                    .collect();
                Expr::List(nodes)
            }
            Value::Struct(_) => todo!("Struct value to Expr conversion is not implemented yet"),
        }
    }
}

pub type BuiltinFn = fn(&mut Evaluator, Vec<HirNodeId>) -> Option<Value>;

pub struct Evaluator<'a> {
    ast: &'a Hir,
    constants: HashMap<SymbolId, Value>,
    macros: HashMap<SymbolId, MacroDef>,
    macro_params: HashMap<SymbolId, HirNodeId>,
    builtins: HashMap<String, BuiltinFn>,
    symbol_map: HashMap<SymbolId, SymbolId>,
}

impl<'a> Evaluator<'a> {
    pub fn new(ast: &'a Hir) -> Self {
        let mut eval = Self {
            ast,
            constants: HashMap::new(),
            macros: HashMap::new(),
            macro_params: HashMap::new(),
            builtins: HashMap::new(),
            symbol_map: HashMap::new(),
        };
        eval.register_default_builtins();
        eval
    }

    fn register_default_builtins(&mut self) {
        self.builtins.insert("sizeOf".to_string(), |eval, args| {
            let arg = eval.evaluate(*args.get(0)?)?;
            match arg {
                Value::Type(ty) => Some(Value::Int(ty.size_of() as i64)),
                _ => None,
            }
        });

        self.builtins.insert("as".to_string(), |eval, args| {
            let _target_type = eval.evaluate(*args.get(0)?)?;
            let value = eval.evaluate(*args.get(1)?)?;
            Some(value)
        });

        self.builtins.insert("typeOf".to_string(), |eval, args| {
            let val = eval.evaluate(*args.get(0)?)?;
            match val {
                Value::Int(_) => Some(Value::Type(popper_ast::type_::Type::Int)),
                Value::Bool(_) => Some(Value::Type(popper_ast::type_::Type::Bool)),
                Value::String(_) => Some(Value::Type(popper_ast::type_::Type::Ptr(Box::new(
                    popper_ast::type_::Type::Char,
                )))),
                Value::Char(_) => Some(Value::Type(popper_ast::type_::Type::Char)),
                Value::Float(_) => Some(Value::Type(popper_ast::type_::Type::Float)),
                Value::Type(_) => Some(Value::Type(popper_ast::type_::Type::Type(None))),
                Value::List(l) => {
                    todo!()
                }
                _ => None,
            }
        });

        self.builtins.insert("typeName".to_string(), |eval, args| {
            let arg = eval.evaluate(*args.get(0)?);
            println!("ARG: {:?}", arg);
            match arg.unwrap() {
                Value::Type(ty) => match &ty {
                    popper_ast::type_::Type::AggregateType(name, _, _) => {
                        Some(Value::String(name.clone()))
                    }
                    _ => Some(Value::String(ty.to_string())),
                },
                _ => None,
            }
        });

        self.builtins
            .insert("fieldCount".to_string(), |eval, args| {
                let arg = eval.evaluate(*args.get(0)?)?;
                match arg {
                    Value::Type(ty) => match ty {
                        popper_ast::type_::Type::AggregateType(_, fields, _) => {
                            Some(Value::Int(fields.fields.len() as i64))
                        }
                        _ => Some(Value::Int(0)),
                    },
                    _ => None,
                }
            });

        self.builtins.insert("fieldName".to_string(), |eval, args| {
            let arg_ty = eval.evaluate(*args.get(0)?)?;
            let arg_idx = eval.evaluate(*args.get(1)?)?;
            match (arg_ty, arg_idx) {
                (Value::Type(ty), Value::Int(idx)) => match ty {
                    popper_ast::type_::Type::AggregateType(_, fields, _) => fields
                        .fields
                        .get(idx as usize)
                        .map(|(name, _)| Value::String(name.clone())),
                    _ => None,
                },
                _ => None,
            }
        });

        self.builtins.insert("fieldType".to_string(), |eval, args| {
            let arg_ty = eval.evaluate(*args.get(0)?)?;
            let arg_idx = eval.evaluate(*args.get(1)?)?;
            match (arg_ty, arg_idx) {
                (Value::Type(ty), Value::Int(idx)) => match ty {
                    popper_ast::type_::Type::AggregateType(_, fields, _) => fields
                        .fields
                        .get(idx as usize)
                        .map(|(_, t)| Value::Type(t.clone())),
                    _ => None,
                },
                _ => None,
            }
        });

        self.builtins
            .insert("fieldOffset".to_string(), |eval, args| {
                let arg_ty = eval.evaluate(*args.get(0)?)?;
                let arg_idx = eval.evaluate(*args.get(1)?)?;
                match (arg_ty, arg_idx) {
                    (Value::Type(ty), Value::Int(idx)) => match ty {
                        popper_ast::type_::Type::AggregateType(_, fields, kind) => {
                            if kind == popper_ast::ast::TypeDeclKind::Union {
                                return Some(Value::Int(0));
                            }
                            let mut offset = 0;
                            for i in 0..(idx as usize) {
                                if let Some((_, t)) = fields.fields.get(i) {
                                    offset += t.size_of();
                                }
                            }
                            Some(Value::Int(offset as i64))
                        }
                        _ => None,
                    },
                    _ => None,
                }
            });
    }

    pub fn register_builtin(&mut self, name: String, f: BuiltinFn) {
        self.builtins.insert(name, f);
    }

    pub fn evaluate(&mut self, node_id: HirNodeId) -> Option<Value> {
        let node = self.ast.get(node_id);
        println!("node : {:?}", node);
        match &node.node.kind {
            LangNodeKind::Expr(expr) => self.evaluate_expr(expr, node_id),
            LangNodeKind::Const(c) => {
                let val = self.evaluate(HirNodeId::from(c.value))?;
                self.constants.insert(c.name.0, val.clone());
                Some(val)
            }
            LangNodeKind::Block(nodes) => {
                let mut last = None;
                for &node in nodes {
                    last = self.evaluate(HirNodeId::from(node));
                }
                last
            }
            LangNodeKind::Comptime(n) => self.evaluate(HirNodeId::from(*n)),
            _ => None,
        }
    }

    fn evaluate_expr(&mut self, expr: &Expr, node_id: HirNodeId) -> Option<Value> {
        match expr {
            Expr::Int(i) => Some(Value::Int(*i)),
            Expr::Bool(b) => Some(Value::Bool(*b)),
            Expr::String(s) => Some(Value::String(s.clone())),
            Expr::Char(c) => Some(Value::Char(*c)),
            Expr::Ident(id) => {
                if let Some(Type::Type(Some(info))) = self.ast.get_type(node_id) {
                    Some(Value::Type(*info.ty))
                } else {
                    self.constants.get(&id.0).cloned()
                }
            }
            Expr::Add(lhs, rhs) => {
                let l = self.evaluate(HirNodeId::from(*lhs))?;
                let r = self.evaluate(HirNodeId::from(*rhs))?;
                match (l, r) {
                    (Value::Int(l), Value::Int(r)) => Some(Value::Int(l + r)),
                    _ => None,
                }
            }
            Expr::Sub(lhs, rhs) => {
                let l = self.evaluate(HirNodeId::from(*lhs))?;
                let r = self.evaluate(HirNodeId::from(*rhs))?;
                match (l, r) {
                    (Value::Int(l), Value::Int(r)) => Some(Value::Int(l - r)),
                    _ => None,
                }
            }
            Expr::Mul(lhs, rhs) => {
                let l = self.evaluate(HirNodeId::from(*lhs))?;
                let r = self.evaluate(HirNodeId::from(*rhs))?;
                match (l, r) {
                    (Value::Int(l), Value::Int(r)) => Some(Value::Int(l * r)),
                    _ => None,
                }
            }
            Expr::Div(lhs, rhs) => {
                let l = self.evaluate(HirNodeId::from(*lhs))?;
                let r = self.evaluate(HirNodeId::from(*rhs))?;
                match (l, r) {
                    (Value::Int(l), Value::Int(r)) if r != 0 => Some(Value::Int(l / r)),
                    _ => None,
                }
            }
            Expr::Eq(lhs, rhs) => {
                let l = self.evaluate(HirNodeId::from(*lhs))?;
                let r = self.evaluate(HirNodeId::from(*rhs))?;
                Some(Value::Bool(l == r))
            }
            Expr::Lt(lhs, rhs) => {
                let l = self.evaluate(HirNodeId::from(*lhs))?;
                let r = self.evaluate(HirNodeId::from(*rhs))?;
                match (l, r) {
                    (Value::Int(l), Value::Int(r)) => Some(Value::Bool(l < r)),
                    _ => None,
                }
            }
            Expr::UnaryOp(kind, operand) => {
                let val = self.evaluate(HirNodeId::from(*operand))?;
                match (kind, val) {
                    (UnaryOpKind::ArithmeticNegate, Value::Int(i)) => Some(Value::Int(-i)),
                    (UnaryOpKind::Negate, Value::Bool(b)) => Some(Value::Bool(!b)),
                    _ => None,
                }
            }
            Expr::Type(ty) => Some(Value::Type(ty.clone())),
            Expr::BuiltinCall { name, args } => {
                let f = self.builtins.get(name)?.clone();
                f(self, args.iter().map(|&a| HirNodeId::from(a)).collect())
            }
            _ => None,
        }
    }

    pub fn fold(&mut self) -> Hir {
        self.collect_macros();
        let mut new_ast = Hir::new();
        let root = self.ast.root();
        let new_root = self.fold_node(root, &mut new_ast);
        new_ast.set_root(new_root.into());
        new_ast
    }

    fn collect_macros(&mut self) {
        for node_id in self.ast.nodes() {
            let node = self.ast.get(node_id);
            if let LangNodeKind::MacroDef(m) = &node.node.kind {
                self.macros.insert(m.name.0, m.clone());
            }
        }
    }

    fn expand_macro(&mut self, m: MacroDef, args: &[LangNodeId], new_ast: &mut Hir) -> HirNodeId {
        let old_params = self.macro_params.clone();

        for (param, &arg) in m.params.iter().zip(args.iter()) {
            let folded_arg = self.fold_node(arg.into(), new_ast);
            self.macro_params.insert(param.name.0, folded_arg);
        }

        let result = self.fold_node(m.body.into(), new_ast);

        self.macro_params = old_params;
        result
    }

    fn fold_node(&mut self, node_id: HirNodeId, new_ast: &mut Hir) -> HirNodeId {
        let node = self.ast.get(node_id);
        let node = &node.node;

        if let LangNodeKind::Expr(Expr::Ident(id)) = &node.kind {
            if let Some(&subst_id) = self.macro_params.get(&id.0) {
                return subst_id;
            }
        }

        let span = node.span;
        let new_kind = match &node.kind {
            LangNodeKind::Expr(expr) => LangNodeKind::Expr(self.fold_expr(expr, new_ast)),
            LangNodeKind::Let(let_) => LangNodeKind::Let(Let {
                name: Ident(self.fold_symbol(let_.name.0, new_ast)),
                value: self.fold_node(let_.value.into(), new_ast).into(),
            }),
            LangNodeKind::Block(nodes) => LangNodeKind::Block(
                nodes
                    .iter()
                    .map(|&n| self.fold_node(n.into(), new_ast).into())
                    .collect(),
            ),
            LangNodeKind::If {
                condition,
                then_branch,
                else_branch,
            } => LangNodeKind::If {
                condition: self.fold_node((*condition).into(), new_ast).into(),
                then_branch: self.fold_node((*then_branch).into(), new_ast).into(),
                else_branch: else_branch.map(|e| self.fold_node(e.into(), new_ast).into()),
            },
            LangNodeKind::While { condition, body } => LangNodeKind::While {
                condition: self.fold_node((*condition).into(), new_ast).into(),
                body: self.fold_node((*body).into(), new_ast).into(),
            },
            LangNodeKind::FunctionCall { function, args } => {
                if let LangNodeKind::Expr(Expr::Ident(id)) =
                    &self.ast.get((*function).into()).node.kind
                {
                    if let Some(m) = self.macros.get(&id.0).cloned() {
                        return self.expand_macro(m, args, new_ast);
                    }
                }
                LangNodeKind::FunctionCall {
                    function: self.fold_node((*function).into(), new_ast).into(),
                    args: args
                        .iter()
                        .map(|&a| self.fold_node(a.into(), new_ast).into())
                        .collect(),
                }
            }
            LangNodeKind::Return(expr) => {
                LangNodeKind::Return(self.fold_node((*expr).into(), new_ast).into())
            }
            LangNodeKind::FunctionDef {
                name,
                attrs,
                params,
                ret,
                body,
                is_expr,
                is_vararg,
            } => LangNodeKind::FunctionDef {
                name: Ident(self.fold_symbol(name.0, new_ast)),
                attrs: attrs.clone(),
                params: params
                    .iter()
                    .map(|p| ParamDef {
                        name: Ident(self.fold_symbol(p.name.0, new_ast)),
                        ty: p.ty.clone(),
                    })
                    .collect(),
                ret: ret.clone(),
                body: body.map(|b| self.fold_node(b.into(), new_ast).into()),
                is_expr: *is_expr,
                is_vararg: *is_vararg,
            },
            LangNodeKind::TypeDecl { name, kind, fields } => LangNodeKind::TypeDecl {
                name: Ident(self.fold_symbol(name.0, new_ast)),
                kind: *kind,
                fields: fields
                    .iter()
                    .map(|p| ParamDef {
                        name: Ident(self.fold_symbol(p.name.0, new_ast)),
                        ty: p.ty.clone(),
                    })
                    .collect(),
            },
            LangNodeKind::Assign { lhs, rhs } => LangNodeKind::Assign {
                lhs: self.fold_node((*lhs).into(), new_ast).into(),
                rhs: self.fold_node((*rhs).into(), new_ast).into(),
            },
            LangNodeKind::Const(c) => {
                // On évalue la constante pour la mettre dans le cache
                let _ = self.evaluate(node_id);
                LangNodeKind::Const(Const {
                    name: Ident(self.fold_symbol(c.name.0, new_ast)),
                    value: self.fold_node(c.value.into(), new_ast).into(),
                    ty: c.ty.clone(),
                })
            }
            LangNodeKind::MacroDef(m) => LangNodeKind::MacroDef(MacroDef {
                name: Ident(self.fold_symbol(m.name.0, new_ast)),
                params: m
                    .params
                    .iter()
                    .map(|p| ParamDef {
                        name: Ident(self.fold_symbol(p.name.0, new_ast)),
                        ty: p.ty.clone(),
                    })
                    .collect(),
                body: self.fold_node(m.body.into(), new_ast).into(),
            }),
            LangNodeKind::Comptime(n) => {
                if let Some(val) = self.evaluate((*n).into()) {
                    LangNodeKind::Expr(val.to_expr(new_ast))
                } else {
                    LangNodeKind::Comptime(self.fold_node((*n).into(), new_ast).into())
                }
            }
            LangNodeKind::ExtendDecl {
                target_type,
                methodes,
            } => LangNodeKind::ExtendDecl {
                target_type: target_type.clone(),
                methodes: methodes
                    .iter()
                    .map(|&n| self.fold_node(n.into(), new_ast).into())
                    .collect(),
            },
            LangNodeKind::ContextDecl { name, extensions } => LangNodeKind::ContextDecl {
                name: Ident(self.fold_symbol(name.0, new_ast)),
                extensions: extensions
                    .iter()
                    .map(|&n| self.fold_node(n.into(), new_ast).into())
                    .collect(),
            },
            LangNodeKind::RequireContextDecl { name, items } => LangNodeKind::RequireContextDecl {
                name: Ident(self.fold_symbol(name.0, new_ast)),
                items: items
                    .iter()
                    .map(|&n| self.fold_node(n.into(), new_ast).into())
                    .collect(),
            },

            LangNodeKind::ImportDecl {
                path,
                context_mappings,
            } => LangNodeKind::ImportDecl {
                path: path.clone(),
                context_mappings: context_mappings
                    .iter()
                    .map(|(k, v)| {
                        (
                            Ident(self.fold_symbol(k.0, new_ast)),
                            Ident(self.fold_symbol(v.0, new_ast)),
                        )
                    })
                    .collect(),
            },
            LangNodeKind::InContext { context_name, body } => LangNodeKind::InContext {
                context_name: Ident(self.fold_symbol(context_name.0, new_ast)),
                body: self.fold_node((*body).into(), new_ast).into(),
            },
        };
        let descriptor = if let Some(desc) = self.ast.get_descriptor(node_id) {
            new_ast.add_descriptor(desc.clone())
        } else {
            new_ast.add_descriptor(popper_semantic_analyzer::hir::NodeDescriptor::default())
        };
        new_ast.add(HirNode {
            node: LangNode {
                kind: new_kind,
                span,
            },
            descriptor,
        })
    }

    fn fold_expr(&mut self, expr: &Expr, new_ast: &mut Hir) -> Expr {
        match expr {
            Expr::Ident(id) => {
                if let Some(val) = self.constants.get(&id.0) {
                    return val.to_expr(new_ast);
                }
                Expr::Ident(Ident(self.fold_symbol(id.0, new_ast)))
            }
            Expr::Int(i) => Expr::Int(*i),
            Expr::Float(f) => Expr::Float(*f),
            Expr::String(s) => Expr::String(s.clone()),
            Expr::Char(c) => Expr::Char(*c),
            Expr::Bool(b) => Expr::Bool(*b),
            Expr::UnaryOp(kind, operand) => {
                Expr::UnaryOp(*kind, self.fold_node((*operand).into(), new_ast).into())
            }
            Expr::Add(l, r) => Expr::Add(
                self.fold_node((*l).into(), new_ast).into(),
                self.fold_node((*r).into(), new_ast).into(),
            ),
            Expr::Sub(l, r) => Expr::Sub(
                self.fold_node((*l).into(), new_ast).into(),
                self.fold_node((*r).into(), new_ast).into(),
            ),
            Expr::Mul(l, r) => Expr::Mul(
                self.fold_node((*l).into(), new_ast).into(),
                self.fold_node((*r).into(), new_ast).into(),
            ),
            Expr::Div(l, r) => Expr::Div(
                self.fold_node((*l).into(), new_ast).into(),
                self.fold_node((*r).into(), new_ast).into(),
            ),
            Expr::Eq(l, r) => Expr::Eq(
                self.fold_node((*l).into(), new_ast).into(),
                self.fold_node((*r).into(), new_ast).into(),
            ),
            Expr::Lt(l, r) => Expr::Lt(
                self.fold_node((*l).into(), new_ast).into(),
                self.fold_node((*r).into(), new_ast).into(),
            ),
            Expr::LtEq(l, r) => Expr::LtEq(
                self.fold_node((*l).into(), new_ast).into(),
                self.fold_node((*r).into(), new_ast).into(),
            ),
            Expr::Gt(l, r) => Expr::Gt(
                self.fold_node((*l).into(), new_ast).into(),
                self.fold_node((*r).into(), new_ast).into(),
            ),
            Expr::GtEq(l, r) => Expr::GtEq(
                self.fold_node((*l).into(), new_ast).into(),
                self.fold_node((*r).into(), new_ast).into(),
            ),
            Expr::Ref(n) => Expr::Ref(self.fold_node((*n).into(), new_ast).into()),
            Expr::Deref(n) => Expr::Deref(self.fold_node((*n).into(), new_ast).into()),
            Expr::FieldAccess { base, field } => Expr::FieldAccess {
                base: self.fold_node((*base).into(), new_ast).into(),
                field: Ident(self.fold_symbol(field.0, new_ast)),
            },
            Expr::TypeDeclInstance { type_name, fields } => Expr::TypeDeclInstance {
                type_name: Ident(self.fold_symbol(type_name.0, new_ast)),
                fields: fields
                    .iter()
                    .map(|(name, val)| {
                        (
                            Ident(self.fold_symbol(name.0, new_ast)),
                            self.fold_node((*val).into(), new_ast).into(),
                        )
                    })
                    .collect(),
            },
            Expr::List(elements) => Expr::List(
                elements
                    .iter()
                    .map(|&e| self.fold_node(e.into(), new_ast).into())
                    .collect(),
            ),
            Expr::Index { base, index } => Expr::Index {
                base: self.fold_node((*base).into(), new_ast).into(),
                index: self.fold_node((*index).into(), new_ast).into(),
            },
            Expr::BuiltinCall { name, args } => {
                if let Some(b) = self.builtins.get(name) {
                    let res = b(self, args.iter().map(|x| HirNodeId::from(*x)).collect());
                    if let Some(val) = res {
                        return val.to_expr(new_ast);
                    }
                }
                Expr::BuiltinCall {
                    name: name.clone(),
                    args: args
                        .iter()
                        .map(|&a| self.fold_node(a.into(), new_ast).into())
                        .collect(),
                }
            }
            Expr::Type(t) => Expr::Type(t.clone()),
        }
    }

    fn fold_symbol(&mut self, id: SymbolId, new_ast: &mut Hir) -> SymbolId {
        if let Some(&new_id) = self.symbol_map.get(&id) {
            return new_id;
        }
        let name = self.ast.get_symbol(id).name.clone();
        let new_id = new_ast.add_symbol(&name);
        self.symbol_map.insert(id, new_id);
        new_id
    }
}

pub struct CFTELayer;

impl Layer for CFTELayer {
    type Inner = Hir;
    type Output = Hir;
    fn handle(&mut self, ast: &Self::Inner, _node: HirNodeId) -> Self::Output {
        let mut evaluator = Evaluator::new(ast);
        evaluator.fold()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use popper_ast::ast::{Ident, LangAst, LangNode, LangNodeKind, Span};
    use popper_ast::layer::Ast;
    use popper_semantic_analyzer::hir::HirNodeId;

    #[test]
    fn test_eval_add() {
        let mut ast = LangAst::new();
        let i1 = ast.add(LangNode {
            kind: LangNodeKind::Expr(Expr::Int(1)),
            span: Span::new(0, 0),
        });
        let i2 = ast.add(LangNode {
            kind: LangNodeKind::Expr(Expr::Int(2)),
            span: Span::new(0, 0),
        });
        let add = ast.add(LangNode {
            kind: LangNodeKind::Expr(Expr::Add(i1, i2)),
            span: Span::new(0, 0),
        });

        let hir = popper_semantic_analyzer::hir::Hir::create_from_ast(&ast);
        let mut eval = Evaluator::new(&hir);
        assert_eq!(eval.evaluate(add.into()), Some(Value::Int(3)));
    }

    #[test]
    fn test_eval_const() {
        let mut ast = LangAst::new();
        let sym_x = ast.add_symbol("x");
        let i5 = ast.add(LangNode {
            kind: LangNodeKind::Expr(Expr::Int(5)),
            span: Span::new(0, 0),
        });
        let c = ast.add(LangNode {
            kind: LangNodeKind::Const(popper_ast::ast::Const {
                name: Ident(sym_x),
                value: i5,
                ty: None,
            }),
            span: Span::new(0, 0),
        });

        let id_x = ast.add(LangNode {
            kind: LangNodeKind::Expr(Expr::Ident(Ident(sym_x))),
            span: Span::new(0, 0),
        });

        let hir = popper_semantic_analyzer::hir::Hir::create_from_ast(&ast);
        let mut eval = Evaluator::new(&hir);
        eval.evaluate(c.into());
        assert_eq!(eval.evaluate(id_x.into()), Some(Value::Int(5)));
    }

    #[test]
    fn test_eval_sizeof() {
        let mut ast = LangAst::new();
        let ty_int = ast.add(LangNode {
            kind: LangNodeKind::Expr(Expr::Type(popper_ast::type_::Type::Int)),
            span: Span::new(0, 0),
        });
        let sizeof = ast.add(LangNode {
            kind: LangNodeKind::Expr(Expr::BuiltinCall {
                name: "sizeOf".to_string(),
                args: vec![ty_int],
            }),
            span: Span::new(0, 0),
        });

        let hir = popper_semantic_analyzer::hir::Hir::create_from_ast(&ast);
        let mut eval = Evaluator::new(&hir);
        assert_eq!(eval.evaluate(sizeof.into()), Some(Value::Int(4)));
    }

    #[test]
    fn test_eval_typeof() {
        let mut ast = LangAst::new();
        let i5 = ast.add(LangNode {
            kind: LangNodeKind::Expr(Expr::Int(5)),
            span: Span::new(0, 0),
        });
        let typeof_ = ast.add(LangNode {
            kind: LangNodeKind::Expr(Expr::BuiltinCall {
                name: "typeOf".to_string(),
                args: vec![i5],
            }),
            span: Span::new(0, 0),
        });

        let hir = popper_semantic_analyzer::hir::Hir::create_from_ast(&ast);
        let mut eval = Evaluator::new(&hir);
        assert_eq!(
            eval.evaluate(typeof_.into()),
            Some(Value::Type(popper_ast::type_::Type::Int))
        );
    }

    #[test]
    fn test_custom_builtin() {
        let mut ast = LangAst::new();
        let i10 = ast.add(LangNode {
            kind: LangNodeKind::Expr(Expr::Int(10)),
            span: Span::new(0, 0),
        });
        let call = ast.add(LangNode {
            kind: LangNodeKind::Expr(Expr::BuiltinCall {
                name: "double".to_string(),
                args: vec![i10],
            }),
            span: Span::new(0, 0),
        });

        let hir = popper_semantic_analyzer::hir::Hir::create_from_ast(&ast);
        let mut eval = Evaluator::new(&hir);
        eval.register_builtin("double".to_string(), |eval, args| {
            let val = eval.evaluate(*args.get(0)?)?;
            match val {
                Value::Int(i) => Some(Value::Int(i * 2)),
                _ => None,
            }
        });

        assert_eq!(eval.evaluate(call.into()), Some(Value::Int(20)));
    }

    #[test]
    fn test_layer_trait() {
        let mut ast = LangAst::new();
        let i1 = ast.add(LangNode {
            kind: LangNodeKind::Expr(Expr::Int(1)),
            span: Span::new(0, 0),
        });
        let i2 = ast.add(LangNode {
            kind: LangNodeKind::Expr(Expr::Int(2)),
            span: Span::new(0, 0),
        });
        let add = ast.add(LangNode {
            kind: LangNodeKind::Expr(Expr::Add(i1, i2)),
            span: Span::new(0, 0),
        });
        ast.set_root(add);

        let hir = popper_semantic_analyzer::hir::Hir::create_from_ast(&ast);
        let mut eval = Evaluator::new(&hir);
        let result = hir.apply_layer(&mut eval);
        assert_eq!(result, Some(Value::Int(3)));
    }

    #[test]
    fn test_fold_comptime() {
        let mut ast = LangAst::new();
        let i1 = ast.add(LangNode {
            kind: LangNodeKind::Expr(Expr::Int(1)),
            span: Span::new(0, 0),
        });
        let i2 = ast.add(LangNode {
            kind: LangNodeKind::Expr(Expr::Int(2)),
            span: Span::new(0, 0),
        });
        let add = ast.add(LangNode {
            kind: LangNodeKind::Expr(Expr::Add(i1, i2)),
            span: Span::new(0, 0),
        });
        let comptime = ast.add(LangNode {
            kind: LangNodeKind::Comptime(add),
            span: Span::new(0, 0),
        });
        ast.set_root(comptime);

        let hir = popper_semantic_analyzer::hir::Hir::create_from_ast(&ast);
        let mut eval = Evaluator::new(&hir);
        let folded_ast = eval.fold();

        let root = folded_ast.get(folded_ast.root());
        match &root.node.kind {
            LangNodeKind::Expr(Expr::Int(3)) => {}
            _ => panic!("Expected Expr::Int(3), got {:?}", root.node.kind),
        }
    }

    #[test]
    fn test_fold_const_replacement() {
        let mut ast = LangAst::new();
        let sym_x = ast.add_symbol("X");
        let i10 = ast.add(LangNode {
            kind: LangNodeKind::Expr(Expr::Int(10)),
            span: Span::new(0, 0),
        });
        let c = ast.add(LangNode {
            kind: LangNodeKind::Const(Const {
                name: Ident(sym_x),
                value: i10,
                ty: None,
            }),
            span: Span::new(0, 0),
        });

        let id_x = ast.add(LangNode {
            kind: LangNodeKind::Expr(Expr::Ident(Ident(sym_x))),
            span: Span::new(0, 0),
        });
        let i5 = ast.add(LangNode {
            kind: LangNodeKind::Expr(Expr::Int(5)),
            span: Span::new(0, 0),
        });
        let add = ast.add(LangNode {
            kind: LangNodeKind::Expr(Expr::Add(id_x, i5)),
            span: Span::new(0, 0),
        });

        let block = ast.add(LangNode {
            kind: LangNodeKind::Block(vec![c, add]),
            span: Span::new(0, 0),
        });
        ast.set_root(block);

        let hir = popper_semantic_analyzer::hir::Hir::create_from_ast(&ast);
        let mut eval = Evaluator::new(&hir);
        let folded_ast = eval.fold();

        let root = folded_ast.get(folded_ast.root());
        let root = &root.node;
        if let LangNodeKind::Block(nodes) = &root.kind {
            let add_node = folded_ast.get(HirNodeId::from(nodes[1]));
            if let LangNodeKind::Expr(Expr::Add(l, r)) = &add_node.node.kind {
                let ln = folded_ast.get(HirNodeId::from(*l));
                let rn = folded_ast.get(HirNodeId::from(*r));
                assert!(matches!(ln.node.kind, LangNodeKind::Expr(Expr::Int(10))));
                assert!(matches!(rn.node.kind, LangNodeKind::Expr(Expr::Int(5))));
            } else {
                panic!("Expected Add node, got {:?}", add_node.node.kind);
            }
        } else {
            panic!("Expected Block, got {:?}", root.kind);
        }
    }

    #[test]
    fn test_macro_expansion() {
        let mut ast = LangAst::new();

        // mcro add(a: int, b: int) { a + b }
        let sym_add = ast.add_symbol("add");
        let sym_a = ast.add_symbol("a");
        let sym_b = ast.add_symbol("b");

        let id_a = ast.add(LangNode {
            kind: LangNodeKind::Expr(Expr::Ident(Ident(sym_a))),
            span: Span::new(0, 0),
        });
        let id_b = ast.add(LangNode {
            kind: LangNodeKind::Expr(Expr::Ident(Ident(sym_b))),
            span: Span::new(0, 0),
        });
        let body = ast.add(LangNode {
            kind: LangNodeKind::Expr(Expr::Add(id_a, id_b)),
            span: Span::new(0, 0),
        });

        let macro_def = ast.add(LangNode {
            kind: LangNodeKind::MacroDef(MacroDef {
                name: Ident(sym_add),
                params: vec![
                    ParamDef {
                        name: Ident(sym_a),
                        ty: popper_ast::type_::Type::Int,
                    },
                    ParamDef {
                        name: Ident(sym_b),
                        ty: popper_ast::type_::Type::Int,
                    },
                ],
                body,
            }),
            span: Span::new(0, 0),
        });

        // let x = add(1, 2);
        let i1 = ast.add(LangNode {
            kind: LangNodeKind::Expr(Expr::Int(1)),
            span: Span::new(0, 0),
        });
        let i2 = ast.add(LangNode {
            kind: LangNodeKind::Expr(Expr::Int(2)),
            span: Span::new(0, 0),
        });
        let func_ident = ast.add(LangNode {
            kind: LangNodeKind::Expr(Expr::Ident(Ident(sym_add))),
            span: Span::new(0, 0),
        });
        let call = ast.add(LangNode {
            kind: LangNodeKind::FunctionCall {
                function: func_ident,
                args: vec![i1, i2],
            },
            span: Span::new(0, 0),
        });

        let sym_x = ast.add_symbol("x");
        let let_x = ast.add(LangNode {
            kind: LangNodeKind::Let(Let {
                name: Ident(sym_x),
                value: call,
            }),
            span: Span::new(0, 0),
        });

        let block = ast.add(LangNode {
            kind: LangNodeKind::Block(vec![macro_def, let_x]),
            span: Span::new(0, 0),
        });
        ast.set_root(block);

        let hir = popper_semantic_analyzer::hir::Hir::create_from_ast(&ast);
        let mut eval = Evaluator::new(&hir);
        let folded_ast = eval.fold();

        let root = folded_ast.get(folded_ast.root());
        let root = &root.node;
        if let LangNodeKind::Block(nodes) = &root.kind {
            let let_node = folded_ast.get(HirNodeId::from(nodes[1]));
            if let LangNodeKind::Let(let_) = &let_node.node.kind {
                let add_node = folded_ast.get(HirNodeId::from(let_.value));
                if let LangNodeKind::Expr(Expr::Add(l, r)) = &add_node.node.kind {
                    let ln = folded_ast.get(HirNodeId::from(*l));
                    let rn = folded_ast.get(HirNodeId::from(*r));
                    assert!(matches!(ln.node.kind, LangNodeKind::Expr(Expr::Int(1))));
                    assert!(matches!(rn.node.kind, LangNodeKind::Expr(Expr::Int(2))));
                } else {
                    panic!("Expected Add node, got {:?}", add_node.node.kind);
                }
            } else {
                panic!("Expected Let node, got {:?}", let_node.node.kind);
            }
        }
    }
}
