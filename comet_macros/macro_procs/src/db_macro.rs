use itertools::Itertools;
use lazy_static::lazy_static;
use proc_macro::TokenStream;
use proc_macro2::Span;
use std::sync::{Arc, RwLock};

use quote::quote;
use syn::{parse::Result, parse_macro_input, parse_quote, ExprArray};

lazy_static! {
    pub static ref MODELS: Arc<RwLock<Vec<String>>> = Arc::new(RwLock::new(Vec::new()));
}

pub fn perform(input: TokenStream) -> TokenStream {
    let mcall = parse_macro_input!(input as syn::ItemStruct);

    let mut stream = proc_macro::TokenStream::from(
        impl_proto(mcall.clone()).unwrap_or_else(|e| syn::Error::to_compile_error(&e)),
    );

    stream.extend(proc_macro::TokenStream::from(
        impl_model(mcall).unwrap_or_else(|e| syn::Error::to_compile_error(&e)),
    ));

    stream
}

pub fn impl_proto(mcall: syn::ItemStruct) -> Result<proc_macro2::TokenStream> {
    use quote::ToTokens;

    let name = mcall.ident.clone();
    let proto_name = syn::Ident::new(&format!("{}Proto", name).to_string(), Span::call_site());
    let name_id = syn::Ident::new(&format!("{}Id", name).to_string(), Span::call_site());

    MODELS.write().unwrap().push(name.to_string());

    let tt = quote! {
        pub struct #name_id(i32);

        use std::ops::Deref;

        impl Deref for #name_id {
            type Target = i32;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        enum #proto_name {
            New(#name),
            Fetch(#name_id),
            List,
            Update(#name),
            Delete(#name_id),
        }
    };

    Ok(tt.into_token_stream())
}

pub fn impl_model(mcall: syn::ItemStruct) -> Result<proc_macro2::TokenStream> {
    use quote::ToTokens;

    let name = mcall.ident.clone();
    let name_id = syn::Ident::new(&format!("{}Id", name).to_string(), Span::call_site());

    let tt = quote! {
        impl #name {
            pub fn new(model: Self) {
            }
            pub fn fetch(&self) {
            }
            pub fn list(&self) {
            }
            pub fn update(&self) {
            }
            pub fn delete(&self) {
            }
        }
    };

    Ok(tt.into_token_stream())
}
