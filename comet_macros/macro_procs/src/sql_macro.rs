use lazy_static::lazy_static;
use proc_macro::TokenStream;
use std::sync::{Arc, RwLock};

use quote::quote;
use syn::{parse::Result, parse_macro_input, ImplItem};

lazy_static! {
    // TODO: replace with atomics
    pub static ref QUERIES: Arc<RwLock<u64>> = Arc::new(RwLock::new(0));
}

pub fn perform(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::ItemImpl);

    proc_macro::TokenStream::from(
        register_sql_queries(input).unwrap_or_else(|e| syn::Error::to_compile_error(&e)),
    )
}

pub fn register_sql_queries(mut mcall: syn::ItemImpl) -> Result<proc_macro2::TokenStream> {
    mcall.attrs.push(syn::parse_quote! {
        #[rpc]
    });

    // let self_type = *mcall.self_ty.clone();
    let res = mcall
        .items
        .iter()
        .map(|item| match item {
            ImplItem::Method(method) => register_sql_query(method).unwrap(),
            _ => unimplemented!(),
        })
        .collect::<Vec<_>>();

    mcall.items = res
        .iter()
        .flatten()
        .map(|item| syn::parse_quote! { #item })
        .collect();

    Ok(quote! {
        // #[rpc]
        #mcall
    })
}

pub fn register_sql_query(mcall: &syn::ImplItemMethod) -> Result<Vec<proc_macro2::TokenStream>> {
    let mut server_fn = mcall.clone();
    let client_fn = mcall.clone();
    let mut stmts = server_fn.block.stmts.clone();

    let last = stmts.pop().unwrap();
    let server_wrap: syn::Block = syn::parse_quote! { {
            #(#stmts)*
            let query = #last;
             let mut conn = crate::establish_connection();
            let res = query.load::<Self>(&mut conn).unwrap();
            res
        }
    };

    // let mut sql_method = mcall.clone();

    // sql_method.sig.decl.output = syn::ReturnType::Default;

    server_fn.block = server_wrap;

    Ok(vec![
        quote! {
            #[cfg(not(target_arch = "wasm32"))]
            #server_fn
        },
        quote! {
            #[cfg(target_arch = "wasm32")]
            #client_fn
        },
    ])
}
