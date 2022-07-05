use crate::ast::Expr;
use crate::ast::Op as AstOp;
use crate::expr::*;
use crate::lexer::{Token, Keyword, Op, TypeToken};
use crate::value::Type;

#[derive(Debug, Clone)]
pub struct Statement(Expr);

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>
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
            program.statements.push(Statement(self.parse_statement()));
            
        }
        program
    }
    pub fn parse_statement(&mut self) -> Expr {
        let statement = match self.clone().peek() {
            Token::Keyword(Keyword::LET) => self.parse_let_statement(),
            Token::Keyword(Keyword::IF) => self.parse_if_statement(),
            Token::Keyword(Keyword::WHILE) => self.parse_while_statement(),
            Token::Keyword(Keyword::CONST) => self.parse_const_statement(),
            _ => self.parse_expression(),
        };
        
        statement
    }

    pub fn parse_let_statement(&mut self) -> Expr {
        self.advance();
        let name = self.parse_identifier();
        
        self.advance();
        let type_;
        if self.match_token(Token::TWODOTS) {
            
            type_ = Some(Box::new(self.parse_expression()));
            
        } else {
            type_ = None;
        }
        let value = if self.match_token(Token::Op(Op::ASSIGN)) {
            self.parse_expression()
        } else {
            panic!("expected '=' after let");
        };
        Expr::Assign(assign::Assign {
            name: match name {
                Expr::Ident(ident::Ident(n)) => n,
                _ => panic!("expected identifier"),
            },
            value: Box::new(value),
            mutable: true,
            type_: type_,
        })
    }

    pub fn parse_while_statement(&mut self) -> Expr {
        self.advance();
        let condition = self.parse_expression();
        self.advance();
        let body = self.parse_block_statement();
        Expr::While(loop_while::While {
            cond: Box::new(condition),
            body: Box::new(body),
        })
    }

    pub fn parse_const_statement(&mut self) -> Expr {
        self.advance();
        let name = self.parse_identifier();
        self.advance();
        let type_;
        if self.match_token(Token::TWODOTS) {
            type_ = Some(Box::new(self.parse_expression()));
            
        } else {
            type_ = None;
        }
        let value = if self.match_token(Token::Op(Op::ASSIGN)) {
            self.parse_expression()
            
        } else {
            panic!("expected '=' after let");
        };
        Expr::Assign(assign::Assign {
            name: match name {
                Expr::Ident(ident::Ident(n)) => n,
                _ => panic!("expected identifier"),
            },
            value: Box::new(value),
            mutable: false,
            type_: type_,
        })
    }

    pub fn parse_type_expression(&mut self) -> Expr {
        let mut type_ = self.peek();
        match type_ {
            Token::Type(TypeToken::INT) => {
                self.advance();
                Expr::TypeExpr(type_::TypeExpr(Type::Int))
            }
            _ => panic!("expected type"),
        }
    }
    pub fn parse_if_statement(&mut self) -> Expr {
        self.advance();
        let condition = self.parse_expression();
        self.advance();
        
        let then_branch = self.parse_statement();
        if self.check(Token::Keyword(Keyword::ELSE)) {
            self.advance();
            Expr::IfThenElse(ifthenelse::IfThenElse {
                cond: Box::new(condition), 
                then: Box::new(then_branch),
                else_: Box::new(self.parse_block_statement())
            }
        )                
        } else {
            Expr::IfThen(ifthen::IfThen {
                cond: Box::new(condition),
                then: Box::new(then_branch),
                
            })
        }
        
    }

    pub fn parse_block_statement (&mut self) -> Expr {
        let mut statements = vec![];
        self.advance();
        
        while !self.check(Token::RBRACE) {
            
            statements.push(self.parse_statement());
            self.advance();
        }
        self.advance();
        Expr::Block(block::Block {
            body: statements,
        })
    }

    pub fn parse_op_expression(&mut self) -> Expr {
        let token = self.advance();
        let left = self.parse_expression();
        let right = self.parse_expression();
        match token {
            Token::Op(Op::ADD) => Expr::BinOp(binop::BinOp {
                left: Box::new(left),
                right: Box::new(right),
                op: AstOp::Add,
            }),
            Token::Op(Op::SUB) => Expr::BinOp(binop::BinOp {
                left: Box::new(left),
                right: Box::new(right),
                op: AstOp::Sub,
            }),
            Token::Op(Op::MUL) => Expr::BinOp(binop::BinOp {
                left: Box::new(left),
                right: Box::new(right),
                op: AstOp::Mul,
            }),
            Token::Op(Op::DIV) => Expr::BinOp(binop::BinOp {
                left: Box::new(left),
                right: Box::new(right),
                op: AstOp::Div,
            }),
            Token::Op(Op::MOD) => Expr::BinOp(binop::BinOp {
                left: Box::new(left),
                right: Box::new(right),
                op: AstOp::Mod,
            }),
            _ => panic!("Expected binary operator"),

        }
    }

    pub fn parse_number (&mut self) -> Expr {
        let n = match self.peek() {
            Token::Number(n) => n,
            _ => panic!("Expected number"),
        };
        self.advance();
        Expr::Literal(literal::Literal(literal::LiteralType::Number(n.into())))
    }

    pub fn parse_identifier (&mut self) -> Expr {
        
        let name = match self.peek() {
            Token::Ident(name) => name,
            _ => panic!("Expected identifier"),
        };
        Expr::Ident(ident::Ident(name))
    }

    pub fn parse_expression(&mut self) -> Expr {
        let expression = match self.clone().peek() {
            Token::Number(_) => self.parse_number(),
            Token::Ident(_) => self.parse_identifier(),
            Token::LPAREN => self.parse_grouped_expression(),
            Token::Op(_) => self.parse_op_expression(),
            Token::Type(TypeToken::INT)     | 
            Token::Type(TypeToken::BOOL)    | 
            Token::Type(TypeToken::STRING)  |
            Token::Type(TypeToken::ARRAY) => self.parse_type_expression(),
            Token::LBRACE => self.parse_block_statement(),
            _ => panic!("Expected expression: {:?}", self.clone().peek()),
        };

        expression
    }

    pub fn parse_grouped_expression(&mut self) -> Expr {
        self.advance();
        let expression = self.parse_expression();
        self.expect_token(Token::RPAREN);
        expression
    }

    pub fn match_token(&mut self, token: Token) -> bool {
        if self.check(token) {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn expect_token(&mut self, token: Token) {
        if !self.match_token(token.clone()) {
            panic!("Expected {:?}", token.clone());
        }
    }

    pub fn check(&mut self, token: Token) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek() == token
    }

    pub fn peek(&self) -> Token {
        
        self.tokens[self.current].clone()
    }

    pub fn advance(&mut self) -> Token {
        self.current += 1;
        self.peek()
    }


    pub fn is_at_end(&self) -> bool {
        self.peek() == Token::EOF
    }
    
    
}