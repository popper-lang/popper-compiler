use std::fmt::Debug;
use crate::ast::expr::{Expr, ExprType, LiteralType};
use crate::ast::stmt::StmtType::Expression;
use crate::ast::stmt::{Stmt, StmtType};
use crate::errors::{error, Error, ErrorType};
use crate::lexer::{Token, TokenType};


#[derive(Debug, Clone)]
pub struct Parser {
    tokens: Vec<Token>,
    current: i32,
    current_str: usize,
    is_at_end: bool,
    body: String
}



impl Parser {
    pub fn new(tokens: Vec<Token>, body: String) -> Parser {
        Parser {
            tokens,
            current: 0,
            is_at_end: false,
            current_str: 0,
            body: body.clone()
        }
    }



    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();

        while !self.is_at_end {
            if self.peek().token_type == TokenType::EOF {
                break;
            }

            stmts.push(self.parse_statement());
        }

        stmts
    }
    fn parse_statement(&mut self) -> Stmt {
        self.skip_whitespace();
        //println!("{:?}", self.tokens);
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
        Stmt::new(StmtType::Expression { expr }, ex, self.clone().body)
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
        )
    }

    fn parse_init_struct_expresion(&mut self) -> Expr {
        self.skip_whitespace();
        let first_position = self.current_str;
        self.advance();

        let name = self.identifier();
        self.advance();
        self.expect_token(TokenType::LBRACE);
        let mut fields = vec![];
        while !self.check(TokenType::RBRACE) {
            let field_name = self.identifier();
            self.advance();
            self.expect_token(TokenType::COLON);
            let field_value = self.term();
            fields.push((field_name, field_value));
            if !self.check(TokenType::RBRACE) {
                self.expect_token(TokenType::COMMA);
            }
        }
        self.expect_token(TokenType::RBRACE);
        self.bake_up();

        Expr::new(
            Box::new(ExprType::InitStruct {
                name: name,
                fields: fields,
            }),
            first_position..self.current_str,
            self.clone().body,
        )
    }

    fn parse_type_expression(&mut self) -> Expr {
        self.skip_whitespace();
        let type_ = self.peek();
        let first_position = self.current_str;
        match type_.token_type {
            TokenType::INT_TYPE
            | TokenType::BOOL_TYPE
            | TokenType::STRING_TYPE
            | TokenType::LIST_TYPE => {
                self.current_str += self.peek().lexeme.len();
                Expr {
                    expr_type: Box::new(ExprType::Type { type_ }),
                    extract: first_position..self.current_str,
                    body: self.clone().body,
                }
            },
            TokenType::IDENT => {
                self.current_str += self.peek().lexeme.len();
                Expr {
                    expr_type: Box::new(ExprType::Type { type_ }),
                    extract: first_position..self.current_str,
                    body: self.clone().body,
                }
            },

            _ => {
                Error::new(
                    ErrorType::SyntaxError,
                    "invalid expression",
                    first_position..self.current_str,
                    self.clone().body,
                )
                    .panic();
                unreachable!()
            }
        }
    }
    fn parse_if_statement(&mut self) -> Stmt {
        self.skip_whitespace();
        let first_position = self.current_str;
        self.advance();
        let condition = self.term();
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
            )
        } else {
            Stmt::new(
                StmtType::If {
                    cond: condition,
                    then: then_branch,
                },
                first_position..self.current_str,
                self.clone().body,
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
        };
    }


    fn term(&mut self) -> Expr {
        self.skip_whitespace();
        let mut left = self.factor();

        let mut op;
        let mut right;
        let first_position = self.current_str;
        while self.check(TokenType::ADD) || self.check(TokenType::SUB) {
            op = self.peek();
            self.advance();
            right = self.factor();

            left = Expr {
                expr_type: Box::new(ExprType::BinOp { op, left, right }),
                extract: first_position..self.current_str,
                body: self.clone().body,
            };
        }

        left
    }

    fn factor(&mut self) -> Expr {
        self.skip_whitespace();
        let mut left = self.cmp_operator();
        let mut op;
        let mut right;
        let first_position = self.current_str;
        while self.check(TokenType::MUL) || self.check(TokenType::DIV) {
            op = self.peek();
            self.advance();
            right = self.unary();
            left = Expr {
                expr_type: Box::new(ExprType::BinOp { op, left, right }),
                extract: first_position..self.current_str,
                body: self.clone().body,
            };
        }

        left
    }

    fn cmp_operator(&mut self) -> Expr {
        self.skip_whitespace();
        let mut left = self.unary();
        let mut op;
        let mut right;
        let first_position = self.current_str;
        while self.check(TokenType::LT)
            || self.check(TokenType::GT)
            || self.check(TokenType::LTE)
            || self.check(TokenType::GTE)
            || self.check(TokenType::EQUAL)
        {
            op = self.peek();
            self.advance();
            right = self.unary();
            left = Expr {
                expr_type: Box::new(ExprType::CmpOp { op, left, right }),
                extract: first_position..self.current_str,
                body: self.clone().body,
            };
        }

        left
    }

    fn unary(&mut self) -> Expr {
        self.skip_whitespace();
        let first_position = self.current_str;
        if self.match_token(TokenType::NOT) || self.match_token(TokenType::SUB) {
            let op = self.previous();
            let operand = self.unary();
            return Expr {
                expr_type: Box::new(ExprType::UnaryOp { op, operand }),
                extract: first_position..self.current_str,
                body: self.clone().body,
            };
        }
        self.assign()
    }

    fn assign(&mut self) -> Expr {
        self.skip_whitespace();
        let name = self.call();
        let first_position = self.current_str;
        if self.match_token(TokenType::ASSIGN) {
            Expr {
                expr_type: Box::new(ExprType::Assign {
                    name: match *name.expr_type {
                        ExprType::Ident { ident } => ident,
                        _ => {
                            error!(
                                ErrorType::SyntaxError,
                                "ident expected",
                                first_position..self.current_str,
                                self.body.clone()
                            );
                            unreachable!()
                        }
                    },
                    value: self.term(),
                }),
                extract: first_position..self.current_str,
                body: self.clone().body,
            }
        } else {
            name
        }
    }

    fn call(&mut self) -> Expr {
        self.skip_whitespace();
        let first_position = self.current_str;
        let callee = self.ns_get();
        if self.if_is_at_end() {
            return callee;
        }
        loop {
            if self.match_token(TokenType::LPAREN) {
                let mut args = Vec::new();
                if !self.check(TokenType::RPAREN) {
                    args.push(self.term());
                    while self.match_token(TokenType::COMMA) {
                        args.push(self.primary())
                    }
                }
                self.expect_token(TokenType::RPAREN);
                return Expr {
                    expr_type: Box::new(ExprType::Call { name: callee, args }),
                    extract: first_position..self.current_str,
                    body: self.clone().body,
                };
            } else {
                break callee;
            };
        }
    }
    // Example: a::b::c
    // ns means namespace
    fn ns_get(&mut self) -> Expr {
        self.skip_whitespace();
        let first_position = self.current_str;
        let mut name = self.get();
        if self.match_token(TokenType::DOUBLECOLON) {
            let attr = self.term();
            name = Expr {
                expr_type: Box::new(ExprType::NsGet { name, attr }),
                extract: first_position..self.current_str,
                body: self.clone().body,
            };

        }

        name
    }

    // Example: a.b.c
    fn get(&mut self) -> Expr {
        self.skip_whitespace();
        let first_position = self.current_str;
        let mut name = self.primary();
        if self.match_token(TokenType::DOT) {
            let attr = self.term();
            name = Expr {
                expr_type: Box::new(ExprType::Get { name, attr }),
                extract: first_position..self.current_str,
                body: self.clone().body,
            };

        }

        name
    }

    fn primary(&mut self) -> Expr {
        self.skip_whitespace();
        let first_position = self.current_str;
        if self.match_token(TokenType::FALSE) {
            return Expr {
                expr_type: Box::new(ExprType::Literal {
                    literal: LiteralType::Bool(false),
                }),
                extract: first_position..self.current_str,
                body: self.clone().body,
            };
        }
        if self.match_token(TokenType::TRUE) {
            return Expr {
                expr_type: Box::new(ExprType::Literal {
                    literal: LiteralType::Bool(true),
                }),
                extract: first_position..self.current_str,
                body: self.clone().body,
            };
        }

        let res = match self.peek().token_type {
            TokenType::NUMBER => Expr {
                expr_type: Box::new(ExprType::Literal {
                    literal: LiteralType::Number(self.peek().lexeme.parse().unwrap()),
                }),
                extract: first_position..self.current_str + self.peek().lexeme.len(),
                body: self.clone().body,
            },
            TokenType::STRING => {
                self.current_str += self.peek().lexeme.len();
                Expr {
                    expr_type: Box::new(ExprType::Literal {
                        literal: LiteralType::String(self.peek().lexeme.to_string()),
                    }),
                    extract: first_position..self.current_str,
                    body: self.clone().body,
                }
            }
            TokenType::IDENT => Expr {
                expr_type: Box::new(ExprType::Ident { ident: self.peek() }),
                extract: first_position..self.current_str + self.peek().lexeme.len(),
                body: self.clone().body,
            },
            TokenType::LPAREN => self.parse_grouped_expression(),
            TokenType::LBRACKET => self.list(),
            _ => self.expression(),
        };
        self.advance();
        res
    }

    fn identifier(&mut self) -> Expr {
        self.skip_whitespace();
        let first_position = self.current_str;
        if TokenType::IDENT == self.peek().token_type {
            Expr {
                expr_type: Box::new(ExprType::Ident { ident: self.peek() }),
                extract: first_position..self.current_str + self.peek().lexeme.len(),
                body: self.clone().body,
            }
        } else {
            Error::new(
                ErrorType::SyntaxError,
                "expected identifier",
                first_position..self.current_str,
                self.clone().body,
            )
                .panic();
            unreachable!()
        }
    }

    fn list(&mut self) -> Expr {
        self.skip_whitespace();
        let first_position = self.current_str;
        self.expect_token(TokenType::LBRACKET);
        let mut exp_elt = Vec::new();
        let mut index = 0;
        loop {
            if self.peek().token_type == TokenType::RBRACKET {
                self.current_str += self.peek().lexeme.len();
                return Expr {
                    expr_type: Box::new(ExprType::List { elems: exp_elt }),
                    extract: first_position..self.current_str,
                    body: self.clone().body,
                };
            }
            exp_elt.push(self.primary());
            index += 1;
            if self.peek().token_type == TokenType::RBRACKET {
                return Expr {
                    expr_type: Box::new(ExprType::List { elems: exp_elt }),
                    extract: first_position..self.current_str,
                    body: self.clone().body,
                };
            }
            self.expect_token(TokenType::COMMA);

            if index >= 900 {
                error!(
                    ErrorType::SyntaxError,
                    "too many element",
                    first_position..self.current_str,
                    self.clone().body
                )
            }
        }
    }

    fn expression(&mut self) -> Expr {
        self.skip_whitespace();
        let expression = match self.clone().peek().token_type {
            TokenType::INT_TYPE
            | TokenType::BOOL_TYPE
            | TokenType::STRING_TYPE
            | TokenType::LIST_TYPE => self.parse_type_expression(),
            TokenType::CAST => self.parse_cast_expression(),
            TokenType::INIT => self.parse_init_struct_expresion(),
            TokenType::EOF => {
                self.is_at_end = true;
                self.pop().unwrap();
                self.bake_up();
                Expr {
                    expr_type: Box::new(ExprType::Eof),
                    extract: 0..0,
                    body: self.clone().body,
                }
            }
            _ => panic!("Unexpected token: {:?}", self.clone().peek()),
        };

        expression
    }

    fn parse_grouped_expression(&mut self) -> Expr {
        self.skip_whitespace();
        let first_position = self.current_str;
        self.advance();
        let expression = self.term();
        self.expect_token(TokenType::RPAREN);
        Expr {
            expr_type: Box::new(ExprType::Grouping { group: expression }),
            extract: first_position..self.current_str,
            body: self.body.clone(),
        }
    }

    fn parse_cast_expression(&mut self) -> Expr {
        self.skip_whitespace();
        let first_position = self.current_str;
        self.advance();
        let elt = self.term();
        self.expect_token(TokenType::TO);
        let type_ = self.parse_type_expression();
        Expr {
            expr_type: Box::new(ExprType::To { value: elt, type_ }),
            extract: first_position..self.current_str,
            body: self.clone().body,
        }
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
        )
    }

    pub fn match_token(&mut self, token: TokenType) -> bool {
        if self.check(token) {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn expect_token(&mut self, token: TokenType) {
        if !self.match_token(token.clone()) {
            panic!("Expected {:?} got {:?}", token.clone(), self.peek());
        }
    }

    pub fn check(&mut self, token: TokenType) -> bool {
        if self.if_is_at_end() {
            return false;
        }
        self.peek().token_type == token
    }

    pub fn peek(&mut self) -> Token {
        self.tokens[self.current as usize].clone()
    }

    pub fn previous(&self) -> Token {
        self.tokens[self.current as usize - 1].clone()
    }

    pub fn advance(&mut self) -> Token {
        self.current_str += self.peek().lexeme.len();
        self.current += 1;
        self.skip_whitespace();

        self.peek()
    }

    pub fn bake_up(&mut self) -> Token {
        self.current -= 1;
        self.peek()
    }
    pub fn pop(&mut self) -> Option<Token> {
        self.tokens.pop()
    }

    pub fn if_is_at_end(&mut self) -> bool {
        if self.peek().token_type == TokenType::EOF {
            self.is_at_end = true;
        }

        self.is_at_end
    }

    fn skip_whitespace(&mut self) {
        while self.peek().token_type == TokenType::NEWLINE
            || self.peek().token_type == TokenType::SPACE
            || self.peek().token_type == TokenType::TAB
        {
            self.advance();
        }
    }
}