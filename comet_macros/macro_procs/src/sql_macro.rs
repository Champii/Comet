use lazy_static::lazy_static;
use proc_macro::TokenStream;
use std::sync::{Arc, RwLock};

use quote::quote;
use syn::{parse::Result, parse_macro_input};

lazy_static! {
    // TODO: replace with atomics
    pub static ref QUERIES: Arc<RwLock<u64>> = Arc::new(RwLock::new(0));
}

pub fn perform(input: TokenStream) -> TokenStream {
    let mcall = parse_macro_input!(input as syn::ItemFn);

    proc_macro::TokenStream::from(
        register_sql_query(mcall).unwrap_or_else(|e| syn::Error::to_compile_error(&e)),
    )
}

pub fn register_sql_query(mut mcall: syn::ItemFn) -> Result<proc_macro2::TokenStream> {
    let mut server_fn = mcall.clone();
    let mut client_fn = mcall.clone();
    let mut stmts = server_fn.block.stmts.clone();

    /* mcall.attrs.push(syn::parse_quote! {
        #[rpc]
    }); */
    println!("stmts: {:#?}", stmts);

    let last = stmts.pop().unwrap();

    let wrap: syn::Block = syn::parse_quote! {
        {
            // #(#stmts)*
            /* let query = #last;
            let conn = crate::db::get_connection();
            les res = query.execute::<Self>(&mut conn).unwrap();
            res */
            2
        }
    };

    *server_fn.block = wrap;

    Ok(quote! {
        #[cfg(not(target_arch = "wasm32"))]
        #server_fn

        #[cfg(target_arch = "wasm32")]
        #client_fn
    })
}
