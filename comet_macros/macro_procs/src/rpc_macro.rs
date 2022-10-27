use lazy_static::lazy_static;
use proc_macro::TokenStream;
use std::sync::{Arc, RwLock};

use quote::quote;
use syn::{parse::Result, parse_macro_input, ImplItem};

pub struct RpcEntry {
    id: u64,
    model_name: String,
    method_name: String,
    query_variant: String,
    query_types: Vec<String>,
    response_variant: String,
    response_type: String,
}

lazy_static! {
    // TODO: replace with atomics
    pub static ref RPCS: Arc<RwLock<Vec<RpcEntry>>> = Arc::new(RwLock::new(vec![])); // QueryId, ModelName, FnName, InputEnum, OutputEnum
}

pub fn perform(input: TokenStream) -> TokenStream {
    let mcall = parse_macro_input!(input as syn::ItemImpl);

    proc_macro::TokenStream::from(
        register_rpcs(mcall).unwrap_or_else(|e| syn::Error::to_compile_error(&e)),
    )
}

pub fn register_rpcs(mut mcall: syn::ItemImpl) -> Result<proc_macro2::TokenStream> {
    let self_type = *mcall.self_ty.clone();
    let res = mcall
        .items
        .iter()
        .map(|item| match item {
            ImplItem::Method(method) => register_rpc(self_type.clone(), method).unwrap(),
            _ => unimplemented!(),
        })
        .collect::<Vec<_>>();

    mcall.items = res
        .iter()
        .flatten()
        .map(|item| syn::parse_quote! { #item })
        .collect();

    Ok(quote! {
        #mcall
    })
}

pub fn register_rpc(
    self_type: syn::Type,
    mut mcall: &syn::ImplItemMethod,
) -> Result<Vec<proc_macro2::TokenStream>> {
    let mut server_fn = mcall.clone();
    let mut client_fn = mcall.clone();

    let query_types = mcall
        .sig
        .decl
        .inputs
        .iter()
        .map(|arg| {
            let ty = match arg {
                syn::FnArg::SelfRef(_) => self_type.clone(),
                syn::FnArg::Captured(c) => c.ty.clone(),
                _ => unimplemented!(),
            };
            quote! { #ty }.to_string()
        })
        .collect::<Vec<_>>();

    let rpc_nb = RPCS.read().unwrap().len() as u64;

    let query_variant = format!("RPCQuery{}", rpc_nb);

    let response_variant = format!("RPCResponse{}", rpc_nb);

    let responst_type = match &mcall.sig.decl.output {
        syn::ReturnType::Default => syn::parse_quote! { () },
        syn::ReturnType::Type(_, ty) => ty.clone(),
    };

    /* let enum_query_str = quote! { #enum_query_variant }.to_string();
    let enum_response_str = quote! { #enum_response_variant }.to_string(); */

    let model_name = quote! { #self_type }.to_string();
    let fn_name = mcall.sig.ident.to_string();

    let response_type = quote! { #responst_type }.to_string();

    RPCS.write().unwrap().push(RpcEntry {
        id: rpc_nb,
        model_name,
        method_name: fn_name,
        query_variant,
        query_types,
        response_variant,
        response_type,
    });

    // server_fn.block = server_wrap;

    /* let client_wrap: syn::Block = syn::parse_quote! {
        {
            // crate::SOCKET.rpc(RPCProto::enum_response_variant);
        }
    };

    client_fn.block = client_wrap; */

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

pub fn generate_rpc_proto(_input: TokenStream) -> TokenStream {
    let (to_call, enum_stuff): (Vec<_>, Vec<_>) = RPCS
        .read()
        .unwrap()
        .iter()
        .map(|rpc_entry| {
            (
                (
                    syn::parse_str::<syn::Type>(&rpc_entry.model_name).unwrap(),
                    syn::parse_str::<syn::Ident>(&rpc_entry.method_name).unwrap(),
                ),
                (
                    (
                        syn::parse_str::<syn::Variant>(&rpc_entry.query_variant).unwrap(),
                        (
                            rpc_entry
                                .query_types
                                .iter()
                                .map(|s| syn::parse_str::<syn::Type>(&s).unwrap())
                                .collect::<Vec<_>>(),
                            rpc_entry
                                .query_types
                                .iter()
                                .enumerate()
                                .map(|(id, _s)| {
                                    let id = syn::Ident::new(
                                        &format!("arg{}", id),
                                        proc_macro2::Span::call_site(),
                                    );
                                    syn::parse_quote! { #id }
                                })
                                .collect::<Vec<syn::Ident>>(),
                        ),
                    ),
                    (
                        syn::parse_str::<syn::Variant>(&rpc_entry.response_variant).unwrap(),
                        syn::parse_str::<syn::Variant>(&rpc_entry.response_type).unwrap(),
                    ),
                ),
            )
        })
        .unzip();

    let (models, methods): (Vec<_>, Vec<_>) = to_call.into_iter().unzip();
    let (query, response): (Vec<_>, Vec<_>) = enum_stuff.into_iter().unzip();

    let (query_variants, params): (Vec<_>, Vec<_>) = query.into_iter().unzip();
    let (query_types, query_params): (Vec<_>, Vec<_>) = params.into_iter().unzip();

    let (response_variants, response_types): (Vec<_>, Vec<_>) = response.into_iter().unzip();

    // let query_declared_params2 = query_declared_params.clone();
    let query_params2 = query_params.clone();
    let query_variants2 = query_variants.clone();
    let response_variants2 = response_variants.clone();

    let query_params_with_ref = models
        .iter()
        .zip(query_params.clone())
        .zip(query_types.clone())
        .map(|((model, params), types)| {
            params
                .iter()
                .zip(types)
                .map(|(param, ty)| {
                    if ty == model.clone() {
                        syn::parse_quote! { &#param }
                    } else {
                        syn::parse_quote! { #param }
                    }
                })
                .collect::<Vec<syn::Expr>>()
        })
        .collect::<Vec<_>>();

    println!("query_variants: {:?}", query_variants);

    let proto = quote! {
        #[derive(Serialize, Deserialize, Debug, Clone)]
        #[serde(crate = "comet::prelude::serde")] // must be below the derive attribute
        pub enum RPCQuery {
            #(#query_variants(#(#query_types),*)),*,
        }
        #[derive(Serialize, Deserialize, Debug, Clone)]
        #[serde(crate = "comet::prelude::serde")] // must be below the derive attribute
        pub enum RPCResponse {
            #(#response_variants(#response_types)),*
        }

        impl comet::prelude::Proto for RPCQuery {
            fn dispatch(&self) {
                match self.clone() {
                    #(RPCQuery::#query_variants2(#(#query_params),*) => {
                        // TODO: How to get the result back ?
                        #models::#methods(#(#query_params_with_ref),*);
                    }),*,
                    _ => todo!(),
                }
            }

            fn from_bytes(bytes: &[u8]) -> Self {
                serde_cbor::from_slice(bytes).unwrap()
            }

            fn to_bytes(&self) -> Vec<u8> {
                serde_cbor::to_vec(self).unwrap()
            }
        }

        impl comet::prelude::Proto for RPCResponse {
            fn dispatch(&self) {
                match self.clone() {
                    #(RPCResponse::#response_variants2(arg) => {
                        // TODO: Who to contact back with this result ?
                        // #models::#methods(#(#query_params2),*);
                    }),*,
                    _ => todo!(),
                }
            }

            fn from_bytes(bytes: &[u8]) -> Self {
                serde_cbor::from_slice(bytes).unwrap()
            }

            fn to_bytes(&self) -> Vec<u8> {
                serde_cbor::to_vec(self).unwrap()
            }
        }

    };

    proto.into()
}
