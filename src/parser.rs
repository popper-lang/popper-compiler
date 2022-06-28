use pest::iterators::Pair;

use crate::ast::Expr;
use crate::expr::*;
use crate::value::Type;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ExprParser;


pub fn build_ast(rules: Pair<Rule>) -> Result<Expr, String> {
    let rules = rules.clone();
    match rules.as_rule() {
        Rule::statement => {
            match rules.into_inner().next() {
                Some(pair) => build_ast(pair),
                None => Err("empty statement".to_string()),
            }
        }
        Rule::if_statement => {
            let condition = build_ast(rules.clone().into_inner().next().unwrap())?;
            
            let then_expr = build_ast(rules.into_inner().nth(1).unwrap())?;

            Ok(Expr::IfThen(ifthen::IfThen {
                cond: Box::new(condition),
                then: Box::new(then_expr),
            }))
        }
        Rule::while_statement => {
            let rules_one = rules.clone().into_inner().next().unwrap();
            let rule_two = rules.clone().into_inner().nth(1).unwrap();
            let condition = build_ast(rules_one)?;
            let body = build_ast(rule_two)?;

            Ok(Expr::While(loop_while::While {
                cond: Box::new(condition),
                body: Box::new(body),
            }))
        }
        Rule::letter => todo!(),
        Rule::digit => todo!(),
        Rule::ident => {
            let ident = rules.as_str();
            Ok(Expr::Ident(ident::Ident(ident.to_string())))
        },
        Rule::number => Ok(Expr::Literal(literal::Literal(literal::LiteralType::Number(
            rules.as_str().parse::<f64>().unwrap(),
        )))),
        Rule::reserved_word => todo!(),
        Rule::keyword => todo!(),
        Rule::bool => {
            let bool = rules.as_str();
            Ok(Expr::Literal(literal::Literal(literal::LiteralType::Bool(
                bool.to_string() == "true",
            ))))
        },
        Rule::type_builtin => {
            match rules.as_str() {
                "int" => Ok(Expr::TypeExpr(type_::TypeExpr(Type::Int))),
                "bool" => Ok(Expr::TypeExpr(type_::TypeExpr(Type::Bool))),
                "string" => Ok(Expr::TypeExpr(type_::TypeExpr(Type::String))),
                _ => Err("invalid type".to_string()),
            }
        },
        Rule::block => {
            let mut exprs = vec![];
            for pair in rules.into_inner() {
                exprs.push(build_ast(pair)?);
            }
            Ok(Expr::Block(block::Block {
                body: exprs,
            }))
        },
        Rule::declaration => {
            let name = rules.clone().into_inner().next().unwrap().as_str();
            let value = build_ast(rules.clone().into_inner().nth(1).unwrap())?;
            Ok(Expr::Assign(assign::Assign {
                name: name.to_string(),
                value: Box::new(value),
                mutable: false,
                type_: None,
            }))
        },
        Rule::assignment => todo!(),
        Rule::if_else_statement => {
            let condition = build_ast(rules.clone().into_inner().next().unwrap())?;
            let then_expr = build_ast(rules.clone().into_inner().nth(1).unwrap())?;
            let else_expr = build_ast(rules.clone().into_inner().nth(2).unwrap())?;
            Ok(Expr::IfThenElse(ifthenelse::IfThenElse {
                cond: Box::new(condition),
                then: Box::new(then_expr),
                else_: Box::new(else_expr),
            }))
        },
        Rule::for_statement => {
            let name = rules.clone().into_inner().next().unwrap().as_str();
            let iter = build_ast(rules.clone().into_inner().nth(1).unwrap())?;
            let body = build_ast(rules.clone().into_inner().nth(2).unwrap())?;
            Ok(Expr::For(loop_for::For {
                name: name.to_string(),
                iter: Box::new(iter),
                body: Box::new(body),
            }))
        },
        Rule::declaration_with_type => {
            let name = rules.clone().into_inner().next().unwrap().as_str();
            let type_ = build_ast(rules.clone().into_inner().nth(1).unwrap())?;
            let value = build_ast(rules.clone().into_inner().nth(2).unwrap())?;
            Ok(Expr::Assign(assign::Assign {
                name: name.to_string(),
                value: Box::new(value),
                mutable: false,
                type_: Some(Box::new(type_)),
            }))
        },
        Rule::to_expression => {
            let value = build_ast(rules.clone().into_inner().next().unwrap())?;
            let type_ = build_ast(rules.clone().into_inner().nth(1).unwrap())?;
            Ok(Expr::To(to::To {
                value: Box::new(value),
                type_: Box::new(type_),
            }))
        },
        Rule::list => {
            let mut exprs = vec![];
            for pair in rules.into_inner() {
                exprs.push(build_ast(pair)?);
            }
            Ok(Expr::List(list::List {
                elems: exprs,
            }))
        },
        Rule::expression => todo!(),
        Rule::WHITESPACE => todo!(),
        Rule::value => todo!(),
    }
}



