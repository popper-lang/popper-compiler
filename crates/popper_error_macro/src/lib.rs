use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::*;
use syn::punctuated::Punctuated;

struct ExprList {
    punctuated: Punctuated<Expr, Token![,]>,
}

use syn::parse::{Parse, ParseStream};

impl Parse for ExprList {
    fn parse(input: ParseStream) -> Result<Self> {
        let punctuated = Punctuated::parse_terminated(input)?;
        Ok(ExprList { punctuated })
    }
}

/// Derive the `Diagnostic` trait for a struct.
/// require "message" , "code"
#[proc_macro_derive(Diagnostics, attributes(message, code, label, span, note))]
pub fn derive_diagnostics(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let fields = match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => fields.named.iter().collect::<Vec<_>>(),
            Fields::Unit => vec![],
            Fields::Unnamed(_) => panic!("unnamed fields are not supported"),
        },
        Data::Enum(_) => return generate_enum_input(&input),
        _ => unimplemented!(),
    };

    let message = input
        .attrs
        .iter()
        .find_map(|attr| {
            if attr.path().is_ident("message") {
                let meta = attr.meta.clone();
                match meta {
                    Meta::NameValue(MetaNameValue {
                        value: Expr::Lit(lit),
                        ..
                    }) => Some(
                        quote! {
                            #lit
                        }
                    ),
                    Meta::List(list) => {
                        // parse <string>, e.g. "expected token `{0}` but found `{1}`",
                        // where `{0}` and `{1}` are the fields of the struct
                        let token_stream = list.tokens;

                        let format: ExprList = parse2(token_stream)
                            .expect("message attribute must be a string or a list of expressions");

                        let format = format.punctuated;

                        Some(quote! {
                            #format
                        })

                    }
                    _ => None,
                }
            } else {
                None
            }
        })
        .expect("message attribute is required");

    let code = input
        .attrs
        .iter()
        .find_map(|attr| {
            if attr.path().is_ident("code") {
                let meta = attr.meta.clone();
                match meta {
                    Meta::NameValue(MetaNameValue {
                        value: Expr::Lit(lit),
                        ..
                    }) => Some(lit.to_token_stream().to_string()),
                    _ => None,
                }
            } else {
                None
            }
        })
        .expect("code attribute is required");

    let label = input
        .attrs
        .iter()
        .find_map(|attr| {
            if attr.path().is_ident("label") {
                let meta = attr.meta.clone();
                match meta {
                    Meta::NameValue(MetaNameValue {
                        value: Expr::Lit(lit),
                        ..
                    }) => Some(lit),
                    _ => None,
                }
            } else {
                None
            }
        })
        .expect("label attribute is required");

    let span = fields
        .iter()
        .find_map(|field| {
            field.attrs.iter().find_map(|attr| {
                if attr.path().is_ident("span") {
                    Some(&field.ident)
                } else {
                    None
                }
            })
        })
        .expect("span attribute is required");

    let note = input.attrs.iter().find_map(|attr| {
        if attr.path().is_ident("note") {
            let meta = attr.meta.clone();
            match meta {
                Meta::NameValue(MetaNameValue {
                    value: Expr::Lit(lit),
                    ..
                }) => Some(lit),
                _ => None,
            }
        } else {
            None
        }
    });

    let span = quote! {
        self.#span
    };

    let new_fields: Vec<_> = fields
        .iter()
        .map(|field| {
            let ident = field.ident.as_ref().unwrap();
            quote! {
                let #ident = &self.#ident;
            }
        })
        .collect();

    let note = note
        .map(|lit| {
            let lit = lit.lit;
            quote! {
                #(#new_fields)*
                Some(format!(#lit))
            }
        })
        .unwrap_or(quote! { None });

    let expanded = quote! {
        impl Diagnostics for #name {
            fn message(&self) -> String {
                #(#new_fields)*
                format!(#message)
            }

            fn code(&self) -> u32 {
                #(#new_fields)*
                format!(#code).parse().unwrap()
            }

            fn label(&self) -> String {
                #(#new_fields)*
                format!(#label)
            }

            fn span(&self) -> popper_ast::ast::Span {
                #span
            }

            fn note(&self) -> Option<String> {
                #note
            }
        }
    };

    expanded.into()
}


fn generate_enum_input(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;
    let variants = match input.data {
        Data::Enum(ref data) => data.variants.iter().collect::<Vec<_>>(),
        _ => panic!("Diagnostics can only be derived for enums"),
    };
    
    let m = variants.into_iter().map(|variant| {
        if let Fields::Unnamed(ref unnamed) = variant.fields {
            let fields: Vec<_> = unnamed.unnamed.iter().collect();
            if fields.len() != 1 {
                panic!("Diagnostics enum variants must have exactly one unnamed field");
            }
            let variant_name = &variant.ident;
            return quote! {
                #name::#variant_name(ref _0)
            }
        };
        panic!("Diagnostics enum variants must have exactly one unnamed field");
    }).collect::<Vec<_>>();
    
    let expanded = quote! {
        impl Diagnostics for #name {
            fn message(&self) -> String {
                match self {
                    #(#m => _0.message(),)*
                }
            }

            fn code(&self) -> u32 {
                match self {
                    #(#m => _0.code(),)*
                }
            }

            fn label(&self) -> String {
                match self {
                    #(#m => _0.label(),)*
                }
            }

            fn span(&self) -> popper_ast::ast::Span {
                match self {
                    #(#m => _0.span(),)*
                }
            }

            fn note(&self) -> Option<String> {
                match self {
                    #(#m => _0.note(),)*
                }
            }
        }
    };
    
    expanded.into()

}
