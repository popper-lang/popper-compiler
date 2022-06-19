
use lalrpop_util::lalrpop_mod;
use crate::executer::value::Type;
use crate::tree::Expr;
use crate::tree::Literal;
use crate::tree::Op;
use crate::tree::IOp;
use crate::executer::Vm;
use crate::executer::value;
use std::fs;

lalrpop_mod!(pub popper);

#[cfg(test)]
fn test_value(path: &str, value_tested: value::Value) {
    // WARNING: THIS IS NOT A TEST
    let contents = fs::read_to_string(path)
        .expect("Something went wrong reading the file");
    let exprs = popper::ExprsParser::new().parse(&contents);
    match exprs {
        Ok(exprs) => {
            let mut vm = Vm::new();
            let value = vm.eval_expr(exprs);
            match value {
                Ok(value) => assert_eq!(value_tested, value),
                Err(err) => {
                    panic!("erreur: {:?}", err);
                }
            };
        }
        Err(e) => {
            println!("erreur: {:?}", e);
        }
    }
}

#[cfg(test)]
fn assert_expr_eq(string: &str, expr_tested: Expr) {
    let expr;
    match popper::ExprsParser::new().parse(string) {
        Ok(o) => {
            expr = o;
        },
        Err(_) => {
            assert!(false);
            return
        }
    };

    assert_eq!(
        expr, 
        expr_tested
    )
}

#[test]
fn test_def_var_syntax() {
    assert!(matches!(popper::ExprsParser::new().parse("let = 5"), Err(_)))
}

#[test]
fn test_def_var_expr() {
    assert_expr_eq(
        "let a = 5",
        Expr::Block { 
            body: vec![
                Expr::Assign { 
                    name: "a".to_string(),
                    value: Box::new(
                        Expr::Literal { value: Literal::Number(5.0) } 
                    ),
                    mutable: true,
                    type_: Some(Type::Int)
                }
            ] 
        }
    )
}

#[test]
fn test_op_syntax() {
    assert!(matches!(popper::ExprsParser::new().parse("2 + "), Err(_)))
}

#[test]
fn test_op_expr() {
    assert_expr_eq("5+8", Expr::Block { body: 
        vec![
            Expr::BinOp { 
                op: Op::Add, 
                left: Box::new(Expr::Literal {value: Literal::Number(5.)}), 
                right: Box::new(Expr::Literal {value: Literal::Number(8.)})
            }
        ]
    })

}

#[test]
fn test_cmp_op_syntax() {
    assert!(matches!(popper::ExprsParser::new().parse("2 > "), Err(_)))
}

#[test]
fn test_cmp_op_expr() {
    assert_expr_eq("5 < 8", Expr::Block { body: 
        vec![
            Expr::BinOp { 
                op: Op::Lt, 
                left: Box::new(Expr::Literal {value: Literal::Number(5.)}), 
                right: Box::new(Expr::Literal {value: Literal::Number(8.)})
            }
        ]
    })
}

#[test]
fn test_iop_syntax() {
    assert!(matches!(popper::ExprsParser::new().parse("3 += 5"), Err(_)))
}

#[test]
fn test_iop_expr() {
    assert_expr_eq("a += 8", Expr::Block { body: 
        vec![
            Expr::IOp { 
                op: IOp::IAdd, 
                value: Box::new(
                    Expr::Literal { value: Literal::Number(8.) }
                ),
                name: "a".to_string() }
        ]
    })
}

#[test]
fn test_if_block_syntax() {
    assert!(matches!(popper::ExprsParser::new().parse("if a == 5 {"), Err(_)))
}

#[test]
fn test_if_block_expr() {
    assert_expr_eq("if a { b }", Expr::Block { body: 
        vec![
            Expr::IfThen { 
                cond: Box::new(Expr::Ident {ident: "a".to_string()}),
                then: Box::new(
                    Expr::Block { body: vec![
                        Expr::Ident { ident: "b".to_string() }
                        ] 
                    }
                )
            } 
        ]
    })
}

#[test]
fn test_while_block_syntax() {
    assert!(matches!(popper::ExprsParser::new().parse("while {}"), Err(_)))
}

#[test]
fn test_while_block_expr() {
    assert_expr_eq("while a { b }", Expr::Block { body: 
        vec![
            Expr::While {
                 cond: Box::new(
                     Expr::Ident { ident: "a".to_string() }
                ),
                body: Box::new(
                    Expr::Block { 
                        body: 
                        vec![
                            Expr::Ident { ident: "b".to_string() }
                        ] 
                    }
                ) 
            }
        ]
    })
}

#[test]
fn test_func_syntax() {
    assert!(matches!(popper::ExprsParser::new().parse("def () {}"), Err(_)))
}

#[test]
fn test_func_expr() {
    assert_expr_eq("def a(b) { c }", Expr::Block { body: vec![
        Expr::FunDef { name: "a".to_string(),
                       args: vec![
                           Expr::Ident { ident: "b".to_string() }
                        ], 
                        body: Box::new(
                            Expr::Block { body: vec![
                                Expr::Ident {ident: "c".to_string()}
                            ]}
                        ) 
                    }
        ]})
}

#[test] 
fn test_def_var_value() {
    test_value("/Users/antoine/Documents/tlang/src/tlang_asset/test_def_var.txt", value::Value::Number(5.))
}

#[test] 
fn test_op_value() {
    test_value("/Users/antoine/Documents/tlang/src/tlang_asset/test_op.txt", value::Value::Number(8.))
}

#[test]
fn test_cmp_op_value() {
    test_value("/Users/antoine/Documents/tlang/src/tlang_asset/test_cmp_op.txt", value::Value::Bool(true))
}
