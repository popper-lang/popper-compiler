use crate::ast::expr::{Expr, ExprType, LiteralType};
use crate::parser::Parser;
use crate::lexer::{TokenType};
use crate::errors::{error, Error, ErrorType};

impl Parser {
    pub fn parse_asm_expression(&mut self) -> Expr {

        Expr::new(
            Box::new(ExprType::Asm {
                asm: self.peek().lexeme,
            }),
            self.current_str..self.current_str,
            self.clone().body,
            self.clone().file
        )
    }

    pub fn parse_init_struct_expression(&mut self) -> Expr {
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
            self.clone().file
        )
    }

    pub fn parse_type_expression(&mut self) -> Expr {
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
                    file: self.clone().file
                }
            },
            TokenType::IDENT => {
                self.current_str += self.peek().lexeme.len();
                Expr {
                    expr_type: Box::new(ExprType::Type { type_ }),
                    extract: first_position..self.current_str,
                    body: self.clone().body,
                    file: self.clone().file
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

    pub fn term(&mut self) -> Expr {
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
                file: self.clone().file
            };
        }

        left
    }

    pub fn factor(&mut self) -> Expr {
        self.skip_whitespace();
        let mut left = self.cmp_operator();
        let mut op;
        let mut right;
        let first_position = self.current_str;
        while self.check(TokenType::MUL) || self.check(TokenType::DIV) {
            op = self.peek();
            self.advance();
            right = self.cmp_operator();
            left = Expr {
                expr_type: Box::new(ExprType::BinOp { op, left, right }),
                extract: first_position..self.current_str,
                body: self.clone().body,
                file: self.clone().file
            };
        }

        left
    }

    pub fn modulo(&mut self) -> Expr {
        self.skip_whitespace();
        let mut left = self.unary();
        let mut op;
        let mut right;
        let first_position = self.current_str;
        while self.check(TokenType::MOD) {
            op = self.peek();
            self.advance();
            right = self.unary();
            left = Expr {
                expr_type: Box::new(ExprType::BinOp { op, left, right }),
                extract: first_position..self.current_str,
                body: self.clone().body,
                file: self.clone().file
            };
        }

        left
    }

    pub fn cmp_operator(&mut self) -> Expr {
        self.skip_whitespace();
        let mut left = self.modulo();
        let mut op;
        let mut right;
        let first_position = self.current_str;
        while self.check(TokenType::LT)
            || self.check(TokenType::GT)
            || self.check(TokenType::LTE)
            || self.check(TokenType::GTE)
            || self.check(TokenType::EQ)
        {
            op = self.peek();
            self.advance();
            right = self.unary();
            left = Expr {
                expr_type: Box::new(ExprType::CmpOp { op, left, right }),
                extract: first_position..self.current_str,
                body: self.clone().body,
                file: self.clone().file
            };
        }

        left
    }

    pub fn unary(&mut self) -> Expr {
        self.skip_whitespace();
        let first_position = self.current_str;
        if self.check(TokenType::NOT) || self.check(TokenType::SUB) {
            let op = self.peek();
            self.advance();
            let operand = self.unary();
            return Expr {
                expr_type: Box::new(ExprType::UnaryOp { op, operand }),
                extract: first_position..self.current_str,
                body: self.clone().body,
                file: self.clone().file
            };
        }
        self.assign()
    }

    pub fn assign(&mut self) -> Expr {
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
                file: self.clone().file
            }
        } else {
            name
        }
    }

    pub fn call(&mut self) -> Expr {
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
                        args.push(self.ns_get())
                    }
                }
                self.expect_token(TokenType::RPAREN);
                return Expr {
                    expr_type: Box::new(ExprType::Call { name: callee, args }),
                    extract: first_position..self.current_str,
                    body: self.clone().body,
                    file: self.clone().file
                };
            } else {
                break callee;
            };
        }
    }

    // Example: a::b::c
    // ns means namespace
    pub fn ns_get(&mut self) -> Expr {
        self.skip_whitespace();
        let first_position = self.current_str;
        let mut name = self.get();
        if self.match_token(TokenType::DOUBLECOLON) {
            let attr = self.term();
            name = Expr {
                expr_type: Box::new(ExprType::NsGet { name, attr }),
                extract: first_position..self.current_str,
                body: self.clone().body,
                file: self.clone().file
            };

        }

        name
    }

    // Example: a.b.c
    pub fn get(&mut self) -> Expr {
        self.skip_whitespace();
        let first_position = self.current_str;
        let mut name = self.range();
        if self.match_token(TokenType::DOT) {
            let attr = self.term();
            name = Expr {
                expr_type: Box::new(ExprType::Get { name, attr }),
                extract: first_position..self.current_str,
                body: self.clone().body,
                file: self.clone().file
            };

        }

        name
    }

    pub fn range(&mut self) -> Expr {
        self.skip_whitespace();
        let first_position = self.current_str;
        let start = self.primary();
        if self.match_token(TokenType::COLON) {
            let end = self.primary();
            return Expr {
                expr_type: Box::new(ExprType::Range { start, end }),
                extract: first_position..self.current_str,
                body: self.clone().body,
                file: self.clone().file
            };
        }

        start
    }

    pub fn primary(&mut self) -> Expr {
        self.skip_whitespace();
        let first_position = self.current_str;
        if self.match_token(TokenType::FALSE) {
            return Expr {
                expr_type: Box::new(ExprType::Literal {
                    literal: LiteralType::Bool(false),
                }),
                extract: first_position..self.current_str,
                body: self.clone().body,
                file: self.clone().file
            };
        }
        if self.match_token(TokenType::TRUE) {
            return Expr {
                expr_type: Box::new(ExprType::Literal {
                    literal: LiteralType::Bool(true),
                }),
                extract: first_position..self.current_str,
                body: self.clone().body,
                file: self.clone().file
            };
        }


        let res = match self.peek().token_type {
            TokenType::NUMBER => Expr {
                expr_type: Box::new(ExprType::Literal {
                    literal: LiteralType::Number(self.peek().lexeme.parse().unwrap()),
                }),
                extract: first_position..self.current_str + self.peek().lexeme.len(),
                body: self.clone().body,
                file: self.clone().file
            },
            TokenType::STRING => {
                self.current_str += self.peek().lexeme.len();
                Expr {
                    expr_type: Box::new(ExprType::Literal {
                        literal: LiteralType::String(self.peek().lexeme.to_string()),
                    }),
                    extract: first_position..self.current_str,
                    body: self.clone().body,
                    file: self.file.clone()
                }
            }
            TokenType::IDENT => Expr {
                expr_type: Box::new(ExprType::Ident { ident: self.peek() }),
                extract: first_position..self.current_str + self.peek().lexeme.len(),
                body: self.clone().body,
                file: self.clone().file
            },
            TokenType::ASM => self.parse_asm_expression(),
            TokenType::LPAREN => self.parse_grouped_expression(),
            TokenType::LBRACKET => self.list(),
            _ => self.expression(),
        };
        self.advance();
        res
    }

    pub fn identifier(&mut self) -> Expr {
        self.skip_whitespace();
        let first_position = self.current_str;
        if TokenType::IDENT == self.peek().token_type {
            Expr {
                expr_type: Box::new(ExprType::Ident { ident: self.peek() }),
                extract: first_position..self.current_str + self.peek().lexeme.len(),
                body: self.clone().body,
                file: self.clone().file

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

    pub fn list(&mut self) -> Expr {
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
                    file: self.clone().file
                };
            }
            exp_elt.push(self.primary());
            index += 1;
            if self.peek().token_type == TokenType::RBRACKET {
                return Expr {
                    expr_type: Box::new(ExprType::List { elems: exp_elt }),
                    extract: first_position..self.current_str,
                    body: self.clone().body,
                    file: self.clone().file
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

    pub fn expression(&mut self) -> Expr {
        self.skip_whitespace();
        let expression = match self.clone().peek().token_type {
            TokenType::INT_TYPE
            | TokenType::BOOL_TYPE
            | TokenType::STRING_TYPE
            | TokenType::LIST_TYPE => self.parse_type_expression(),
            TokenType::CAST => self.parse_cast_expression(),
            TokenType::INIT => self.parse_init_struct_expression(),
            TokenType::EOF => {
                self.is_at_end = true;
                self.pop().unwrap();
                self.bake_up();
                Expr {
                    expr_type: Box::new(ExprType::Eof),
                    extract: 0..0,
                    body: self.clone().body,
                    file: self.clone().file
                }
            }
            _ => panic!("Unexpected token: {:?}", self.clone().peek()),
        };

        expression
    }

    pub fn parse_grouped_expression(&mut self) -> Expr {
        self.skip_whitespace();
        let first_position = self.current_str;
        self.advance();
        let expression = self.term();
        self.expect_token(TokenType::RPAREN);
        Expr {
            expr_type: Box::new(ExprType::Grouping { group: expression }),
            extract: first_position..self.current_str,
            body: self.body.clone(),
            file: self.clone().file
        }
    }

    pub fn parse_cast_expression(&mut self) -> Expr {
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
            file: self.clone().file
        }
    }

}