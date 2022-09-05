use crate::ast::expr::Expr;
use crate::ast::stmt::Stmt;
use crate::lexer::{Token, TokenType};
use crate::errors::error;
use crate::ast::expr::LiteralType;


#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Stmt>
}


#[derive(Debug, Clone)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            current: 0,
        }
    }
    pub fn parse(&mut self) -> Program {
        let mut program = Program {
            statements: vec![],
        };
        while !self.is_at_end()  {
            program.statements.push(self.parse_statement());
            
        }
        program
    }
    fn parse_statement(&mut self) -> Stmt {
        let statement = match self.clone().peek().token_type {
            TokenType::LET => self.parse_let_statement(),
            TokenType::IF => self.parse_if_statement(),
            TokenType::WHILE => self.parse_while_statement(),
            TokenType::CONST => self.parse_const_statement(),
            TokenType::LBRACE => self.parse_block_statement(),
            TokenType::FUN => self.function(),
            TokenType::CLASS => self.parse_class_statement(),
            _ => self.parse_expression_statement(),
        };
        
        statement
    }


    fn parse_expression_statement(&mut self) -> Stmt {
        let expr = self.term();
        Stmt::Expression {
            expr: expr,
        }
    }

    fn parse_class_statement(&mut self) -> Stmt {
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
        Stmt::Class {
            name: match name {
                Expr::Ident { ident } => ident.lexeme,
                _ => unreachable!()
            },
            methods
            
        }
    }
    fn parse_let_statement(&mut self) -> Stmt {
        self.advance();
        let name = self.identifier();
        
        self.advance();
        let type_;
        if self.match_token(TokenType::TWODOTS) {
            
            type_ = Some(self.parse_type_expression());
            
        } else {
            type_ = None;
        }
        let value = if self.match_token(TokenType::ASSIGN) {
            Some(self.term())
        } else {
            None
        };
        Stmt::Let {
            name: match name {
                Expr::Ident { ident } => ident,
                _ => panic!("expected identifier"),
            },
            value,
            mutable: true,
            type_: type_,
        }
    }



    fn parse_while_statement(&mut self) -> Stmt {
        self.advance();
        let condition = self.expression();
        self.advance();
        let body = self.parse_block_statement();
        Stmt::While { 
            cond: condition,
            body: Box::new(body),
        }
    }

    fn parse_const_statement(&mut self) -> Stmt {
        self.advance();
        let name = self.identifier();
        self.advance();
        let type_;
        if self.match_token(TokenType::TWODOTS) {
            type_ = Some(self.expression());
            
        } else {
            type_ = None;
        }
        let value = if self.match_token(TokenType::ASSIGN) {
            Some(self.term())
            
        } else {
            None
        };
        Stmt::Let {
            name: match name {
                Expr::Ident { ident } => ident,
                _ => panic!("expected identifier"),
            },
            value,
            mutable: false,
            type_: type_,
        }
    }

    fn parse_type_expression(&mut self) -> Expr {
        let type_ = self.peek();
        match type_.token_type {
            TokenType::INT_TYPE | TokenType::BOOLEAN_TYPE | TokenType::STRING_TYPE | TokenType::ARRAY_TYPE => {
                Expr::Type { type_ }
            }
            
            _ => {
                error!("invalid expression", type_.line, type_.pos);
            },
        }
    }
    fn parse_if_statement(&mut self) -> Stmt {
        self.advance();
        let condition = self.term();
        self.advance();
        
        let then_branch = self.parse_block_statement();
        
        if self.match_token(TokenType::ELSE) {
            Stmt::IfElse {
                cond: condition, 
                then: Box::new(then_branch),
                else_: Box::new(self.parse_block_statement())
            }
                    
        } else {
            Stmt::If {
                cond: condition,
                then: Box::new(then_branch),
                
            }
        }
        
    }

    fn parse_block_statement(&mut self) -> Stmt {
        self.advance();
        let mut statements = vec![];
        
        while !self.match_token(TokenType::RBRACE) {
            statements.push(self.parse_statement());
            
            
        }
        Stmt::Block {
            body: statements,
        }
    }

    fn term(&mut self) -> Expr {
        println!("term peek 1: {:?}", self.peek());
        let mut left = self.factor();
        println!("term peek 2: {:?}", self.peek());
        let mut op;
        let mut right;
        while self.check(TokenType::ADD) || self.check(TokenType::SUB) {
            println!("peek term: {:?}", self.peek());
            op = self.peek();
            self.advance();
            right = self.factor();
            println!("right: {:?}", right);
            left = Expr::BinOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
            
        }

        left
    }

    fn factor(&mut self) -> Expr {
        println!("factor peek 1: {:?}", self.peek());
        let mut left = self.unary();
        let mut op;
        let mut right;
        while self.check(TokenType::MUL) || self.check(TokenType::DIV) {
            op = self.peek();
            self.advance();
            right = self.unary();
            left = Expr::BinOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
            
        }

        left 
    }
    
    fn unary(&mut self) -> Expr {
        println!("unary peek 1: {:?}", self.peek());
        if self.match_token(TokenType::NOT) || self.match_token(TokenType::SUB) {
            let op = self.previous();
            let operand = self.unary();
            return Expr::UnaryOp {
                op,
                operand: Box::new(operand),
            };
        }
        self.call()
    }

    fn get(&mut self) -> Expr {
        let mut name = self.primary();
        println!("name {:#?}", name);
        if self.match_token(TokenType::DOT) {
            println!("get peek 1: {:?}", self.peek());
            let attr = match self.identifier() {
                Expr::Ident { ident } => ident.lexeme,
                _ => unreachable!()
            };
            name = Expr::Get {
                name: Box::new(name),
                attr,
            }
        }
        
        name
            
    }

    fn call(&mut self) -> Expr {
        println!("call peek 1: {:?}", self.peek());
        let callee = self.get();
        println!("call 1-2: {:?}", self.peek());
        if self.is_at_end() {
            return callee;
        }
        println!("call peek 2: {:?}", self.peek());
        loop {
            if self.match_token(TokenType::LPAREN) {
                println!("[call] match token LPAREN");
                let mut args = Vec::new();
                if !self.check(TokenType::RPAREN) {
                    println!("call: {:?}", self.peek());
                    args.push(self.term());
                    while self.match_token(TokenType::COMMA) {
                        args.push(self.primary())
                    }
                }
                self.expect_token(TokenType::RPAREN);
                return Expr::Call { name: Box::new(callee), args }
            } else {
                
                println!("call peek 3: {:?}", self.peek());
                break callee;
            };
        }

    }


    fn primary(&mut self) -> Expr {
        println!("primary peek 1: {:?}", self.peek());
        if self.match_token(TokenType::FALSE) { 
            return Expr::Literal {
                literal: LiteralType::Bool(false),
            };
            
        }
        if self.match_token(TokenType::TRUE) { 
            return Expr::Literal {
                literal: LiteralType::Bool(true),
            }
        }

        let res = match self.peek().token_type {
            TokenType::NUMBER => Expr::Literal {
                literal: LiteralType::Number(self.peek().lexeme.parse().unwrap()),
            },
            TokenType::STRING => Expr::Literal {
                literal: LiteralType::String(self.peek().lexeme.to_string()),
            },
            TokenType::IDENT => Expr::Ident { ident: self.peek() },
            TokenType::LPAREN => {
                self.parse_grouped_expression()
            },
            _ => {
                self.expression()
            }
        };
        self.advance();
        println!("primary peek 2: {:?}", self.peek());
        res

    }
    
    fn identifier(&mut self) -> Expr {
        if let TokenType::IDENT = self.peek().token_type {
            Expr::Ident { ident: self.peek() }
        } else {
            error!("expected identifier", self.peek().line, self.peek().pos)
        }
    }


    fn expression(&mut self) -> Expr {
        let expression = match self.clone().peek().token_type {
            TokenType::INT_TYPE        | 
            TokenType::BOOLEAN_TYPE    | 
            TokenType::STRING_TYPE     |
            TokenType::ARRAY_TYPE => self.parse_type_expression(),
            TokenType::CAST => self.parse_cast_expression(),
            _ => panic!("Unexpected token: {:?}", self.clone().peek()),
        };

        expression
    }

    fn parse_grouped_expression(&mut self) -> Expr {
        self.advance();
        let expression = self.term();
        self.expect_token(TokenType::RPAREN);
        Expr::Grouping {
            group: Box::new(expression)
        }
    }

    fn parse_cast_expression(&mut self) -> Expr {
        self.advance();
        println!("parse_cast_expression peek: {:?}", self.peek());
        let elt = self.term();
        self.expect_token(TokenType::TO);
        let type_ = self.parse_type_expression();
        Expr::To {
            value: Box::new(elt),
            type_: Box::new(type_),
        }
    }

    fn function(&mut self) -> Stmt {
        self.advance();
        println!("function peek 1 (loop): {:?}", self.peek());
        let name = match self.identifier() {
            Expr::Ident { ident } => ident,
            _ => panic!("Expected identifier"),
        };
        self.advance();
        self.expect_token(TokenType::LPAREN);
        let mut args = Vec::new();
        if !self.check(TokenType::RPAREN) {
            args.push(match self.identifier() {
                Expr::Ident { ident: Token { lexeme: e, ..} } => e.to_string(),
                _ => error!("unexpected expression")
            });
            self.advance();
            while self.match_token(TokenType::COMMA) {
                args.push(match self.identifier() {
                    Expr::Ident { ident: Token { lexeme: e, ..} } => e.to_string(),
                    _ => error!("unexpected expression")
                });
                self.advance();
                
                
            }
        }
        println!("function peek 2 (loop): {:?}", self.peek());
        self.expect_token(TokenType::RPAREN);
        println!("function peek 3 (loop): {:?}", self.peek());
        let body = self.parse_statement();
        Stmt::Function { name, args, body: Box::new(body) }
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
            panic!("Expected {:?}", token.clone());
        }
    }

    pub fn check(&mut self, token: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token
    }

    pub fn peek(&self) -> Token {
        
        self.tokens[self.current].clone()
    }

    pub fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    pub fn advance(&mut self) -> Token {
        self.current += 1;
        self.peek()
    }

    pub fn bake_up(&mut self) -> Token {
        self.current -= 1;
        self.peek()
    }

    


    pub fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }
    
    
}