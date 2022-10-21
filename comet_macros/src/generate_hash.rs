//! Creates unique identifiers for macros using procedural macros and [UUID](https://crates.io/crates/uuid)
//! # Examples
//! ```
//!
//! macro_rules! gen_fn {
//!     ($a:ty, $b:ty) => {
//!         comet_macros::generate_hash!{ _gen_fn!{ $a, $b } }
//!     };
//! }
//!
//! macro_rules! _gen_fn {
//!     ($hash:ident, $a:ty, $b:ty) => {
//!     
//!     };
//! }
//!
//! mod test {
//!     gen_fn!{ u64, u64 }
//!     gen_fn!{ u64, u64 }
//! }
//! ```
//!
use proc_macro::TokenStream;
use proc_macro2::Span;

use syn::{parse::Result, parse_macro_input, parse_quote, Expr};

pub fn perform(input: TokenStream) -> TokenStream {
    //! generate a unique identifier with a span of `span::call_site` and
    //! insert it as the first argument to a macro call followed by a comma.

    let mcall = parse_macro_input!(input as syn::Macro);

    proc_macro::TokenStream::from(
        alter_macro(mcall).unwrap_or_else(|e| syn::Error::to_compile_error(&e)),
    )
}

fn alter_macro(mut mcall: syn::Macro) -> Result<proc_macro2::TokenStream> {
    use quote::ToTokens;

    let first_arg: Expr = mcall.parse_body().unwrap();
    let sym = syn::Ident::new(
        &format!("Event{}", crate::utils::hash(&first_arg)),
        Span::call_site(),
    );

    let mut inserted_gensym: proc_macro2::TokenStream = parse_quote!(#sym, );

    inserted_gensym.extend(mcall.tts);
    mcall.tts = inserted_gensym;

    Ok(mcall.into_token_stream())
}
