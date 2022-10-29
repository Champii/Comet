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

    mcall.items = res.iter().map(|item| syn::parse_quote! { #item }).collect();

    Ok(quote! {
        // #[rpc]
        #mcall
    })
}

pub fn register_sql_query(mcall: &syn::ImplItemMethod) -> Result<proc_macro2::TokenStream> {
    let mut server_fn = mcall.clone();
    let mut stmts = server_fn.block.stmts.clone();

    let last = stmts.pop().unwrap();

    let server_wrap: syn::Block = syn::parse_quote! {
        {
            #(#stmts)*
            let query = #last;
             let mut conn = crate::establish_connection();
            let res = query.load::<Self>(&mut conn).unwrap();
            res
        }
    };

    server_fn.block = server_wrap;

    Ok(quote! {
        #server_fn
    })
}

pub fn perform_watch(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::ImplItemMethod);

    proc_macro::TokenStream::from(
        register_sql_watch(input).unwrap_or_else(|e| syn::Error::to_compile_error(&e)),
    )
}
pub fn register_sql_watch(mcall: syn::ImplItemMethod) -> Result<proc_macro2::TokenStream> {
    if mcall
        .attrs
        .iter()
        .position(|attr| attr.tts.to_string() == "(target_arch = \"wasm32\")")
        .is_some()
    {
        return Ok(quote! { #mcall });
    }

    let mut server_fn = mcall.clone();
    let stmts = server_fn.block.stmts.clone();

    let (orig_stmts, added_stmts) = stmts.split_at(stmts.len() - 3);
    let orig_stmts = orig_stmts.to_vec();
    let added_stmts = added_stmts.to_vec();

    let server_wrap: syn::Block = syn::parse_quote! {
        {
            #(#orig_stmts)*

            let query_str = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
            let strs = query_str.split("--").collect::<Vec<_>>();
            let mut query_str = strs[0].to_string();
            let binds_str = strs[1].to_string();
            let binds_vec = binds_str.strip_prefix(" binds: [").unwrap().strip_suffix("]").unwrap().split(",");
            binds_vec.enumerate().for_each(|(id, bind)| {
                query_str = query_str.replace(&format!("${}", id + 1), bind);
            });
            query_str = query_str.replace("\"", "");

            use reactive_pg::{Event, watch};

            let handler = watch::<Self>(
                &query_str,
                Box::new(move |events| {
                    for event in events {
                        match event {
                            Event::Insert(row) => println!("insert: {:?}", row),
                            Event::Update(row) => println!("change: {:?}", row),
                            Event::Delete(id) => println!("delete: {:?}", id),
                        }
                    }
                }),
            )
            .await;
            #(#added_stmts)*
        }
    };

    server_fn.block = server_wrap;

    Ok(quote! {
        #server_fn
    })
}
