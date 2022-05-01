use crate::lexer::Lexer;
use crate::tree::Expr;
use crate::tree::Op;

pub fn parse_expr(tokens: &mut Vec<Lexer>) -> Result<Expr, String> {
    if tokens.len() == 0 {
        return Err("Unexpected end of file".to_string());
    }
    let token = tokens[0].clone();
    match token {
        Lexer::Number(n) => {
            tokens.remove(0);
            return Ok(Expr::Number { value: n.parse::<f64>().unwrap() });
        },
        Lexer::Identifier(i) => {
            tokens.remove(0);
            Ok(Expr::Identifier { name: i.clone() })
        },
        Lexer::Keyword(k) => {
            match k.as_ref() {
                "if" => {
                    let mut is_else = false;
                    tokens.remove(0); // remove "if " of if block
                    let cond = parse_expr(tokens)?;
                    
                    if tokens[0] != Lexer::Seperator("{".to_string()) {
                        return Err("Expected token '{'".to_string());
                    }
                    if tokens.last().unwrap() != &Lexer::Seperator("}".to_string()) {
                        return Err("Expected token '}'".to_string());
                    }
                    tokens.remove(0); // remove "{" of if block 
                    if tokens.iter().filter(|x| x == &&Lexer::Keyword("else".to_string())).count() >= 1 {
                        let index = tokens.iter().position(|x| x == &Lexer::Keyword("else".to_string())).unwrap();
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
                        println!("tokens: {:?}", tokens);
                        tokens.remove(0);
                        if tokens[0] != Lexer::Seperator("{".to_string()) {
                            return Err("Expected token '{'".to_string());
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
                "while" => {
                    tokens.remove(0); // remove "while" of while block
                    let cond = parse_expr(tokens)?;
                    if tokens[0] != Lexer::Keyword("do".to_string()) {
                        return Err("Expected keyword 'do'".to_string());
                    }
                    tokens.remove(0); // remove "do" of while block
                    let body = parse_expr(tokens)?;
                    Ok(Expr::While {
                        cond: Box::new(cond),
                        body: Box::new(body)
                    })
                },
                "let" => {
                    tokens.remove(0);
                    if let Lexer::Identifier(name) = tokens[0].clone() {
                        
                        tokens.remove(0);
                        if tokens[0] != Lexer::Operator("=".to_string()) {
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
                "for" => {
                    tokens.remove(0);
                    println!("for: {:?}", tokens.len());
                    if tokens.len() < 4 {
                        return Err("Expected identifier: invalid syntash".to_string());
                    }
                    if let Lexer::Identifier(name) = tokens[0].clone()  {
                        tokens.remove(0);
                        if tokens[0] != Lexer::Keyword("in".to_string()) {
                            return Err("Expected keyword 'in'".to_string());
                        }
                        tokens.remove(0);
                        let iter = parse_expr(tokens)?;
                        if tokens[0] != Lexer::Keyword("do".to_string()) {
                            return Err("Expected keyword 'do'".to_string());
                        }
                        tokens.remove(0);
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
        Lexer::Operator(op) => {
            tokens.remove(0); // remove operator of binary expression
            let left = parse_expr(tokens)?;
            let right = parse_expr(tokens)?;
            let op_enum = match op.as_ref() {
                "+" => Op::Add,
                "-" => Op::Sub,
                "*" => Op::Mul,
                "/" => Op::Div,
                "%" => Op::Mod,
                "==" => Op::Eq,
                "!=" => Op::Neq,
                "<" => Op::Lt,
                ">" => Op::Gt,
                "<=" => Op::Le,
                ">=" => Op::Ge,
                "&&" => Op::And,
                "||" => Op::Or,
                _ => Op::Invalid
            };
            Ok(Expr::BinOp {
                op: op_enum.clone(),
                left: Box::new(left),
                right: Box::new(right)
            })
        },
        Lexer::String(s) => {
            tokens.remove(0);
            Ok(Expr::String { value: s.clone() })
        },
        _ => Err("Unexpected token".to_string())
    }
}
