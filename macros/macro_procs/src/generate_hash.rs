use proc_macro::TokenStream;
use proc_macro2::Span;

use syn::{parse::Result, parse_macro_input, parse_quote, Expr};

pub fn perform(input: TokenStream) -> TokenStream {
    let mcall = parse_macro_input!(input as syn::Macro);

    proc_macro::TokenStream::from(
        insert_macro_arg(mcall).unwrap_or_else(|e| syn::Error::to_compile_error(&e)),
    )
}

fn insert_macro_arg(mut mcall: syn::Macro) -> Result<proc_macro2::TokenStream> {
    use quote::ToTokens;

    let first_arg: Expr = mcall.parse_body().unwrap();

    let sym = syn::Ident::new(
        &format!("Event{}", crate::utils::hash(&first_arg)),
        Span::call_site(),
    );

    let mut inserted_ident: proc_macro2::TokenStream = parse_quote!(#sym, );

    inserted_ident.extend(mcall.tokens);
    mcall.tokens = inserted_ident;

    Ok(mcall.into_token_stream())
}
