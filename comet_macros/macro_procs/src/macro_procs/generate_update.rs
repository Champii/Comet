use proc_macro::TokenStream;
use proc_macro2::Span;

use quote::quote;
use syn::{parse::Result, parse_macro_input};

pub fn perform(input: TokenStream) -> TokenStream {
    let mcall = parse_macro_input!(input as syn::Expr);

    proc_macro::TokenStream::from(
        generate_update_row(mcall).unwrap_or_else(|e| syn::Error::to_compile_error(&e)),
    )
}
fn generate_update_row(mcall: syn::Expr) -> Result<proc_macro2::TokenStream> {
    use quote::ToTokens;

    let sym = syn::Ident::new(
        &format!("Event{}", crate::utils::hash(&mcall)),
        Span::call_site(),
    );

    let tt = quote! {
            Msg::#sym
    };

    Ok(tt.into_token_stream())
}
