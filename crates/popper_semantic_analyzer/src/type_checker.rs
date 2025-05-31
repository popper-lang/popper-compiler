use popper_ast::ast::{LangAst, Expr, LangNodeId, LangNodeKind};
use popper_ast::layer::Layer;
use popper_ast::type_::Type;
use crate::{LayerOutput, SemanticAnalyzer, SemanticContext, SemanticLayer};
use crate::error::SemanticError;

#[derive(Default, Debug, Clone)]
pub struct TypeChecker {}
impl TypeChecker {
    pub fn new() -> Self {
        TypeChecker {}
    }
}

impl SemanticLayer for TypeChecker {
    type Output = Type;

    fn handle(&mut self, analyzer: &mut SemanticAnalyzer, node: LangNodeId) -> LayerOutput<Self::Output> {
        let node = analyzer.ast.get(node).clone();
        match node.kind {
            LangNodeKind::If { condition, then_branch, else_branch } => {
                let cond_ty = analyzer.analyze(condition)?.unwrap();
                if cond_ty != Type::Bool {
                    return LayerOutput::ResErr(
                        SemanticError::type_mismatch(
                            "Bool".to_string(),
                            cond_ty.to_string(),
                            node.span
                        )
                    );
                }
                analyzer.analyze(then_branch)?.unwrap();
                if let Some(else_branch) = else_branch {
                    analyzer.analyze(else_branch)?.unwrap()
                } else {
                    Type::Void
                };
                LayerOutput::Handled
            },
            LangNodeKind::Block(block) => {
                for stmt in block {
                    analyzer.analyze(stmt)?.unwrap();
                }
                LayerOutput::Handled
            },
            LangNodeKind::Expr(
                Expr::Int(_)
            ) => {
                LayerOutput::ResOk(Type::Int)
            },
            LangNodeKind::Expr(
                Expr::String(_)
            ) => {
                LayerOutput::ResOk(Type::String)
            },
            LangNodeKind::Expr(
                Expr::UnaryOp(_, v)
            ) => {
                let ty = analyzer.analyze(v)?.unwrap();
                match ty {
                    Type::Int | Type::Float => LayerOutput::ResOk(ty),
                    _ => LayerOutput::ResErr(
                        SemanticError::type_mismatch(
                            "Int or Float".to_string(),
                            ty.to_string(),
                            node.span
                        )
                    ),
                }
            }
            LangNodeKind::Expr(
                Expr::Add(left, right)
            ) => {
                let left_ty = analyzer.analyze(left)?.unwrap();
                let right_ty = analyzer.analyze(right)?.unwrap();
                if left_ty == Type::Int && right_ty == Type::Int {
                    LayerOutput::ResOk(Type::Int)
                } else if left_ty == Type::Float && right_ty == Type::Float {
                    LayerOutput::ResOk(Type::Float)
                } else {
                    LayerOutput::ResErr(
                        SemanticError::type_mismatch(
                            "(Int, Int) or (Float, Float)".to_string(),
                            format!("({}, {})", left_ty, right_ty),
                            node.span
                        )
                    )
                }
            },
            LangNodeKind::FunctionCall { function, args } => {
                let func_ty = analyzer.analyze(function)?.unwrap();
                if let Type::Function(param_types, ret_type) = func_ty {
                    if args.len() != param_types.len() {
                        return LayerOutput::ResErr(
                            SemanticError::argument_count_mismatch(
                                param_types.len(),
                                args.len(),
                                node.span
                            )
                        );
                    }
                    for (arg, param_ty) in args.iter().zip(param_types) {
                        let arg_ty = analyzer.analyze(*arg)?.unwrap();
                        if arg_ty != param_ty {
                            return LayerOutput::ResErr(
                                SemanticError::type_mismatch(
                                    param_ty.to_string(),
                                    arg_ty.to_string(),
                                    node.span
                                )
                            );
                        }
                    }
                    LayerOutput::ResOk(*ret_type)
                } else {
                    LayerOutput::ResErr(
                        SemanticError::not_a_function(func_ty.to_string(), func_ty, node.span)
                    )
                }
            }
            
            _ => LayerOutput::NotHandled,
        }
    }
}