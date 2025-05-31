use popper_ast::ast::{Expr, LangNode, LangNodeKind, UnaryOpKind};
use popper_ast::layer::Ast;
use popper_ast::token::TokenKind;
use crate::Parser;

impl Parser {
    pub(crate) fn parse_binary_expr(&mut self) -> crate::error::Result<popper_ast::ast::LangNodeId> {
        let lhs = self.parse_factor()?;
        let mut current = lhs;

        loop {
            let token = self.cursor.peek_token()?;
            match token.kind {
                TokenKind::Plus => {
                    self.cursor.next_token()?; // consume the operator
                    let rhs = self.parse_factor()?;
                    let node = LangNode {
                        kind: LangNodeKind::Expr(
                            Expr::Add(
                                current,
                                rhs,
                            )
                        ),
                        span: token.span.merge(self.cursor.peek_token()?.span),
                    };
                    current = self.ast.add(node);
                },
                TokenKind::Minus => {
                    self.cursor.next_token()?; // consume the operator
                    let rhs = self.parse_factor()?;
                    let node = LangNode {
                        kind: LangNodeKind::Expr(
                            Expr::Sub(
                                current,
                                rhs,
                            )
                        ),
                        span: token.span.merge(self.cursor.peek_token()?.span),
                    };
                    current = self.ast.add(node);
                },
                _ => break, // no more binary operators
            }
        }

        Ok(current)
    }

    pub(crate) fn parse_factor(&mut self) -> crate::error::Result<popper_ast::ast::LangNodeId> {
        let literal = self.parse_unary_expr()?;
        let mut current = literal;

        loop {
            let token = self.cursor.peek_token()?;
            match token.kind {
                TokenKind::Multiply => {
                    self.cursor.next_token()?; // consume the operator
                    let rhs = self.parse_factor()?;
                    let node = LangNode {
                        kind: LangNodeKind::Expr(
                            Expr::Mul(
                                current,
                                rhs,
                            )
                        ),
                        span: token.span.merge(self.cursor.peek_token()?.span),
                    };
                    current = self.ast.add(node);
                },
                TokenKind::Divide => {
                    self.cursor.next_token()?; // consume the operator
                    let rhs = self.parse_factor()?;
                    let node = LangNode {
                        kind: LangNodeKind::Expr(
                            Expr::Div(
                                current,
                                rhs,
                            )
                        ),
                        span: token.span.merge(self.cursor.peek_token()?.span),
                    };
                    current = self.ast.add(node);
                },
                _ => break, // no more binary operators
            }
        }

        Ok(current)
    }
    
    pub(crate) fn parse_unary_expr(&mut self) -> crate::error::Result<popper_ast::ast::LangNodeId> {
        let token = self.cursor.peek_token()?;
        match token.kind {
            TokenKind::Plus => {
                self.cursor.next_token()?; // consume the operator
                let expr = self.parse_unary_expr()?;
                let node = LangNode {
                    kind: LangNodeKind::Expr(
                        Expr::UnaryOp(
                            UnaryOpKind::ArithmeticPlus,
                            expr,
                        )
                    ),
                    span: token.span.merge(self.cursor.peek_token()?.span),
                };
                Ok(self.ast.add(node))
            },
            TokenKind::Minus => {
                self.cursor.next_token()?; // consume the operator
                let expr = self.parse_unary_expr()?;
                let node = LangNode {
                    kind: LangNodeKind::Expr(
                        Expr::UnaryOp(
                            UnaryOpKind::ArithmeticNegate,
                            expr,
                        )
                    ),
                    span: token.span.merge(self.cursor.peek_token()?.span),
                };
                Ok(self.ast.add(node))
            },
            _ => self.parse_function_call(),
        }
    }
}