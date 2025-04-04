

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::*;

#[proc_macro_derive(Diagnostics, attributes(message, code, label, span))]
pub fn derive_diagnostics(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let fields = match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => fields.named.iter().collect::<Vec<_>>(),
            Fields::Unit => vec![],
            Fields::Unnamed(_) => panic!("unnamed fields are not supported"),
        },
        _ => unimplemented!(),
    };

    let message = input.attrs.iter().find_map(|attr| {
        if attr.path().is_ident("message") {
            let meta = attr.meta.clone();
            match meta {
                Meta::NameValue(MetaNameValue { value: Expr::Lit(lit), .. }) => Some(lit.lit),
                _ => None,
            }
        } else {
            None
        }
    }).expect("message attribute is required");

    let code = input.attrs.iter().find_map(|attr| {
        if attr.path().is_ident("code") {
            let meta = attr.meta.clone();
            match meta {
                Meta::NameValue(MetaNameValue { value: Expr::Lit(lit), .. }) => Some(lit.to_token_stream().to_string()),
                _ => None,
            }
        } else {
            None
        }
    }).expect("code attribute is required");

    let label = input.attrs.iter().find_map(|attr| {
        if attr.path().is_ident("label") {
            let meta = attr.meta.clone();
            match meta {
                Meta::NameValue(MetaNameValue { value: Expr::Lit(lit), .. }) => Some(lit),
                _ => None,
            }
        } else {
            None
        }
    }).expect("label attribute is required");

    let span = fields.iter().find_map(|field| {
        field.attrs.iter().find_map(|attr| {
            if attr.path().is_ident("span") {
                Some(&field.ident)
            } else {
                None
            }
        })
    }).expect("span attribute is required");

    let note = input.attrs.iter().find_map(|attr| {
        if attr.path().is_ident("note") {
            let meta = attr.meta.clone();
            match meta {
                Meta::NameValue(MetaNameValue { value: Expr::Lit(lit), .. }) => Some(lit),
                _ => None,
            }
        } else {
            None
        }
    });

    let span = quote! {
        self.#span
    };

    let new_fields: Vec<_> = fields.iter().map(|field| {
        let ident = field.ident.as_ref().unwrap();
        quote! {
        let #ident = &self.#ident;
    }
    }).collect();

    let expanded = quote! {
        impl Diagnostics for #name {
            fn message(&self) -> &str {
                #(#new_fields)*
                format!(#message)
            }

            fn code(&self) -> u32 {
                #(#new_fields)*
                format!(#code).parse().unwrap()
            }

            fn label(&self) -> &str {
                #(#new_fields)*
                format!(#label)
            }

            fn span(&self) -> Span {
                #span
            }

            fn note(&self) -> Option<&str> {
                #note
            }
        }
    };

    expanded.into()
}
