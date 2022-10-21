use proc_macro::TokenStream;
use proc_macro2::Span;

use quote::quote;
use syn::{parse::Result, parse_macro_input, parse_quote, ExprArray};

pub fn perform(input: TokenStream) -> TokenStream {
    let mcall = parse_macro_input!(input as syn::Expr);

    proc_macro::TokenStream::from(
        exprs_to_idents(mcall).unwrap_or_else(|e| syn::Error::to_compile_error(&e)),
    )
}

pub fn exprs_to_idents(mcall: syn::Expr) -> Result<proc_macro2::TokenStream> {
    use quote::ToTokens;

    let args: ExprArray = parse_quote!(#mcall);

    let idents = args
        .elems
        .iter()
        .map(|x| {
            syn::Ident::new(
                &format!("Event{}", crate::utils::hash(&x)),
                Span::call_site(),
            )
        })
        .collect::<Vec<_>>();

    let tt = quote! {
        #[derive(Clone)]
        pub enum Msg {
            #(#idents),*
        }
    };

    Ok(tt.into_token_stream())
}
