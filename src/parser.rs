

use pest::iterators::Pair;

use crate::ast::Expr;
use crate::expr::*;
use crate::expr::ident::Ident;
use crate::expr::literal::LiteralType;
use crate::value::Type;
use crate::ast::Op;

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
        Rule::string => {
            let s = rules.as_str()[1..rules.as_str().len() - 1].to_string();
            Ok(Expr::Literal(literal::Literal(LiteralType::String(s))))
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
            build_ast(rules.into_inner().next().unwrap())
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
        Rule::fun_statement => {
            let name = rules.clone().into_inner().next().unwrap().as_str();
            let mut args = vec![];
            let filrered = rules.clone().into_inner().skip(1).filter(|pair| pair.as_rule() != Rule::block);
            let list_type = filrered.clone().filter(|pair| pair.as_rule() == Rule::type_builtin);
            let list_ident = filrered.clone().filter(|pair| pair.as_rule() == Rule::ident);
            let group = list_ident.zip(list_type);
            for (ident, type_) in group {
                args.push((match build_ast(ident)? {
                    Expr::Ident(i) => i,
                    e => panic!("expected ident found {:?}", e),
                }, build_ast(type_)?));
            }

            let block = build_ast(rules.clone().into_inner().skip_while(|e| e.as_rule() != Rule::assign_op).nth(1).unwrap())?;
            return Ok(Expr::FunDef(fundef::FunDef {
                name: name.to_string(),
                args: args,
                body: Box::new(block),
            }))

        },
        Rule::call_expression => {
            let name = rules.clone().into_inner().next().unwrap().as_str();
            let mut args = vec![];
            for pair in rules.clone().into_inner().skip(1) {
                args.push(build_ast(pair)?);
            }
            Ok(Expr::Call(call::Call {
                name: name.to_string(),
                args: args,
            }))
        },
        Rule::struct_statement => {
            let name = rules.clone().into_inner().next().unwrap().as_str();
            let mut fields = vec![];
            let iter = rules
            .clone()
            .into_inner()
            .skip(1)
            .filter(
                |pair| pair.as_rule() == Rule::ident
            )
            .zip(
                rules.clone().into_inner().filter(
                    |pair| pair.as_rule() == Rule::type_builtin
                )
            );
            for (ident, type_) in iter {
                fields.push((ident::Ident(ident.as_str().to_string()), build_ast(type_)?))
            }
            
            Ok(Expr::StructDef(structdef::StructDef {
                name: name.to_string(),
                fields: fields,
            }))
        },
        Rule::init_struct_expression => {
            let name = rules.clone().into_inner().next().unwrap().as_str().to_string();
            let mut name_attr = Vec::new();
            let mut value = Vec::new();
            let iter_one = rules.clone().into_inner().skip(1).step_by(2);
            let iter_two =  rules.clone().into_inner().step_by(2).skip(1);
            for i in iter_one {
                name_attr.push(match build_ast(i)? {
                    Expr::Ident(i) => i,
                    e => { 
                        panic!("expected ident, found {:#?}", e);
                    }
                })
            }
            for n in iter_two {
                value.push(build_ast(n)?);
            }
            Ok(Expr::CallStruct(callstruct::CallStruct {
                name,
                args:  name_attr.into_iter().zip(value.into_iter()).collect()
            }))
        },
        Rule::impl_statement => {
            let name = rules.clone().into_inner().next().unwrap();
            let func = match build_ast(rules.into_inner().nth(1).unwrap())? {
                Expr::FunDef(f) => f,
                e => panic!("expected function found {:#?}", e)
            };
            Ok(Expr::Impl(impl_::Impl {
                name_struct: name.as_str().to_string(),
                name_method: func.name,
                args: func.args,
                body: func.body
            }))
        },
        Rule::attr_expression => {
            let struct_name = rules.clone().into_inner().next().unwrap();
            let attr_name = rules.into_inner().nth(1).unwrap();

            Ok(Expr::GetAttr(getattr::GetAttr {
                name: Box::new(build_ast(struct_name)?),
                attr: match build_ast(attr_name)? {
                    Expr::Ident(ident::Ident(i)) => i,
                    e => panic!("expected ident found {:#?}", e)
                }
            }))
        }
        Rule::range_expression => {
            let start = build_ast(rules.clone().into_inner().next().unwrap())?;
            let end = build_ast(rules.clone().into_inner().nth(1).unwrap())?;
            Ok(Expr::Range(range::Range {
                start: Box::new(start),
                end: Box::new(end),
            }))
        },
        Rule::typeof_expression => {
            let value = build_ast(rules.clone().into_inner().next().unwrap())?;
            Ok(Expr::Typeof(typeof_::Typeof {
                value: Box::new(value),
            }))
        },
        Rule::expression => {
            todo!()
        },
        Rule::parent_expression => {
            build_ast(rules.clone().into_inner().next().unwrap())
        },
        Rule::call_attr_expression => {
            let name = rules.clone().into_inner().next().unwrap();
            let attr = rules.clone().into_inner().nth(1).unwrap();
            let attr_string = match attr.as_str() {
                "+" => "add",
                "-" => "sub",
                "*" => "mul",
                "/" => "div",
                ">" => "gt",
                "<" => "lt",
                ">=" => "ge",
                "<=" => "le",
                "==" => "eq",
                "!=" => "ne",
                "^" => "pow",
                "&&" => "and",
                "||" => "or",
                e => e
            };
            let mut args = vec![];
            for pair in rules.clone().into_inner().skip(2) {
                args.push(build_ast(pair)?);
            }
            Ok(Expr::GetFunc(getfunc::GetFunc {
                name: Box::new(build_ast(name)?),
                func: attr_string.to_string(),
                args: args,
            }))
        },
        Rule::WHITESPACE => todo!(),
        Rule::value => todo!(),
        Rule::declaration_attr => todo!(),
        //Rule::EOI => Ok(Expr::Empty),
        Rule::op => todo!(),
        Rule::program => todo!(),
        Rule::index_expression => {
            let name = rules.clone().into_inner().next().unwrap();
            let index = rules.clone().into_inner().nth(1).unwrap();
            Ok(Expr::Index(index::Index {
                name: Ident(name.as_str().to_string()),
                index: Box::new(build_ast(index)?),
            }))
        },
        Rule::use_statement => {
            let name = rules.clone().into_inner().next().unwrap();
            let as_name = rules.clone().into_inner().nth(1).unwrap();
            Ok(Expr::Module(module::Module {
                name: name.as_str()[1..name.as_str().len()-1].to_string(),
                as_name: as_name.as_str().to_string(),
            }))
        },
        Rule::int_type => {
            Ok(Expr::TypeExpr(type_::TypeExpr(Type::Int)))
        },
        Rule::string_type => {
            Ok(Expr::TypeExpr(type_::TypeExpr(Type::String)))
        },
        Rule::bool_type => {
            Ok(Expr::TypeExpr(type_::TypeExpr(Type::Bool)))
        },
        Rule::list_type => {
            Ok(Expr::TypeExpr(type_::TypeExpr(Type::List)))
        },
        Rule::assign_op => todo!(),
        Rule::COMMENT => todo!(),
    }
}



