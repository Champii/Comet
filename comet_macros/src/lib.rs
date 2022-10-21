//! Creates unique identifiers for macros using procedural macros and [UUID](https://crates.io/crates/uuid)
//! # Examples
//! ```
//!
//! macro_rules! gen_fn {
//!     ($a:ty, $b:ty) => {
//!         gensym::gensym!{ _gen_fn!{ $a, $b } }
//!     };
//! }
//!
//! macro_rules! _gen_fn {
//!     ($gensym:ident, $a:ty, $b:ty) => {
//!         fn $gensym(a: $a, b: $b) {
//!             unimplemented!()
//!         }
//!     };
//! }
//!
//! mod test {
//!     gen_fn!{ u64, u64 }
//!     gen_fn!{ u64, u64 }
//! }
//! ```
extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;

use quote::quote;
use syn::{parse::Result, parse_macro_input, parse_quote, Expr, ExprArray};

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn hash<T: Hash + ?Sized>(t: &T) -> String {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish().to_string()
}

#[proc_macro]
pub fn generate_msg(input: TokenStream) -> TokenStream {
    let mcall = parse_macro_input!(input as syn::Expr);

    proc_macro::TokenStream::from(
        generate_msg_call(mcall).unwrap_or_else(|e| syn::Error::to_compile_error(&e)),
    )
}
fn generate_msg_call(mcall: syn::Expr) -> Result<proc_macro2::TokenStream> {
    use quote::ToTokens;

    let args: ExprArray = parse_quote!(#mcall);

    let idents = args
        .elems
        .iter()
        .map(|x| syn::Ident::new(&format!("Event{}", hash(&x)), Span::call_site()))
        .collect::<Vec<_>>();

    let tt = quote! {
        #[derive(Clone)]
        pub enum Msg {
            #(#idents),*
        }
    };

    Ok(tt.into_token_stream())
}

#[proc_macro]
pub fn generate_update(input: TokenStream) -> TokenStream {
    let mcall = parse_macro_input!(input as syn::Expr);

    proc_macro::TokenStream::from(
        generate_update_call(mcall).unwrap_or_else(|e| syn::Error::to_compile_error(&e)),
    )
}
fn generate_update_call(mcall: syn::Expr) -> Result<proc_macro2::TokenStream> {
    use quote::ToTokens;

    let sym = syn::Ident::new(&format!("Event{}", hash(&mcall)), Span::call_site());

    let tt = quote! {
            Msg::#sym
    };

    Ok(tt.into_token_stream())
}

#[proc_macro]
pub fn gensym(input: TokenStream) -> TokenStream {
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
    let sym = syn::Ident::new(&format!("Event{}", hash(&first_arg)), Span::call_site());

    let mut inserted_gensym: proc_macro2::TokenStream = parse_quote!(#sym, );

    inserted_gensym.extend(mcall.tts);
    mcall.tts = inserted_gensym;

    Ok(mcall.into_token_stream())
}
