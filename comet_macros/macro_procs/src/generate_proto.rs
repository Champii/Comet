use itertools::Itertools;
use lazy_static::lazy_static;
use proc_macro::TokenStream;
use proc_macro2::Span;
use std::sync::{Arc, RwLock};

use quote::quote;
use syn::{parse::Result, parse_macro_input, parse_quote, ExprArray};

pub fn perform(input: TokenStream) -> TokenStream {
    proc_macro::TokenStream::from(
        exprs_to_idents(input).unwrap_or_else(|e| syn::Error::to_compile_error(&e)),
    )
}

pub fn exprs_to_idents(mcall: TokenStream) -> Result<proc_macro2::TokenStream> {
    use quote::ToTokens;

    let (models, inner): (Vec<_>, Vec<_>) = crate::db_macro::MODELS
        .read()
        .unwrap()
        .iter()
        .map(|name| {
            (
                syn::Ident::new(name, Span::call_site()),
                syn::Ident::new(&format!("{}Proto", name).to_string(), Span::call_site()),
            )
        })
        .unzip();

    let tt = quote! {
        enum Proto {
            #(#models(#inner)),*
        }
    };

    Ok(tt.into_token_stream())
}
