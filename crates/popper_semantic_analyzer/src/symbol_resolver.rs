use crate::error::{SemanticError, TypeMismatch};
use crate::{LayerOutput, SemanticAnalyzer, SemanticLayer};
use popper_ast::ast::{Expr, LangNodeId, LangNodeKind, Span, SymbolId, TypeDeclKind};
use popper_ast::attribute::Attribute;
use popper_ast::type_::{Fields, Type, TypeInfo};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SymbolStorage {
    pub id: SymbolId,
    pub ty: Type,
    pub span: Span,
    pub used_count: usize,
    pub def_at: Option<LangNodeId>,
    pub is_type: bool,
}

impl SymbolStorage {
    pub fn new(id: SymbolId, ty: Type, span: Span) -> Self {
        SymbolStorage {
            id,
            ty,
            span,
            used_count: 0,
            def_at: None,
            is_type: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Scope {
    parent: Option<Box<Scope>>,
    symbols: Vec<SymbolStorage>,
}

impl Default for Scope {
    fn default() -> Self {
        Scope {
            parent: None,
            symbols: Vec::new(),
        }
    }
}

impl Scope {
    pub fn new_root() -> Self {
        Scope {
            parent: None,
            symbols: Vec::new(),
        }
    }

    pub fn new(parent: Scope) -> Self {
        Scope {
            parent: Some(Box::new(parent)),
            symbols: Vec::new(),
        }
    }

    pub fn create_child(&self) -> Self {
        Scope::new(self.clone())
    }

    pub fn insert(&mut self, id: SymbolId, ty: Type, span: Span, is_type: bool) {
        self.symbols.push(SymbolStorage {
            id,
            ty,
            span,
            used_count: 0,
            def_at: None,
            is_type,
        });
    }

    pub fn get(&self, id: SymbolId) -> Option<&Type> {
        for symbol in &self.symbols {
            if symbol.id == id {
                return Some(&symbol.ty);
            }
        }
        self.parent.as_ref().and_then(|p| p.get(id))
    }

    pub fn get_mut(&mut self, id: SymbolId) -> Option<&mut SymbolStorage> {
        for symbol in &mut self.symbols {
            if symbol.id == id {
                return Some(symbol);
            }
        }
        self.parent.as_mut().and_then(|p| p.get_mut(id))
    }

    pub fn get_parent(&self, n: usize) -> Option<&Scope> {
        let mut current = self;
        for _ in 0..n {
            if let Some(parent) = &current.parent {
                current = parent;
            } else {
                return None;
            }
        }
        Some(current)
    }
}

#[derive(Debug, Clone)]
pub struct StructInfo {
    pub name: String,
    pub fields: Vec<(String, u32)>,
}

#[derive(Default, Debug, Clone)]
pub struct SymbolResolver {
    global_scope: Scope,
    current_scope_idx: usize,
    expected_ret_ty: Option<Type>,
    struct_infos: Vec<StructInfo>,
    pub method_table: HashMap<String, HashMap<String, Type>>,
    pub context_method_table: HashMap<String, HashMap<String, HashMap<String, Type>>>,
    pub current_self_ty: Option<Type>,
}

impl SymbolResolver {
    pub fn new() -> SymbolResolver {
        SymbolResolver {
            global_scope: Scope::new_root(),
            current_scope_idx: 0,
            expected_ret_ty: None,
            struct_infos: Vec::new(),
            method_table: HashMap::new(),
            context_method_table: HashMap::new(),
            current_self_ty: None,
        }
    }

    pub fn enter_scope(&mut self) {
        self.current_scope_idx += 1;
        self.global_scope = self.global_scope.create_child();
    }

    pub fn exit_scope(&mut self) {
        if self.current_scope_idx > 0 {
            self.current_scope_idx -= 1;
            if let Some(parent) = &self.global_scope.parent {
                self.global_scope = *parent.clone();
            }
        }
    }

    pub fn insert(&mut self, id: SymbolId, ty: Type, span: Span, is_type: bool) {
        self.global_scope.insert(id, ty, span, is_type);
    }

    pub fn get(&self, id: SymbolId) -> Option<&Type> {
        self.global_scope.get(id)
    }

    pub fn get_mut(&mut self, id: SymbolId) -> Option<&mut SymbolStorage> {
        self.global_scope.get_mut(id)
    }

    pub fn get_struct_info(&self, name: &str) -> Option<&StructInfo> {
        self.struct_infos.iter().find(|info| info.name == name)
    }

    pub fn get_current_scope_depth(&self) -> usize {
        self.current_scope_idx
    }

    pub fn always_return(&self, node_id: LangNodeId, analyzer: &SemanticAnalyzer) -> bool {
        let node = analyzer.ast.get(node_id).clone();
        match node.kind {
            LangNodeKind::Return(_) => true,
            LangNodeKind::Block(elts) => elts.iter().any(|elt| self.always_return(*elt, analyzer)),
            LangNodeKind::If {
                then_branch,
                else_branch,
                ..
            } => {
                self.always_return(then_branch, analyzer)
                    && else_branch.map_or(false, |eb| self.always_return(eb, analyzer))
            }
            _ => false,
        }
    }

    pub fn mark_unreachable_code(&self, node_id: LangNodeId, analyzer: &mut SemanticAnalyzer) {
        let node = analyzer.ast.get(node_id).clone();
        match node.kind {
            LangNodeKind::Block(elts) => {
                let mut found_return = false;
                for elt in elts {
                    if found_return {
                        analyzer.hir.mark_unreachable(elt.into());
                    }
                    if self.always_return(elt, analyzer) {
                        found_return = true;
                    }
                }
            }
            _ => {}
        }
    }
}

impl Iterator for SymbolResolver {
    type Item = Scope;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_scope_idx == 0 {
            return None;
        }
        let scope = self.global_scope.get_parent(self.current_scope_idx);
        scope.cloned()
    }
}

impl SemanticLayer for SymbolResolver {
    type Output = Type;

    fn handle(
        layer_id: usize,
        analyzer: &mut SemanticAnalyzer,
        node: LangNodeId,
    ) -> LayerOutput<Self::Output> {
        let lang_node = analyzer.ast.get(node).clone();
        match lang_node.kind {
            LangNodeKind::Expr(Expr::Ident(id)) => {
                if let Some(ss) = analyzer.layers[layer_id]
                    .symbol_resolver_mut()
                    .get_mut(id.0)
                {
                    let ty = if ss.is_type {
                        let name = if let Type::AggregateType(name, _, _) = &ss.ty {
                            name.clone()
                        } else {
                            ss.ty.to_string()
                        };
                        Type::Type(Some(TypeInfo {
                            builtin_type: false,
                            name,
                            ty: Box::new(ss.ty.clone()),
                        }))
                    } else {
                        ss.ty.clone()
                    };
                    analyzer.hir.incr_used(node.into());
                    analyzer.hir.set_type(node.into(), ty.clone());
                    ss.used_count += 1; // Increment usage count
                    LayerOutput::ResOk(ty)
                } else {
                    dbg!(
                        &analyzer.layers[layer_id]
                            .symbol_resolver_mut()
                            .current_scope_idx
                    );
                    LayerOutput::ResErr({
                        let symbol = analyzer.ast.get_symbol(id.0);
                        SemanticError::symbol_not_found(symbol.name.clone(), lang_node.span)
                    })
                }
            }
            LangNodeKind::Expr(Expr::TypeDeclInstance { type_name, fields }) => {
                if let Some(ss) = analyzer.layers[layer_id]
                    .symbol_resolver()
                    .clone()
                    .get(type_name.0)
                {
                    if let Type::AggregateType(_, field_types, TypeDeclKind::Struct) = &ss {
                        for (field_name, field_value) in fields {
                            let field_ty = analyzer.analyze(field_value)?.unwrap();
                            if let Some(expected_ty) =
                                field_types.get_field(&analyzer.ast.get_symbol(field_name.0).name)
                            {
                                if *expected_ty != field_ty {
                                    return LayerOutput::ResErr(SemanticError::type_mismatch(
                                        expected_ty.to_string(),
                                        field_ty.to_string(),
                                        lang_node.span,
                                    ));
                                }
                            } else {
                                return LayerOutput::ResErr(SemanticError::field_not_found(
                                    analyzer.ast.get_symbol(field_name.0).name.clone(),
                                    lang_node.span,
                                ));
                            }
                        }
                        analyzer.hir.set_type(node.into(), ss.clone());
                        LayerOutput::ResOk(ss.clone())
                    } else if let Type::AggregateType(_, field_types, TypeDeclKind::Union) = &ss {
                        if fields.len() != 1 {
                            return LayerOutput::ResErr(SemanticError::type_mismatch(
                                "Union with exactly one field".to_string(),
                                format!("Union with {} fields", fields.len()),
                                lang_node.span,
                            ));
                        }
                        let (field_name, field_value) = &fields[0];
                        let field_ty = analyzer.analyze(*field_value)?.unwrap();
                        if let Some(expected_ty) =
                            field_types.get_field(&analyzer.ast.get_symbol(field_name.0).name)
                        {
                            if *expected_ty != field_ty {
                                return LayerOutput::ResErr(SemanticError::type_mismatch(
                                    expected_ty.to_string(),
                                    field_ty.to_string(),
                                    lang_node.span,
                                ));
                            }
                        } else {
                            return LayerOutput::ResErr(SemanticError::field_not_found(
                                analyzer.ast.get_symbol(field_name.0).name.clone(),
                                lang_node.span,
                            ));
                        }
                        analyzer.hir.set_type(node.into(), ss.clone());
                        LayerOutput::ResOk(ss.clone())
                    } else {
                        LayerOutput::ResErr(SemanticError::type_mismatch(
                            "Struct".to_string(),
                            ss.to_string(),
                            lang_node.span,
                        ))
                    }
                } else {
                    LayerOutput::ResErr({
                        let symbol = analyzer.ast.get_symbol(type_name.0);
                        SemanticError::symbol_not_found(symbol.name.clone(), lang_node.span)
                    })
                }
            }
            LangNodeKind::Let(l) => {
                let ty = analyzer.analyze(l.value)?.unwrap();
                let (stored_ty, is_type) = if let Type::Type(Some(info)) = &ty {
                    ((*info.ty).clone(), true)
                } else {
                    (ty.clone(), false)
                };
                analyzer.layers[layer_id].symbol_resolver_mut().insert(
                    l.name.0,
                    stored_ty,
                    lang_node.span,
                    is_type,
                );
                analyzer.hir.set_type(node.into(), ty.clone());
                LayerOutput::ResOk(ty)
            }
            LangNodeKind::FunctionDef {
                name,
                params,
                ret,
                body,
                is_vararg,
                attrs,
                ..
            } => {
                let mut param_types = Vec::new();
                fn resolve_ty(ty: &Type, resolver: &SymbolResolver) -> Type {
                    match ty {
                        Type::SelfType => {
                            if let Some(self_ty) = &resolver.current_self_ty {
                                self_ty.clone()
                            } else {
                                ty.clone()
                            }
                        }
                        Type::Ptr(inner) => Type::Ptr(Box::new(resolve_ty(&**inner, resolver))),
                        Type::AggregateType(n, fields, _) if fields.fields.is_empty() => {
                            let mut current = Some(&resolver.global_scope);
                            while let Some(scope) = current {
                                for sym in &scope.symbols {
                                    if let Type::AggregateType(sn, _, _) = &sym.ty {
                                        if sn == n {
                                            return sym.ty.clone();
                                        }
                                    }
                                }
                                current = scope.parent.as_deref();
                            }
                            ty.clone()
                        }
                        _ => ty.clone(),
                    }
                }

                for param in params.clone() {
                    let p_ty = resolve_ty(&param.ty, analyzer.layers[layer_id].symbol_resolver());
                    param_types.push(p_ty);
                }

                let resolved_ret = resolve_ty(&ret, analyzer.layers[layer_id].symbol_resolver());

                let ty = Type::Function(
                    param_types,
                    Box::new(resolved_ret.clone()),
                    is_vararg,
                    attrs.contains(&Attribute::StdCallC),
                    analyzer.layers[layer_id]
                        .symbol_resolver()
                        .current_self_ty
                        .is_some(),
                );

                analyzer.layers[layer_id].symbol_resolver_mut().insert(
                    name.0,
                    ty.clone(),
                    lang_node.span,
                    false,
                );
                analyzer.hir.set_type(node.into(), ty.clone());
                analyzer.layers[layer_id]
                    .symbol_resolver_mut()
                    .expected_ret_ty = Some(resolved_ret.clone());
                if let Some(body) = body {
                    analyzer.layers[layer_id]
                        .symbol_resolver_mut()
                        .enter_scope();
                    for param in params.iter() {
                        let p_ty =
                            resolve_ty(&param.ty, analyzer.layers[layer_id].symbol_resolver());
                        analyzer.layers[layer_id].symbol_resolver_mut().insert(
                            param.name.0,
                            p_ty,
                            lang_node.span,
                            false,
                        );
                    }
                    analyzer.analyze(body)?;
                    analyzer.layers[layer_id]
                        .symbol_resolver()
                        .clone()
                        .mark_unreachable_code(body, analyzer);
                    analyzer.layers[layer_id].symbol_resolver_mut().exit_scope();

                    if !analyzer.layers[layer_id]
                        .symbol_resolver()
                        .clone()
                        .always_return(body, analyzer)
                        && resolved_ret != Type::Void
                    {
                        return LayerOutput::ResErr(SemanticError::return_not_found(
                            lang_node.span,
                        ));
                    }
                }

                LayerOutput::ResOk(ty)
            }
            LangNodeKind::TypeDecl { name, fields, kind } => {
                let mut field_types = Vec::new();
                for params in fields.clone() {
                    let name = analyzer.ast.get_symbol(params.name.0).name.clone();
                    field_types.push((name, params.ty.clone()));
                }
                // check for duplicate field names
                let mut seen = HashMap::new();
                for (field_name, _) in &field_types {
                    if seen.contains_key(field_name) {
                        return LayerOutput::ResErr(SemanticError::duplicate_field(
                            field_name.clone(),
                            lang_node.span,
                        ));
                    }
                    seen.insert(field_name.clone(), true);
                }

                let name_str = analyzer.ast.get_symbol(name.0).name.clone();

                let ty = Type::AggregateType(
                    name_str,
                    Fields {
                        fields: field_types,
                    },
                    kind,
                );

                analyzer.layers[layer_id].symbol_resolver_mut().insert(
                    name.0,
                    ty.clone(),
                    lang_node.span,
                    true,
                );
                analyzer.hir.set_type(node.into(), ty.clone());
                LayerOutput::ResOk(ty)
            }
            LangNodeKind::Return(ret) => {
                if let Some(expected) = &analyzer.layers[layer_id]
                    .symbol_resolver_mut()
                    .clone()
                    .expected_ret_ty
                {
                    let ret_ty = analyzer.analyze(ret)?.unwrap();
                    if ret_ty != *expected {
                        return LayerOutput::ResErr(SemanticError::type_mismatch(
                            expected.to_string(),
                            ret_ty.to_string(),
                            lang_node.span,
                        ));
                    }
                    // Reset after return
                } /* else {
                return LayerOutput::ResErr(SemanticError::return_not_in_function(
                lang_node.span,
                ));
                } */
                LayerOutput::Handled
            }
            LangNodeKind::Block(elts) => {
                analyzer.layers[layer_id]
                    .symbol_resolver_mut()
                    .enter_scope();
                for elt in elts {
                    analyzer.analyze(elt)?;
                }
                analyzer.layers[layer_id].symbol_resolver_mut().exit_scope();
                LayerOutput::Handled
            }
            LangNodeKind::ContextDecl { name, extensions } => {
                let context_name = analyzer.ast.get_symbol(name.0).name.clone();
                for ext in extensions {
                    let (target_type, methodes) = if let LangNodeKind::ExtendDecl {
                        target_type,
                        methodes,
                    } = &analyzer.ast.get(ext).kind
                    {
                        (target_type.clone(), methodes.clone())
                    } else {
                        continue;
                    };

                    {
                        let mut t = target_type.clone();
                        while let Type::Ptr(inner) = t {
                            t = *inner;
                        }
                        let type_name = if let Type::AggregateType(tname, _, _) = &t {
                            tname.clone()
                        } else {
                            t.to_string()
                        };

                        analyzer.layers[layer_id]
                            .symbol_resolver_mut()
                            .enter_scope();
                        let self_ty = if let Type::Ptr(_) = target_type {
                            target_type.clone()
                        } else {
                            Type::Ptr(Box::new(target_type.clone()))
                        };
                        analyzer.layers[layer_id]
                            .symbol_resolver_mut()
                            .current_self_ty = Some(self_ty);
                        for method in &methodes {
                            let meth_ty = analyzer.analyze(*method)?;
                            if let Some(meth_ty) = meth_ty {
                                if let LangNodeKind::FunctionDef { name: fname, .. } =
                                    analyzer.ast.get(*method).kind.clone()
                                {
                                    let method_name = analyzer.ast.get_symbol(fname.0).name.clone();
                                    analyzer.layers[layer_id]
                                        .symbol_resolver_mut()
                                        .context_method_table
                                        .entry(context_name.clone())
                                        .or_default()
                                        .entry(type_name.clone())
                                        .or_default()
                                        .insert(method_name, meth_ty);
                                }
                            }
                        }
                        analyzer.layers[layer_id]
                            .symbol_resolver_mut()
                            .current_self_ty = None;
                        analyzer.layers[layer_id].symbol_resolver_mut().exit_scope();
                    }
                }
                LayerOutput::Handled
            }
            LangNodeKind::RequireContextDecl { name, items } => {
                let context_name = analyzer.ast.get_symbol(name.0).name.clone();
                let mut extend_items = Vec::new();
                let mut other_items = Vec::new();
                for &item in &items {
                    if let LangNodeKind::ExtendDecl {
                        target_type,
                        methodes,
                    } = &analyzer.ast.get(item).kind
                    {
                        extend_items.push((target_type.clone(), methodes.clone()));
                    } else {
                        other_items.push(item);
                    }
                }

                for item in other_items {
                    analyzer.analyze(item)?;
                }

                for (target_type, methodes) in extend_items {
                    let mut t = target_type.clone();
                    while let Type::Ptr(inner) = t {
                        t = *inner;
                    }
                    let type_name = if let Type::AggregateType(tname, _, _) = &t {
                        tname.clone()
                    } else {
                        t.to_string()
                    };

                    analyzer.layers[layer_id]
                        .symbol_resolver_mut()
                        .enter_scope();
                    let self_ty = if let Type::Ptr(_) = target_type {
                        target_type.clone()
                    } else {
                        Type::Ptr(Box::new(target_type.clone()))
                    };
                    analyzer.layers[layer_id]
                        .symbol_resolver_mut()
                        .current_self_ty = Some(self_ty);

                    for method in methodes {
                        let meth_ty = analyzer.analyze(method)?;
                        if let Some(meth_ty) = meth_ty {
                            if let LangNodeKind::FunctionDef { name: fname, .. } =
                                analyzer.ast.get(method).kind.clone()
                            {
                                let method_name = analyzer.ast.get_symbol(fname.0).name.clone();
                                analyzer.layers[layer_id]
                                    .symbol_resolver_mut()
                                    .context_method_table
                                    .entry(context_name.clone())
                                    .or_default()
                                    .entry(type_name.clone())
                                    .or_default()
                                    .insert(method_name, meth_ty);
                            }
                        }
                    }

                    analyzer.layers[layer_id]
                        .symbol_resolver_mut()
                        .current_self_ty = None;
                    analyzer.layers[layer_id].symbol_resolver_mut().exit_scope();
                }
                LayerOutput::Handled
            }
            LangNodeKind::InContext { context_name, body } => {
                let ctx_name_str = analyzer.ast.get_symbol(context_name.0).name.clone();

                // Temporarily expose context methods
                let mut added_methods = Vec::new();
                if let Some(ctx_methods) = analyzer.layers[layer_id]
                    .symbol_resolver()
                    .context_method_table
                    .get(&ctx_name_str)
                    .cloned()
                {
                    for (type_name, methods) in ctx_methods {
                        for (method_name, ty) in methods {
                            let old_ty = analyzer.layers[layer_id]
                                .symbol_resolver_mut()
                                .method_table
                                .entry(type_name.clone())
                                .or_default()
                                .insert(method_name.clone(), ty);
                            added_methods.push((type_name.clone(), method_name, old_ty));
                        }
                    }
                }

                analyzer.analyze(body)?;

                // Cleanup exposed context methods
                for (type_name, method_name, old_ty) in added_methods {
                    if let Some(methods) = analyzer.layers[layer_id]
                        .symbol_resolver_mut()
                        .method_table
                        .get_mut(&type_name)
                    {
                        if let Some(old) = old_ty {
                            methods.insert(method_name, old);
                        } else {
                            methods.remove(&method_name);
                        }
                    }
                }

                LayerOutput::Handled
            }
            LangNodeKind::ExtendDecl { .. } => LayerOutput::Handled,
            LangNodeKind::ImportDecl { .. } => LayerOutput::Handled,

            LangNodeKind::Expr(Expr::BuiltinCall { name, args }) => match name.as_str() {
                "typeOf" | "fieldType" => LayerOutput::ResOk(Type::Type(None)),
                "sizeOf" | "fieldCount" | "fieldOffset" => LayerOutput::ResOk(Type::Int),
                "typeName" | "fieldName" => LayerOutput::ResOk(Type::Ptr(Box::new(Type::Char))),
                "vaStart" | "vaEnd" => LayerOutput::ResOk(Type::Void),
                "vaArg" => {
                    if args.len() == 1 {
                        let ty = analyzer.analyze(args[0])?.unwrap();
                        if let Type::Type(Some(info)) = ty {
                            LayerOutput::ResOk(*info.ty)
                        } else {
                            LayerOutput::ResErr(SemanticError::type_mismatch(
                                "Type".to_string(),
                                ty.to_string(),
                                lang_node.span,
                            ))
                        }
                    } else {
                        LayerOutput::ResErr(SemanticError::argument_count_mismatch(
                            1,
                            args.len(),
                            lang_node.span,
                        ))
                    }
                }
                _ => LayerOutput::NotHandled,
            },
            _ => LayerOutput::NotHandled,
        }
    }
}
