use crate::parser::Parser;
use crate::ast::stmt::{Stmt, StmtType};
use crate::ast::expr::{Expr, ExprType, LiteralType};
use crate::lexer::{Token, TokenType};
use crate::errors::{error, ErrorType, Error};

impl Parser {
    pub fn parse_statement(&mut self) -> Stmt {
        self.skip_whitespace();
        let statement = match self.clone().peek().token_type {
            TokenType::LET => self.parse_let_statement(),
            TokenType::IF => self.parse_if_statement(),
            TokenType::WHILE => self.parse_while_statement(),
            TokenType::CONST => self.parse_const_statement(),
            TokenType::LBRACE => self.parse_block_statement(),
            TokenType::FUN => self.function(),
            TokenType::CLASS => self.parse_class_statement(),
            TokenType::USE => self.parse_use_statement(),
            TokenType::IN  => self.parse_import_statement(),
            TokenType::IMPL => self.parse_impl_statement(),
            TokenType::STRUCT => self.parse_struct_statement(),
            _ => self.parse_expression_statement(),
        };

        statement
    }

    fn parse_expression_statement(&mut self) -> Stmt {
        let expr = self.term();
        let ex = expr.clone().extract;
        Stmt::new(StmtType::Expression { expr }, ex, self.clone().body, self.clone().file)
    }

    fn parse_class_statement(&mut self) -> Stmt {
        self.skip_whitespace();
        let first_position = self.current_str;
        self.advance();
        let name = self.identifier();
        self.advance();
        self.expect_token(TokenType::LBRACE);
        let mut methods = vec![];
        while !self.check(TokenType::RBRACE) {
            self.expect_token(TokenType::FUN);
            self.bake_up();
            methods.push(self.function());
        }
        self.expect_token(TokenType::RBRACE);
        Stmt::new(
            StmtType::Class {
                name: match *name.expr_type {
                    ExprType::Ident { ident } => ident.lexeme,
                    _ => unreachable!(),
                },
                methods,
            },
            first_position..self.current_str,
            self.clone().body,
            self.clone().file
        )
    }

    fn parse_struct_statement(&mut self) -> Stmt {
        self.skip_whitespace();
        let first_position = self.current_str;
        self.advance();
        let name = self.identifier();
        self.advance();
        self.expect_token(TokenType::LBRACE);
        let mut fields = vec![];
        while !self.check(TokenType::RBRACE)  {
            let field_name = match *self.identifier().expr_type {
                ExprType::Ident { ident } => ident.lexeme,
                _ => unreachable!(),
            };
            self.advance();
            self.expect_token(TokenType::COLON);
            let field_type = self.parse_type_expression();
            fields.push((field_name, field_type));
            self.advance();
            if !self.check(TokenType::RBRACE) {
                self.expect_token(TokenType::COMMA);
            }

        }

        self.expect_token(TokenType::RBRACE);
        return Stmt::new(
            StmtType::Struct {
                name: match *name.expr_type {
                    ExprType::Ident { ident } => ident.lexeme,
                    _ => unreachable!(),
                },
                fields,
            },
            first_position..self.current_str,
            self.clone().body,
            self.clone().file
        );

    }

    fn parse_let_statement(&mut self) -> Stmt {
        let first_position = self.current_str;
        self.advance();
        let name = self.identifier();
        let type_;
        self.advance();
        if self.match_token(TokenType::COLON) {
            type_ = Some(self.parse_type_expression());
        } else {
            type_ = None;
        }

        let value = if self.match_token(TokenType::ASSIGN) {
            Some(self.term())
        } else {
            None
        };
        Stmt::new(
            StmtType::Let {
                name: match *name.expr_type {
                    ExprType::Ident { ident } => ident,
                    _ => panic!("expected identifier"),
                },
                value,
                mutable: true,
                type_,
            },
            first_position..self.current_str,
            self.clone().body,
            self.clone().file

        )
    }

    fn parse_while_statement(&mut self) -> Stmt {
        self.skip_whitespace();
        let first_position = self.current_str;
        self.advance();
        let condition = self.term();
        let body = self.parse_block_statement();
        Stmt::new(
            StmtType::While {
                cond: condition,
                body,
            },
            first_position..self.current_str,
            self.clone().body,
            self.clone().file
        )
    }

    fn parse_use_statement(&mut self) -> Stmt {
        self.skip_whitespace();
        let first_position = self.current_str;
        self.advance();
        let name = self.primary();
        if let ExprType::Literal { literal } = *name.expr_type {
            if let LiteralType::String(value) = literal {
                self.expect_token(TokenType::AS);
                if let ExprType::Ident { ident } = *self.identifier().expr_type {
                    self.advance();
                    return Stmt::new(
                        StmtType::Use {
                            path: value,
                            as_: ident.lexeme,
                        },
                        first_position..self.current_str,
                        self.clone().body,
                        self.clone().file
                    );
                } else {
                    panic!("expected identifier")
                }
            } else {
                panic!("expected string")
            }
        } else {
            panic!("expected string")
        }

    }

    fn parse_const_statement(&mut self) -> Stmt {
        self.skip_whitespace();
        let first_position = self.current_str;
        self.advance();
        let name = self.identifier();
        self.advance();
        let type_;
        if self.match_token(TokenType::COLON) {
            type_ = Some(self.term());
        } else {
            type_ = None;
        }
        let value = if self.match_token(TokenType::ASSIGN) {
            Some(self.term())
        } else {
            None
        };
        Stmt::new(
            StmtType::Let {
                name: match *name.expr_type {
                    ExprType::Ident { ident } => ident,
                    _ => panic!("expected identifier"),
                },
                value,
                mutable: false,
                type_,
            },
            first_position..self.current_str,
            self.clone().body,
            self.clone().file
        )
    }

    fn parse_if_statement(&mut self) -> Stmt {

        self.skip_whitespace();
        let first_position = self.current_str;
        self.advance();
        let condition = self.term();
        println!("{:?}", condition);
        let then_branch = self.parse_block_statement();

        if self.match_token(TokenType::ELSE) {
            Stmt::new(
                StmtType::IfElse {
                    cond: condition,
                    then: then_branch,
                    else_: self.parse_block_statement(),
                },
                first_position..self.current_str,
                self.clone().body,
                self.clone().file
            )
        } else {
            Stmt::new(
                StmtType::If {
                    cond: condition,
                    then: then_branch,
                },
                first_position..self.current_str,
                self.clone().body,
                self.clone().file
            )
        }
    }

    fn parse_block_statement(&mut self) -> Stmt {
        self.skip_whitespace();
        let first_position = self.current_str;
        self.advance();
        let mut statements = vec![];

        while !self.match_token(TokenType::RBRACE) {
            statements.push(self.parse_statement());
        }
        Stmt::new(
            StmtType::Block { body: statements },
            first_position..self.current_str,
            self.clone().body,
            self.clone().file
        )
    }

    fn parse_import_statement(&mut self) -> Stmt {
        self.skip_whitespace();
        let first_position = self.current_str;
        self.advance();

        let ident = self.identifier();
        self.advance();
        self.expect_token(TokenType::IMPORT);
        let mut list_ident = vec![match *self.identifier().expr_type {
            ExprType::Ident { ident } => ident.lexeme,
            _ => unreachable!()
        }];
        self.advance();
        while self.match_token(TokenType::COMMA) {
            list_ident.push(match *self.identifier().expr_type {
                ExprType::Ident { ident } => ident.lexeme,
                _ => unreachable!()
            });
            self.advance();
        }

        Stmt::new(
            StmtType::Import {
                name: match *ident.expr_type {
                    ExprType::Ident { ident } => ident.lexeme,
                    _ => panic!("expected identifier"),
                },
                imports: list_ident,
            },
            first_position..self.current_str,
            self.clone().body,
            self.clone().file
        )
    }

    fn parse_impl_statement(&mut self) -> Stmt {
        self.skip_whitespace();
        let first_position = self.current_str;
        self.advance();
        let name = match *self.identifier().expr_type {
            ExprType::Ident { ident } => ident.lexeme,
            _ => panic!("expected identifier"),
        };
        self.advance();
        self.expect_token(TokenType::LBRACE);
        let mut methods = vec![];
        while !self.match_token(TokenType::RBRACE) {
            methods.push(self.function());
        }
        return Stmt {
            stmt_type: Box::new(StmtType::Impl { struct_name: name, methods }),
            extract: first_position..self.current_str,
            body: self.clone().body,
            file: self.clone().file
        };
    }

    fn function(&mut self) -> Stmt {
        self.skip_whitespace();
        let first_position = self.current_str;
        self.advance();
        let name = match *self.identifier().expr_type {
            ExprType::Ident { ident } => ident,
            _ => panic!("Expected identifier"),
        };
        self.advance();
        self.expect_token(TokenType::LPAREN);
        let mut args = Vec::new();
        if !self.check(TokenType::RPAREN) {
            args.push(match *self.identifier().expr_type {
                ExprType::Ident {
                    ident: Token { lexeme: e, .. },
                } => e.to_string(),
                _ => {
                    unreachable!()
                }
            });
            self.advance();
            while self.match_token(TokenType::COMMA) {
                args.push(match *self.identifier().expr_type {
                    ExprType::Ident {
                        ident: Token { lexeme: e, .. },
                    } => e.to_string(),
                    _ => {
                        unreachable!()
                    }
                });
                self.advance();
            }
        }
        self.expect_token(TokenType::RPAREN);
        let body = self.parse_statement();
        Stmt::new(
            StmtType::Function { name, args, body },
            first_position..self.current_str,
            self.clone().body,
            self.clone().file
        )
    }

}