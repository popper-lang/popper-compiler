use crate::ast::expr::Expr;
use crate::ast::stmt::Stmt;
use crate::lexer::{Token, TokenType};
use crate::errors::error;
use crate::value::Value;



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
    fn parse_let_statement(&mut self) -> Stmt {
        self.advance();
        let name = self.parse_identifier();
        
        self.advance();
        let type_;
        if self.match_token(TokenType::TWODOTS) {
            
            type_ = Some(self.parse_type_expression());
            
        } else {
            type_ = None;
        }
        let value = if self.match_token(TokenType::ASSIGN) {
            self.parse_expression()
        } else {
            panic!("expected '=' after let");
        };
        Stmt::Assign {
            name: match name {
                Expr::Ident { ident } => ident,
                _ => panic!("expected identifier"),
            },
            value: Box::new(value),
            mutable: true,
            type_: type_,
        }
    }



    fn parse_while_statement(&mut self) -> Stmt {
        self.advance();
        let condition = self.parse_expression();
        self.advance();
        let body = self.parse_block_statement();
        Stmt::While { 
            cond: condition,
            body: Box::new(body),
        }
    }

    fn parse_const_statement(&mut self) -> Stmt {
        self.advance();
        let name = self.parse_identifier();
        self.advance();
        let type_;
        if self.match_token(TokenType::TWODOTS) {
            type_ = Some(self.parse_expression());
            
        } else {
            type_ = None;
        }
        let value = if self.match_token(TokenType::ASSIGN) {
            self.term()
            
        } else {
            panic!("expected '=' after let");
        };
        Stmt::Assign {
            name: match name {
                Expr::Ident { ident } => ident,
                _ => panic!("expected identifier"),
            },
            value: Box::new(value),
            mutable: false,
            type_: type_,
        }
    }

    fn parse_type_expression(&mut self) -> Expr {
        println!("parse_type_expression peek: {:?}", self.peek());
        let type_ = self.peek();
        match type_.token_type {
            TokenType::INT | TokenType::BOOLEAN | TokenType::STRING => {
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
        let mut statements = vec![];
        
        while !self.match_token(TokenType::RBRACE) {
            statements.push(self.parse_statement());
            
            
        }
        Stmt::Block {
            body: statements,
        }
    }

    fn term(&mut self) -> Expr {
        let mut left = self.factor();
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
        if self.match_token(TokenType::NOT) || self.match_token(TokenType::SUB) {
            let op = self.previous();
            let operand = self.unary();
            return Expr::UnaryOp {
                op,
                operand: Box::new(operand),
            };
        }
        self.primary()
    }

    fn primary(&mut self) -> Expr {
        
        if self.match_token(TokenType::FALSE) { 
            return Expr::Literal {
                literal: Value::Bool(false),
            };
            
        }
        if self.match_token(TokenType::TRUE) { 
            return Expr::Literal {
                literal: Value::Bool(true),
            }
        }

        let res = match self.peek().token_type {
            TokenType::Number(n) => Expr::Literal {
                literal: Value::Number(n.into()),
            },
            TokenType::String(s) => Expr::Literal {
                literal: Value::String(s),
            },
            TokenType::Ident(_) => Expr::Ident { ident: self.peek() },
            TokenType::LPAREN => {
                self.parse_grouped_expression()
            },
            _ => {
                self.parse_expression()
            }
        };
        self.advance();
        res

    }
    
    fn parse_identifier(&mut self) -> Expr {
        let mut identifier = String::new();
        if let TokenType::Ident(s) = self.peek().token_type {
            identifier.push_str(&s);
        } else {
            error!("expected identifier", self.peek().line, self.peek().pos);
        }
        Expr::Ident { ident: self.peek() }
    }


    fn parse_expression(&mut self) -> Expr {
        let expression = match self.clone().peek().token_type {
            TokenType::INT        | 
            TokenType::BOOLEAN    | 
            TokenType::STRING     |
            TokenType::ARRAY => self.parse_type_expression(),
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