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
    let query_id = QUERIES.read().unwrap().clone();

    *QUERIES.write().unwrap() = query_id + 1;

    if mcall
        .attrs
        .iter()
        .position(|attr| attr.tts.to_string() == "(target_arch = \"wasm32\")")
        .is_some()
    {
        //TODO: generate call cache here
        generate_client_sql_watch(mcall, query_id)
    } else {
        generate_server_sql_watch(mcall, query_id)
    }
}

pub fn generate_server_sql_watch(
    mcall: syn::ImplItemMethod,
    _query_id: u64,
) -> Result<proc_macro2::TokenStream> {
    let mut server_fn = mcall.clone();
    let stmts = server_fn.block.stmts.clone();

    let (query, rest) = stmts.split_at(stmts.len() - 4);

    let server_wrap: syn::Block = syn::parse_quote! {
        {
            #(#query)*

            let query_str = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
            let strs = query_str.split("--").collect::<Vec<_>>();
            let mut query_str = strs[0].to_string();
            let binds_str = strs[1].to_string();
            let binds_vec = binds_str.strip_prefix(" binds: [").unwrap().strip_suffix("]").unwrap().split(",");
            binds_vec.enumerate().for_each(|(id, bind)| {
                query_str = query_str.replace(&format!("${}", id + 1), bind);
            });
            query_str = query_str.replace("\"", "");

            use reactive_pg::{Event as PgEvent, watch};

            let client2 = client.clone();

            let handle = watch::<Self>(
                &query_str,
                Box::new(move |events| {
                    let new_events = events.clone().into_iter().map(Event::from).collect::<Vec<_>>();

                    let client3 = client2.clone();

                    tokio::task::spawn(async move {
                        client3
                            .send(
                                Proto::Event(request_id, new_events),
                            )
                        .await
                    });
                }),
            )
            .await;

            client.add_query(request_id, handle).await;

            #(#rest)*
        }
    };

    server_fn.block = server_wrap;

    Ok(quote! {
        #server_fn
    })
}

pub fn generate_client_sql_watch(
    mcall: syn::ImplItemMethod,
    query_id: u64,
) -> Result<proc_macro2::TokenStream> {
    let mut client_fn = mcall.clone();
    let mut stmts = client_fn.block.stmts.clone();

    let stmt = stmts.pop().unwrap();

    let client_wrap: syn::Block = syn::parse_quote! {
        {
            let mut cache = crate::CACHE.write().await;
            match cache.query(#query_id) {
                Some(results) => return results,
                None => {
                    // FIXME: Beware of data races with next request id if another request is
                    // issued between
                    let request_id = crate::SOCKET.read().await.as_ref().map(|socket| socket.get_next_request_id()).unwrap();

                    cache.register_request(request_id, #query_id);

                    let results = #stmt;

                    cache.update(#query_id, results.clone());

                    comet::console_log!("cache after update {:#?}", *cache);

                    results
                },
            }
        }
    };

    client_fn.block = client_wrap;

    Ok(quote! {
        #client_fn
    })
}
