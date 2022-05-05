use crate::lexer::Token;
use crate::lexer::Keyword;
use crate::lexer::Seperator;
use crate::lexer::Operator;
use crate::lexer::Identifier;
use crate::tree::Expr;
use crate::tree::Op;

pub fn parse_token(tokens: &mut Vec<Token>) -> Result<Expr, String> {
    if tokens.len() == 0 {
        return Err("Unexpected end of file".to_string());
    }
    let token = tokens[0].clone();
    match token {
        Token::Literal(n) => {
            tokens.remove(0);
            return Ok(Expr::Literal { value: n });
        },
        Token::Identifier(i) => {
            tokens.remove(0);
            Ok(Expr::Identifier { name: i.0.clone() })
        },
        Token::Keyword(k) => {
            match k {
                Keyword::If => {
                    let mut is_else = false;
                    tokens.remove(0); // remove "if " of if block
                    let cond = parse_expr(tokens)?;
                    
                    if tokens[0] != Token::Seperator(Seperator::LeftBrace) {
                        return Err("Expected token '{'".to_string());
                    }
                    if tokens.last().unwrap() != &Token::Seperator(Seperator::RightBrace) {
                        return Err("Expected token '}'".to_string());
                    }
                    tokens.remove(0); // remove "{" of if block 
                    if tokens.iter().filter(|x| x == &&Token::Keyword(Keyword::Else)).count() >= 1 {
                        let index = tokens.iter().position(|x| x == &Token::Keyword(Keyword::Else)).unwrap();
                        tokens.remove(index-1); // remove "}" of if block
                        is_else = true
                    } else {
                        tokens.pop(); // remove "}" of if block
                    }
                    let then = parse_expr(tokens)?;
                    
                    if tokens.len() == 0 {
                        Ok(Expr::IfThen {
                            cond: Box::new(cond),
                            then: Box::new(then)
                        })
                    } else if is_else {
                        tokens.remove(0);
                        if tokens[0] != Token::Seperator(Seperator::LeftBrace) {
                            return Err("Expected token '{'".to_string());
                        }
                        if tokens.last() != Some(&Token::Seperator(Seperator::RightBrace)) {
                            return Err("Expected token '}'".to_string());
                        }
                        tokens.remove(0);
                        tokens.pop();
                        let else_ = parse_expr(tokens)?;
                        Ok(Expr::IfThenElse {
                            cond: Box::new(cond),
                            then: Box::new(then),
                            else_: Box::new(else_)
                        })
                        
                    } else {
                        Err("error syntatic".to_string()) // doesn't match any case         
                    
                    }
                    
                },
                Keyword::While => {
                    tokens.remove(0); // remove "while" of while block
                    let cond = parse_expr(tokens)?;
                    if tokens[0] != Token::Seperator(Seperator::LeftBrace) {
                        return Err("Expected token '{'".to_string());
                    }
                    if tokens[tokens.len()-1] != Token::Seperator(Seperator::RightBrace) {
                        return Err("Expected token '}'".to_string());
                    }
                    tokens.remove(0); // remove "{" of while block
                    tokens.pop(); // remove "}" of while block
                    let body = parse_expr(tokens)?;
                    Ok(Expr::While {
                        cond: Box::new(cond),
                        body: Box::new(body)
                    })
                },
                Keyword::Let => {
                    tokens.remove(0);
                    if let Token::Identifier(Identifier(name)) = tokens[0].clone() {
                        
                        tokens.remove(0);
                        if tokens[0] != Token::Operator(Operator::Assign) {
                            return Err("Expected operator '='".to_string());
                        } else {
                            tokens.remove(0);
                        }
                        let value = parse_expr(tokens)?;
                        Ok(Expr::Assign {
                            name: name.clone(),
                            value: Box::new(value)
                        })
                    } else {
                        Err("Expected identifier".to_string())
                    }
                },
                Keyword::For => {
                    tokens.remove(0);
                    if tokens.len() < 4 {
                        return Err("Expected identifier: invalid syntash".to_string());
                    }
                    if let Token::Identifier(Identifier(name)) = tokens[0].clone()  {
                        tokens.remove(0);
                        if tokens[0] != Token::Keyword(Keyword::In) {
                            return Err("Expected keyword 'in'".to_string());
                        }
                        tokens.remove(0);
                        let iter = parse_expr(tokens)?;
                        if tokens[0] != Token::Seperator(Seperator::LeftBrace) {
                            return Err("Expected keyword '{'".to_string());
                        }
                        if tokens[tokens.len()-1] != Token::Seperator(Seperator::RightBrace) {
                            return Err("Expected keyword '}'".to_string());
                        }
                        tokens.remove(0);
                        tokens.pop();
                        let body = parse_expr(tokens)?;
                        Ok(Expr::For {
                            name: name.clone(),
                            iter: Box::new(iter),
                            body: Box::new(body)
                        })
                    } else {
                        Err("Expected identifier: no var".to_string())
                    }
                },
                
                _ => Err("Unexpected keyword: ".to_string())
            }
        },
        Token::Operator(op) => {
            tokens.remove(0); // remove operator of binary expression
            let left = parse_expr(tokens)?;
            let right = parse_expr(tokens)?;
            let op_enum = match op {
                Operator::Add => Op::Add,
                Operator::Sub => Op::Sub,
                Operator::Mul => Op::Mul,
                Operator::Div => Op::Div,
                Operator::Mod => Op::Mod,
                Operator::Eq => Op::Eq,
                Operator::Lt => Op::Lt,
                Operator::Gt => Op::Gt,
                Operator::Le => Op::Le,
                Operator::Ge => Op::Ge,
                Operator::Neq => Op::Neq,
                Operator::And => Op::And,
                Operator::Or => Op::Or,
                Operator::Assign => Op::Assign,

                
            };
            Ok(Expr::BinOp {
                op: op_enum.clone(),
                left: Box::new(left),
                right: Box::new(right)
            })
        }
        _ => Err("Unexpected token".to_string())
    }
}