use proc_macro2::{Ident, Span};
use quote::{format_ident, quote};
use crate::ast::expr::ExprType;
use crate::ast::stmt::StmtType;


pub fn generate_stmt_code(stmt: StmtType) -> proc_macro2::TokenStream {

    match stmt {
        StmtType::Let { name, type_, value, mutable } => {
            let name = Ident::new(name.lexeme.as_str(), Span::call_site());
            let mutable_token = if mutable {
                Some(quote!{ mut } )
            } else {
                None
            };

            let value_code = match value {
                Some(expr) => {
                    let expr = generate_expr_code(*expr.expr_type);
                    quote! { #expr }
                },
                None => quote! {},
            };

            let type_code = match type_ {
                Some(expr) => {
                    let expr = generate_expr_code(*expr.expr_type);
                    quote! { #expr }
                },
                None => quote! {},
            };

            return quote! {
                let #mutable_token #name #type_code #value_code;
            }
        }
    }
    quote! {

    }
}

pub fn generate_expr_code(expr: ExprType) -> proc_macro2::TokenStream {
    quote! {}
}
