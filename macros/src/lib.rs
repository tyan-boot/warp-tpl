use proc_macro::TokenStream;

use proc_macro2::{Ident, Span};
use quote::quote;
use syn::parse_macro_input;
use syn::parse_quote;
use syn::{FnArg, ItemFn, Pat, PatIdent, PathArguments, ReturnType, Signature, Type};

#[proc_macro_attribute]
pub fn route_wrapper(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as ItemFn);

    let mut wrapper_sig: Signature = input.sig.clone();

    let fn_name = format!("{}_wrapped", &input.sig.ident);
    let fn_name = Ident::new(&fn_name, Span::call_site());
    input.sig.ident = fn_name.clone();

    let ok_type = match &wrapper_sig.output {
        ReturnType::Default => panic!(),
        ReturnType::Type(_, ty) => match ty.as_ref() {
            Type::Path(tp) => {
                let arguments = &tp.path.segments.first().unwrap().arguments;
                match arguments {
                    PathArguments::AngleBracketed(arguments) => arguments.args.first().unwrap(),
                    _ => panic!(),
                }
            }
            _ => panic!(),
        },
    };

    wrapper_sig.output = parse_quote! { -> std::result::Result<#ok_type, warp::Rejection> };

    let args: Vec<&PatIdent> = input
        .sig
        .inputs
        .iter()
        .filter_map(|it| match it {
            FnArg::Receiver(_) => None,
            FnArg::Typed(pt) => match pt.pat.as_ref() {
                Pat::Ident(ident) => Some(ident),
                _ => None,
            },
        })
        .collect();

    TokenStream::from(quote! {
        #input

        #wrapper_sig {
            #fn_name(#(#args),*).await.map_err(warp::reject::custom)
        }
    })
}
