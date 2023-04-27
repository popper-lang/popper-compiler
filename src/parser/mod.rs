pub mod parser_expr;
pub mod parser_stmt;
use std::fmt::Debug;
use crate::ast::expr::{Expr, ExprType, LiteralType};
use crate::ast::stmt::{Stmt, StmtType};
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