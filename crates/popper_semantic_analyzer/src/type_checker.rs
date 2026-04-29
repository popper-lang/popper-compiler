use crate::error::SemanticError;
use crate::hir::MathOpKind;
use crate::{LayerOutput, SemanticAnalyzer, SemanticLayer};
use popper_ast::ast::{Expr, LangNodeId, LangNodeKind, Span};
use popper_ast::type_::{Type, TypeInfo};
use popper_index::Idx;

#[derive(Default, Debug, Clone)]
pub struct TypeChecker {}
impl TypeChecker {
    pub fn new() -> Self {
        TypeChecker {}
    }

    pub fn bin_op_result_type(
        op: &str,
        analyzer: &mut SemanticAnalyzer,
        left: LangNodeId,
        right: LangNodeId,
        parent: LangNodeId,
        span: Span,
    ) -> LayerOutput<Type> {
        let left = analyzer.analyze(left)?.unwrap();
        let right = analyzer.analyze(right)?.unwrap();
        match (op, left.clone(), right.clone()) {
            (_, Type::Int, Type::Int) => {
                analyzer
                    .hir
                    .set_math_op_kind(parent.into(), MathOpKind::IntAndInt);
                match op {
                    "+" | "-" | "*" | "/" => LayerOutput::ResOk(Type::Int),
                    "==" | "<" | ">" | "<=" | ">=" => LayerOutput::ResOk(Type::Bool),
                    _ => panic!("Invalid operator for Int: {}", op),
                }
            }
            (_, Type::Float, Type::Float) => {
                analyzer
                    .hir
                    .set_math_op_kind(parent.into(), MathOpKind::FloatAndFloat);
                match op {
                    "+" | "-" | "*" | "/" => LayerOutput::ResOk(Type::Float),
                    "==" | "<" | ">" | "<=" | ">=" => LayerOutput::ResOk(Type::Bool),
                    _ => panic!("Invalid operator for Float: {}", op),
                }
            }
            ("+", Type::Ptr(_), Type::Int) => {
                analyzer
                    .hir
                    .set_math_op_kind(parent.into(), MathOpKind::PtrAndInt);
                LayerOutput::ResOk(left)
            }
            ("+", Type::Int, Type::Ptr(_)) => {
                analyzer
                    .hir
                    .set_math_op_kind(parent.into(), MathOpKind::IntAndPtr);
                LayerOutput::ResOk(left)
            }
            (_, left, right) => LayerOutput::ResErr(SemanticError::type_mismatch(
                "(Int, Int) or (Float, Float)".to_string(),
                format!("({}, {})", left, right),
                span,
            )),
        }
    }
}

impl SemanticLayer for TypeChecker {
    type Output = Type;

    fn handle(
        _layer_id: usize,
        analyzer: &mut SemanticAnalyzer,
        node_id: LangNodeId,
    ) -> LayerOutput<Self::Output> {
        let node = analyzer.ast.get(node_id).clone();
        let res = match node.kind {
            LangNodeKind::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let cond_ty = analyzer.analyze(condition)?.unwrap();
                if cond_ty != Type::Bool {
                    return LayerOutput::ResErr(SemanticError::type_mismatch(
                        "Bool".to_string(),
                        cond_ty.to_string(),
                        node.span,
                    ));
                }
                analyzer.analyze(then_branch)?;
                if let Some(else_branch) = else_branch {
                    analyzer.analyze(else_branch)?;
                }
                LayerOutput::Handled
            }

            LangNodeKind::Expr(Expr::Int(_)) => LayerOutput::ResOk(Type::Int),
            LangNodeKind::Expr(Expr::String(_)) => {
                LayerOutput::ResOk(Type::Ptr(Box::new(Type::Char)))
            }
            LangNodeKind::Expr(Expr::Char(_)) => LayerOutput::ResOk(Type::Char),
            LangNodeKind::Expr(Expr::Bool(_)) => LayerOutput::ResOk(Type::Bool),
            LangNodeKind::Expr(Expr::Type(t)) => LayerOutput::ResOk(Type::Type(Some(TypeInfo {
                builtin_type: false,
                name: String::new(),
                ty: Box::new(t),
            }))),
            LangNodeKind::Expr(Expr::List(elems)) => {
                if elems.is_empty() {
                    todo!("Handle empty list type inference or require explicit type annotation")
                }
                let first_ty = analyzer.analyze(elems[0])?.unwrap();
                for &elem in &elems[1..] {
                    let elem_ty = analyzer.analyze(elem)?.unwrap();
                    if elem_ty != first_ty {
                        let node_elem = analyzer.ast.get(elem);
                        return LayerOutput::ResErr(SemanticError::type_mismatch(
                            first_ty.to_string(),
                            elem_ty.to_string(),
                            node_elem.span,
                        ));
                    }
                }
                LayerOutput::ResOk(Type::List(Box::new(first_ty), elems.len()))
            }
            LangNodeKind::Expr(Expr::Ref(r)) => {
                let ty = analyzer.analyze(r)?.unwrap();
                LayerOutput::ResOk(Type::Ptr(Box::new(ty)))
            }
            LangNodeKind::Expr(Expr::Deref(r)) => {
                let ty = analyzer.analyze(r)?.unwrap();
                if let Type::Ptr(p) = ty {
                    LayerOutput::ResOk(*p)
                } else {
                    LayerOutput::ResErr(SemanticError::type_mismatch(
                        "Ptr".to_string(),
                        ty.to_string(),
                        node.span,
                    ))
                }
            }
            LangNodeKind::Expr(Expr::UnaryOp(_, v)) => {
                let ty = analyzer.analyze(v)?.unwrap();
                match ty {
                    Type::Int | Type::Float => LayerOutput::ResOk(ty),
                    _ => LayerOutput::ResErr(SemanticError::type_mismatch(
                        "Int or Float".to_string(),
                        ty.to_string(),
                        node.span,
                    )),
                }
            }
            LangNodeKind::Expr(Expr::Add(left, right)) => {
                Self::bin_op_result_type("+", analyzer, left, right, node_id, node.span)
            }
            LangNodeKind::Expr(Expr::Sub(left, right)) => {
                Self::bin_op_result_type("-", analyzer, left, right, node_id, node.span)
            }
            LangNodeKind::Expr(Expr::Mul(left, right)) => {
                Self::bin_op_result_type("*", analyzer, left, right, node_id, node.span)
            }
            LangNodeKind::Expr(Expr::Div(left, right)) => {
                Self::bin_op_result_type("/", analyzer, left, right, node_id, node.span)
            }
            LangNodeKind::Expr(Expr::Eq(left, right)) => {
                Self::bin_op_result_type("==", analyzer, left, right, node_id, node.span)
            }
            LangNodeKind::Expr(Expr::Gt(left, right)) => {
                Self::bin_op_result_type(">", analyzer, left, right, node_id, node.span)
            }
            LangNodeKind::Expr(Expr::Lt(left, right)) => {
                Self::bin_op_result_type("<", analyzer, left, right, node_id, node.span)
            }
            LangNodeKind::Expr(Expr::GtEq(left, right)) => {
                Self::bin_op_result_type(">=", analyzer, left, right, node_id, node.span)
            }
            LangNodeKind::Expr(Expr::LtEq(left, right)) => {
                Self::bin_op_result_type("<=", analyzer, left, right, node_id, node.span)
            }
            LangNodeKind::Expr(Expr::FieldAccess { base, field }) => {
                let mut base_ty = analyzer.analyze(base)?.unwrap();
                while let Type::Ptr(inner) = base_ty.clone() {
                    base_ty = *inner;
                }

                // Resolve struct fields from global scope if empty
                let mut override_ty = None;
                if let Type::AggregateType(name, fields, _) = &base_ty {
                    if fields.fields.is_empty() {
                        for layer in &analyzer.layers {
                            if let crate::SemanticLayerKind::SymbolResolver(sr) = layer {
                                if let Some(Type::AggregateType(_, resolved_fields, _)) =
                                    sr.get(analyzer.ast.add_symbol(name))
                                {
                                    override_ty = Some(Type::AggregateType(
                                        name.clone(),
                                        resolved_fields.clone(),
                                        popper_ast::ast::TypeDeclKind::Struct,
                                    ));
                                }
                            }
                        }
                    }
                }
                if let Some(ty) = override_ty {
                    base_ty = ty;
                }

                if let Type::AggregateType(_, fields, _) = base_ty {
                    let field_name = analyzer.ast.get_symbol(field.0).name.clone();
                    if let Some(field_ty) = fields.get_field(&field_name) {
                        LayerOutput::ResOk(field_ty.clone())
                    } else {
                        LayerOutput::ResErr(SemanticError::field_not_found(field_name, node.span))
                    }
                } else if let Type::Type(Some(info)) = base_ty {
                    let field_name = analyzer.ast.get_symbol(field.0).name.clone();

                    // On récupère le vrai nom de la struct encapsulée
                    let type_name = if let Type::AggregateType(name, _, _) = &*info.ty {
                        name.clone()
                    } else {
                        info.ty.to_string()
                    };

                    let mut method_ty = None;
                    for layer in &analyzer.layers {
                        if let crate::SemanticLayerKind::SymbolResolver(sr) = layer {
                            if let Some(methods) = sr.method_table.get(&type_name) {
                                if let Some(ty) = methods.get(&field_name) {
                                    method_ty = Some(ty.clone());
                                }
                            }
                        }
                    }

                    if let Some(ty) = method_ty {
                        LayerOutput::ResOk(ty)
                    } else {
                        LayerOutput::ResErr(SemanticError::field_not_found(
                            format!("static method {}", field_name),
                            node.span,
                        ))
                    }
                } else {
                    LayerOutput::ResErr(SemanticError::type_mismatch(
                        "Struct".to_string(),
                        base_ty.to_string(),
                        node.span,
                    ))
                }
            }
            LangNodeKind::Expr(Expr::Index { base, index }) => {
                let base_ty = analyzer.analyze(base)?.unwrap();
                let index_ty = analyzer.analyze(index)?.unwrap();
                if index_ty != Type::Int {
                    return LayerOutput::ResErr(SemanticError::type_mismatch(
                        "Int".to_string(),
                        index_ty.to_string(),
                        node.span,
                    ));
                }
                if let Type::List(elem_ty, _) = base_ty {
                    LayerOutput::ResOk(*elem_ty)
                } else if let Type::Ptr(inner) = base_ty {
                    LayerOutput::ResOk(*inner)
                } else {
                    LayerOutput::ResErr(SemanticError::type_mismatch(
                        "List or Ptr".to_string(),
                        base_ty.to_string(),
                        node.span,
                    ))
                }
            }
            LangNodeKind::FunctionCall { function, args } => {
                let func_node = analyzer.ast.get(function).clone();
                let mut func_ty = None;
                let mut is_method = false;
                let mut expected_args_offset = 0;

                if let LangNodeKind::Expr(popper_ast::ast::Expr::FieldAccess { base, field }) =
                    &func_node.kind
                {
                    let mut base_ty = analyzer.analyze(*base)?.unwrap();
                    while let Type::Ptr(inner) = base_ty.clone() {
                        base_ty = *inner;
                    }

                    let mut override_ty = None;
                    if let Type::AggregateType(name, fields, _) = &base_ty {
                        if fields.fields.is_empty() {
                            for layer in &analyzer.layers {
                                if let crate::SemanticLayerKind::SymbolResolver(sr) = layer {
                                    if let Some(Type::AggregateType(_, resolved_fields, _)) =
                                        sr.get(analyzer.ast.add_symbol(name))
                                    {
                                        override_ty = Some(Type::AggregateType(
                                            name.clone(),
                                            resolved_fields.clone(),
                                            popper_ast::ast::TypeDeclKind::Struct,
                                        ));
                                    }
                                }
                            }
                        }
                    }
                    if let Some(ty) = override_ty {
                        base_ty = ty;
                    }

                    let field_name = analyzer.ast.get_symbol(field.0).name.clone();

                    if let Type::AggregateType(_, fields, _) = &base_ty {
                        if let Some(field_type) = fields.get_field(&field_name) {
                            func_ty = Some(field_type.clone());
                        }
                    }

                    if func_ty.is_none() {
                        let type_name = if let Type::AggregateType(name, _, _) = &base_ty {
                            name.clone()
                        } else {
                            base_ty.to_string()
                        };

                        for layer in &analyzer.layers {
                            if let crate::SemanticLayerKind::SymbolResolver(sr) = layer {
                                if let Some(methods) = sr.method_table.get(&type_name) {
                                    if let Some(ty) = methods.get(&field_name) {
                                        func_ty = Some(ty.clone());
                                        is_method = true;
                                        expected_args_offset = 1; // self est passé implicitement
                                    }
                                }
                            }
                        }
                    }
                }

                let func_ty = if let Some(t) = func_ty {
                    if is_method {
                        analyzer.hir.set_type(function.into(), t.clone());
                    }
                    t
                } else if is_method {
                    analyzer.analyze(function)?.unwrap()
                } else {
                    analyzer.analyze(function)?.unwrap()
                };

                if let Type::Function(param_types, ret_type, is_vararg, ..) = func_ty {
                    if is_vararg {
                        if args.len() + expected_args_offset < param_types.len() {
                            return LayerOutput::ResErr(SemanticError::argument_count_mismatch(
                                param_types.len(),
                                args.len() + expected_args_offset,
                                node.span,
                            ));
                        }
                    } else {
                        if args.len() + expected_args_offset != param_types.len() {
                            return LayerOutput::ResErr(SemanticError::argument_count_mismatch(
                                param_types.len(),
                                args.len() + expected_args_offset,
                                node.span,
                            ));
                        }
                    }
                    if is_method && expected_args_offset == 1 && !param_types.is_empty() {
                        let param_ty = param_types[0].clone();
                        if let LangNodeKind::Expr(popper_ast::ast::Expr::FieldAccess {
                            base, ..
                        }) = &func_node.kind
                        {
                            let mut base_ty = analyzer.analyze(*base)?.unwrap();
                            let types_match = |t1: &Type, t2: &Type| -> bool {
                                let t1_inner = if let Type::Ptr(inner) = t1 {
                                    &**inner
                                } else {
                                    t1
                                };
                                let t2_inner = if let Type::Ptr(inner) = t2 {
                                    &**inner
                                } else {
                                    t2
                                };
                                match (t1_inner, t2_inner) {
                                    (
                                        Type::AggregateType(n1, _, _),
                                        Type::AggregateType(n2, _, _),
                                    ) => n1 == n2,
                                    _ => t1_inner == t2_inner,
                                }
                            };

                            if !types_match(&base_ty, &param_ty) {
                                return LayerOutput::ResErr(SemanticError::type_mismatch(
                                    param_ty.to_string(),
                                    base_ty.to_string(),
                                    node.span,
                                ));
                            }
                        }
                    }

                    for (i, arg) in args.iter().enumerate() {
                        let arg_ty = analyzer.analyze(*arg)?.unwrap();
                        let param_index = i + expected_args_offset;
                        if param_index < param_types.len() {
                            let param_ty = &param_types[param_index];
                            if arg_ty != *param_ty {
                            let node_arg = analyzer.ast.get(*arg);
                            let mut ok = false;
                            if let Type::Char = param_ty {
                                if let LangNodeKind::Expr(popper_ast::ast::Expr::Int(val)) = node_arg.kind {
                                    if val >= 0 && val <= 255 {
                                        ok = true;
                                        analyzer.hir.set_type((*arg).into(), Type::Char);
                                    }
                                }
                            }
                            if !ok {
                                return LayerOutput::ResErr(SemanticError::type_mismatch(
                                    param_ty.to_string(),
                                    arg_ty.to_string(),
                                    node_arg.span,
                                ));
                            }
                        }
                        }
                    }
                    LayerOutput::ResOk(*ret_type)
                } else {
                    LayerOutput::ResErr(SemanticError::not_a_function(
                        func_ty.to_string(),
                        func_ty,
                        node.span,
                    ))
                }
            }
            LangNodeKind::Assign { lhs, rhs } => {
                let mut lhs_ty = analyzer.analyze(lhs)?.unwrap();
                let mut rhs_ty = analyzer.analyze(rhs)?.unwrap();

                while let Type::Ptr(inner) = lhs_ty.clone() {
                    if lhs_ty == rhs_ty {
                        break;
                    }
                    lhs_ty = *inner;
                }
                while let Type::Ptr(inner) = rhs_ty.clone() {
                    if lhs_ty == rhs_ty {
                        break;
                    }
                    rhs_ty = *inner;
                }

                if lhs_ty != rhs_ty {
                    let node_rhs = analyzer.ast.get(rhs);
                    let mut ok = false;
                    if let Type::Char = lhs_ty {
                        if let LangNodeKind::Expr(popper_ast::ast::Expr::Int(val)) = node_rhs.kind {
                            if val >= 0 && val <= 255 {
                                ok = true;
                                analyzer.hir.set_type(rhs.into(), Type::Char);
                            }
                        }
                    }
                    if !ok {
                        return LayerOutput::ResErr(SemanticError::type_mismatch(
                            lhs_ty.to_string(),
                            rhs_ty.to_string(),
                            node_rhs.span,
                        ));
                    }
                }
                LayerOutput::Handled
            }
            LangNodeKind::While { condition, body } => {
                let cond_ty = analyzer.analyze(condition)?.unwrap();
                if cond_ty != Type::Bool {
                    return LayerOutput::ResErr(SemanticError::type_mismatch(
                        "bool".to_string(),
                        cond_ty.to_string(),
                        node.span,
                    ));
                }
                analyzer.analyze(body)?;
                LayerOutput::Handled
            }
            LangNodeKind::Expr(Expr::BuiltinCall { name, args }) => match name.as_str() {
                "typeOf" => {
                    if args.len() != 1 {
                        return LayerOutput::ResErr(SemanticError::argument_count_mismatch(
                            1,
                            args.len(),
                            node.span,
                        ));
                    }
                    analyzer.analyze(args[0])?;
                    LayerOutput::Handled
                }
                "sizeOf" | "typeName" | "fieldCount" => {
                    if args.len() != 1 {
                        return LayerOutput::ResErr(SemanticError::argument_count_mismatch(
                            1,
                            args.len(),
                            node.span,
                        ));
                    }
                    let arg_ty = analyzer.analyze(args[0])?.unwrap();
                    if !matches!(arg_ty, Type::Type(_)) {
                        return LayerOutput::ResErr(SemanticError::type_mismatch(
                            "Type".to_string(),
                            arg_ty.to_string(),
                            analyzer.ast.get(args[0]).span,
                        ));
                    }
                    LayerOutput::Handled
                }
                "fieldName" | "fieldType" | "fieldOffset" => {
                    if args.len() != 2 {
                        return LayerOutput::ResErr(SemanticError::argument_count_mismatch(
                            2,
                            args.len(),
                            node.span,
                        ));
                    }
                    let arg0_ty = analyzer.analyze(args[0])?.unwrap();
                    if !matches!(arg0_ty, Type::Type(_)) {
                        return LayerOutput::ResErr(SemanticError::type_mismatch(
                            "Type".to_string(),
                            arg0_ty.to_string(),
                            analyzer.ast.get(args[0]).span,
                        ));
                    }
                    let arg1_ty = analyzer.analyze(args[1])?.unwrap();
                    if arg1_ty != Type::Int {
                        return LayerOutput::ResErr(SemanticError::type_mismatch(
                            "Int".to_string(),
                            arg1_ty.to_string(),
                            analyzer.ast.get(args[1]).span,
                        ));
                    }
                    LayerOutput::Handled
                }
                "vaIs" => {
                    if args.len() != 1 {
                        return LayerOutput::ResErr(SemanticError::argument_count_mismatch(
                            1,
                            args.len(),
                            node.span,
                        ));
                    }
                    analyzer.analyze(args[0])?;
                    analyzer.hir.set_type(node_id.into(), Type::Bool);
                    LayerOutput::ResOk(Type::Bool)
                }
                "vaStart" | "vaEnd" => {
                    if args.len() != 1 {
                        return LayerOutput::ResErr(SemanticError::argument_count_mismatch(
                            1,
                            args.len(),
                            node.span,
                        ));
                    }
                    analyzer.analyze(args[0])?;
                    LayerOutput::Handled
                }
                "vaLen" => {
                    if args.len() != 0 {
                        return LayerOutput::ResErr(SemanticError::argument_count_mismatch(
                            0,
                            args.len(),
                            node.span,
                        ));
                    }
                    analyzer.hir.set_type(node_id.into(), Type::Int);
                    LayerOutput::ResOk(Type::Int)
                }
                "vaArg" => {
                    if args.len() != 1 {
                        return LayerOutput::ResErr(SemanticError::argument_count_mismatch(
                            1,
                            args.len(),
                            node.span,
                        ));
                    }
                    let arg_ty = analyzer.analyze(args[0])?.unwrap();
                    if let Type::Type(t) = arg_ty {
                        let t = t.unwrap();
                        analyzer.hir.set_type(node_id.into(), *t.ty);
                    } else {
                        return LayerOutput::ResErr(SemanticError::type_mismatch(
                            "Type".to_string(),
                            arg_ty.to_string(),
                            analyzer.ast.get(args[0]).span,
                        ));
                    }

                    LayerOutput::Handled
                }
                _ => LayerOutput::NotHandled,
            },

            _ => LayerOutput::NotHandled,
        };

        if let LayerOutput::ResOk(ref ty) = res {
            analyzer.hir.set_type(node_id.into(), ty.clone());
        }

        res
    }
}
