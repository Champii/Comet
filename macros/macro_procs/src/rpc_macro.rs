use lazy_static::lazy_static;
use proc_macro::TokenStream;
use std::sync::{Arc, RwLock};

use quote::quote;
use syn::{parse::Result, parse_macro_input, ImplItem};

#[derive(Debug)]
pub struct RpcEntry {
    model_name: String,
    method_name: String,
    query_variant: String,
    query_types: Vec<(bool, String)>, // (is_mut, type)
    response_variant: String,
    response_type: (bool, String), // (is mut self, ret)
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
    mcall: &syn::ImplItemMethod,
) -> Result<Vec<proc_macro2::TokenStream>> {
    let server_fn = mcall.clone();
    let mut client_fn = mcall.clone();

    let query_types = mcall
        .sig
        .decl
        .inputs
        .iter()
        .map(|arg| {
            let (is_mut, ty) = match arg {
                syn::FnArg::SelfRef(fn_arg) => {
                    if fn_arg.mutability.is_some() {
                        (true, self_type.clone())
                    } else {
                        (false, self_type.clone())
                    }
                }
                syn::FnArg::Captured(c) => (false, c.ty.clone()),
                _ => unimplemented!(),
            };
            (is_mut, quote! { #ty }.to_string())
        })
        .collect::<Vec<_>>();

    let rpc_nb = RPCS.read().unwrap().len() as u64;

    let query_variant = format!("RPCQuery{}", rpc_nb);

    let response_variant = format!("RPCResponse{}", rpc_nb);

    let query_variant_real: syn::Ident = syn::parse_str(&query_variant).unwrap();
    let response_variant_real: syn::Ident = syn::parse_str(&response_variant).unwrap();

    let response_type = match &mcall.sig.decl.output {
        syn::ReturnType::Default => syn::parse_quote! { () },
        syn::ReturnType::Type(_, ty) => ty.clone(),
    };

    let response_type = quote! { #response_type }.to_string();
    let response_type = if let Some((true, _)) = query_types.get(0) {
        (true, response_type)
    } else {
        (false, response_type)
    };

    let model_name = quote! { #self_type }.to_string();
    let fn_name = mcall.sig.ident.to_string();

    let response_self: Vec<syn::Ident> = if let (true, _) = response_type.clone() {
        vec![syn::parse_quote! { returned_self }]
    } else {
        vec![]
    };
    let response_self2 = response_self.clone();

    RPCS.write().unwrap().push(RpcEntry {
        model_name,
        method_name: fn_name,
        query_variant: query_variant.clone(),
        query_types,
        response_variant,
        response_type,
    });

    let query_args = mcall
        .sig
        .decl
        .inputs
        .iter()
        .map(|arg| match arg {
            syn::FnArg::SelfRef(_) => quote! { self },
            syn::FnArg::Captured(c) => {
                let pat = &c.pat;

                quote! { #pat }
            }
            _ => unimplemented!(),
        })
        .collect::<Vec<_>>();

    let client_wrap: syn::Block = syn::parse_quote! {
        {
            let response = if let Some(socket) = crate::SOCKET.write().await.as_mut() {
                socket.rpc(Proto::RPCQuery(RPCQuery::#query_variant_real(#(#query_args.clone()),*))).await
            } else {
                    panic!("No socket")
            };

            let response = match response {
                Proto::RPCResponse(RPCResponse::#response_variant_real(#(#response_self,)* response)) => { #(*self = #response_self2;)* response},
                _ => unimplemented!(),
            };

            response
        }
    };

    client_fn.block = client_wrap;

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
    // FIXME: WTF !!! This is why I should stop coding after the 20th consecutive hour haha
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
                                .map(|(is_mut, s)| {
                                    (*is_mut, syn::parse_str::<syn::Type>(&s).unwrap())
                                })
                                .collect::<Vec<_>>(),
                            rpc_entry
                                .query_types
                                .iter()
                                .enumerate()
                                .map(|(id, (is_mut, _s))| {
                                    let id = syn::Ident::new(
                                        &if *is_mut {
                                            format!("arg_{}", id)
                                        } else {
                                            format!("arg_{}", id)
                                        },
                                        proc_macro2::Span::call_site(),
                                    );
                                    (*is_mut, syn::parse_quote! { #id })
                                })
                                .collect::<Vec<(bool, syn::Ident)>>(),
                        ),
                    ),
                    (
                        syn::parse_str::<syn::Variant>(&rpc_entry.response_variant).unwrap(),
                        (
                            rpc_entry.response_type.0,
                            syn::parse_str::<syn::Type>(&rpc_entry.response_type.1).unwrap(),
                        ),
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

    let query_variants2 = query_variants.clone();
    let response_variants2 = response_variants.clone();
    let response_variants3 = response_variants.clone();

    let query_params_with_ref = models
        .iter()
        .zip(query_params.clone())
        .zip(query_types.clone())
        .map(|((model, params), types)| {
            params
                .iter()
                .zip(types)
                .enumerate()
                .map(|(id, ((_, param), (is_mut, ty)))| {
                    if id == 0 && ty == model.clone() {
                        if is_mut {
                            syn::parse_quote! { &mut #param }
                        } else {
                            syn::parse_quote! { & #param }
                        }
                    } else {
                        syn::parse_quote! { #param }
                    }
                })
                .collect::<Vec<syn::Expr>>()
        })
        .collect::<Vec<_>>();

    // fix query_params that are mut
    let query_params = query_params
        .iter()
        .map(|params| {
            params
                .iter()
                .enumerate()
                .map(|(id, (is_mut, param))| {
                    if id == 0 && *is_mut {
                        syn::parse_quote! { mut #param }
                    } else {
                        syn::parse_quote! { #param }
                    }
                })
                .collect::<Vec<syn::Pat>>()
        })
        .collect::<Vec<_>>();

    let response_types = response_types
        .into_iter()
        .zip(models.clone())
        .map(|((is_self_mut, ty), model)| {
            if is_self_mut {
                vec![syn::parse_quote! { #model }, ty]
            } else {
                vec![ty]
            }
        })
        .collect::<Vec<Vec<syn::Type>>>();

    let query_types = query_types
        .iter()
        .map(|vecs| vecs.iter().map(|(_, ty)| ty.clone()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let response_self = response_types
        .iter()
        .map(|types| {
            if types.len() == 2 {
                vec![syn::parse_quote! { arg_0 }]
            } else {
                vec![]
            }
        })
        .collect::<Vec<Vec<syn::Ident>>>();
    let response_self2 = response_self.clone();

    let proto = quote! {
        #[derive(Serialize, Deserialize, Debug, Clone)]
        #[serde(crate = "comet::prelude::serde")] // must be below the derive attribute
        pub enum RPCQuery {
            #(#query_variants(#(#query_types),*)),*
        }
        #[derive(Serialize, Deserialize, Debug, Clone)]
        #[serde(crate = "comet::prelude::serde")] // must be below the derive attribute
        pub enum RPCResponse {
            #(#response_variants(#(#response_types),*)),*
        }

        impl RPCQuery {
        }

        #[async_trait]
        impl comet::prelude::ProtoTrait for RPCQuery {
            type Response = Proto;

            async fn dispatch(self) -> Option<Self::Response> {
                match self {
                    #(RPCQuery::#query_variants2(#(#query_params),*) => {
                        let res = #models::#methods(#(#query_params_with_ref),*).await;
                        Some(Proto::RPCResponse(RPCResponse::#response_variants2(#(#response_self,)* res)))
                    }),*
                    _ => todo!(),
                }
            }
        }

        impl RPCResponse {
        }

        #[async_trait]
        impl comet::prelude::ProtoTrait for RPCResponse {
            type Response = Proto;

            async fn dispatch(self) -> Option<Self::Response> {
                match self {
                    #(RPCResponse::#response_variants3(#(#response_self2,)* arg) => {
                        None
                    }),*
                    _ => todo!(),
                }
            }
        }

    };

    proto.into()
}
