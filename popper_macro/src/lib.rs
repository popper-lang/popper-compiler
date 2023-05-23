use proc_macro::TokenStream;
use syn::parse_macro_input;
use quote::quote;

#[macro_export]
macro_rules! type_to_rs_type {
    ($type_:expr) => {
        match $type_ {
            Type::Int => i32,
            Type::String => String,
            Type::Bool => bool,
            Type::None => (),
            _ => panic!("Not implemented yet"),
        }
    };
}


#[proc_macro]
pub fn string_to_ident(ident: TokenStream) -> TokenStream {
    let string = parse_macro_input!(ident as syn::LitStr);
    let varname = syn::Ident::new(string.value().as_str(), string.span());
    let q = quote! {
        #varname
    };

    q.into()
}

#[proc_macro_attribute]
pub fn repr_function(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as syn::LitStr);
    let item = parse_macro_input!(item as syn::ItemFn);
    let name = item.sig.ident.clone();

    for arg in item.sig.inputs.iter() {
        let arg = match arg {
            syn::FnArg::Typed(arg) => arg,
            _ => panic!("Unexpected argument type"),
        };
        let arg_name = match arg.pat.as_ref() {
            syn::Pat::Ident(arg) => arg.ident.clone(),
            _ => panic!("Unexpected argument type"),
        };
        let arg_type = arg.ty.clone();
        let arg_type = quote! { #type_to_rs_type!(arg_type) };
        let arg_type = syn::parse_str::<syn::Type>(&arg_type.to_string()).unwrap();
    }
    let q = quote! {

    };

    q.into()
}
