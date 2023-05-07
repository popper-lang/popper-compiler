use proc_macro::TokenStream;
use syn::{DeriveInput, Expr, ExprStruct, Member, parse_macro_input, FieldsNamed};
use quote::quote;

#[proc_macro]
pub fn function_to_rs_fn(input: TokenStream) -> TokenStream {
    let tokens = parse_macro_input!(input as ExprStruct);
    dbg!(&tokens);
    let k = quote! {
        #tokens
    };

    k.into()
}

#[proc_macro]
pub fn stmt_to_rs(input: TokenStream) -> TokenStream {
    let tokens = parse_macro_input!(input as ExprStruct);
    if tokens.path.segments.first().unwrap().ident != "StmtType" {
        panic!("expected StmtType")
    }

    match dbg!(tokens.path.segments.last().unwrap().ident.to_string().as_str()) {
        "Expression" => {
            let field_value = tokens.fields.first().unwrap();
            let expr = field_value.expr.clone();
            match expr {
                Expr::Call(call) => {

                }
                _ => panic!("what?")
            }

        },
        _ => unreachable!()

    }
    let k = quote! {
        #tokens
    };

    k.into()
}

#[proc_macro]
pub fn let_stmt_to_rs(input: TokenStream) -> TokenStream {
    let field_named = parse_macro_input!(input as FieldsNamed);

    let n = quote! {
        #field_named
    };

    n.into()
}