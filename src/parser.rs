// #[path = "tree.rs"] mod tree;
use crate::lexer::Lexer;
use crate::tree::Expr;

pub fn parse_expr(tokens: &mut Vec<Lexer>) -> Result<Expr, String> {
    if tokens.len() == 0 {
        return Err("Unexpected end of file".to_string());
    }
    let token = tokens[0].clone();
    match token {
        Lexer::Number(n) => {
            tokens.remove(0);
            return Ok(Expr::Number { value: n.clone() });
        },
        Lexer::Identifier(i) => {
            tokens.remove(0);
            Ok(Expr::Identifier { name: i.clone() })
        },
        Lexer::Keyword(k) => {
            match k.as_ref() {
                "if" => {
                    tokens.remove(0); // remove "if" of if block
                    let cond = parse_expr(tokens)?;
                    if tokens[0] != Lexer::Keyword("then".to_string()) {
                        return Err("Expected keyword 'then'".to_string());
                    }
                    tokens.remove(0); // remove "then" of if block 
                    let then = parse_expr(tokens)?;
                    if tokens.len() == 1 {
                        Ok(Expr::IfThen {
                            cond: Box::new(cond),
                            then: Box::new(then)
                        })
                    } else if tokens.len() >= 2 {
                            tokens.remove(0);
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
                    let cond = parse_expr(tokens)?;
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
                        Ok(Expr::Let {
                            name: name.clone(),
                            value: Box::new(value)
                        })
                    } else {
                        Err("Expected identifier".to_string())
                    }
                },
                _ => Err("Unexpected keyword: ".to_string())
            }
        },
        Lexer::Operator(op) => {
            let left = parse_expr(tokens)?;
            let right = parse_expr(tokens)?;
            Ok(Expr::BinOp {
                op: op.clone(),
                left: Box::new(left),
                right: Box::new(right)
            })
        },
        _ => Err("Unexpected token".to_string())
    }
}
