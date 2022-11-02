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
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();

    mcall.items = res
        .iter()
        .flatten()
        .map(|item| syn::parse_quote! { #item })
        .collect();

    let lower_model_name = quote! { #self_type }.to_string().to_lowercase();
    let wrapper_mod_name = format!("__{}_rpcs_{}", lower_model_name, RPCS.read().unwrap().len());
    let wrapper_mod_name: syn::Ident = syn::parse_str(&wrapper_mod_name).unwrap();

    Ok(quote! {
        mod #wrapper_mod_name {
            use super::*;
            use crate::{Proto, RPCQuery, RPCResponse};

            #mcall
        }
    })
}

pub fn register_rpc(
    self_type: syn::Type,
    mcall: &syn::ImplItemMethod,
) -> Result<Vec<proc_macro2::TokenStream>> {
    let is_watch = mcall.attrs.iter().any(|attr| attr.path.is_ident("watch"));

    let mut server_fn = mcall.clone();
    let mut client_fn = mcall.clone();

    let query_types = mcall
        .sig
        .inputs
        .iter()
        .map(|arg| {
            let (is_mut, ty) = match arg {
                syn::FnArg::Receiver(fn_arg) => {
                    if fn_arg.mutability.is_some() {
                        (true, self_type.clone())
                    } else {
                        (false, self_type.clone())
                    }
                }
                syn::FnArg::Typed(c) => (false, *c.ty.clone()),
            };
            (is_mut, quote! { #ty }.to_string())
        })
        .collect::<Vec<_>>();

    let rpc_nb = RPCS.read().unwrap().len() as u64;

    let query_variant = format!("RPCQuery{}", rpc_nb);

    let response_variant = format!("RPCResponse{}", rpc_nb);

    let query_variant_real: syn::Ident = syn::parse_str(&query_variant).unwrap();
    let response_variant_real: syn::Ident = syn::parse_str(&response_variant).unwrap();

    let response_type_orig = match &mcall.sig.output {
        syn::ReturnType::Default => syn::parse_quote! { () },
        syn::ReturnType::Type(_, ty) => ty.clone(),
    };

    let response_type = quote! { #response_type_orig }.to_string();
    let response_type = if let Some((true, _)) = query_types.get(0) {
        (true, response_type)
    } else {
        (false, response_type)
    };

    let model_name = quote! { #self_type }.to_string();
    let fn_name_ident = mcall.sig.ident.clone();
    let fn_name = mcall.sig.ident.to_string();

    let watch_wrapper_fn_name = if is_watch {
        Some(syn::parse_str::<syn::Ident>(&format!("__{}_watch__", fn_name)).unwrap())
    } else {
        None
    };

    let method_name = if let Some(watch_wrapper_fn_name) = &watch_wrapper_fn_name {
        watch_wrapper_fn_name.to_string()
    } else {
        fn_name.clone()
    };

    let response_self: Vec<syn::Ident> = if let (true, _) = response_type.clone() {
        let returned_self: syn::Ident = syn::parse_quote! { returned_self };

        vec![returned_self]
    } else {
        vec![]
    };
    let response_self2 = response_self.clone();

    RPCS.write().unwrap().push(RpcEntry {
        model_name: model_name.clone(),
        method_name,
        query_variant: query_variant.clone(),
        query_types,
        response_variant,
        response_type,
    });

    let query_args = mcall
        .sig
        .inputs
        .iter()
        .map(|arg| match arg {
            syn::FnArg::Receiver(_) => quote! { self },
            syn::FnArg::Typed(c) => {
                let pat = &c.pat;

                quote! { #pat }
            }
        })
        .collect::<Vec<_>>();

    let query_args2 = query_args.clone();
    let query_args3 = query_args.clone();

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

    let mut generated_fns = vec![];

    if let Some(watch_wrapper_fn_name) = watch_wrapper_fn_name {
        let orig_fn_args = client_fn.sig.inputs.clone().into_iter().collect::<Vec<_>>();
        let mut wrapper_fn_args = client_fn.sig.inputs.clone().into_iter().collect::<Vec<_>>();
        let request_id_arg: syn::FnArg = syn::parse_quote! { request_id: u64 };
        wrapper_fn_args.push(request_id_arg);
        let client_arg: syn::FnArg = syn::parse_quote! { client: comet::server::client::Client };
        wrapper_fn_args.push(client_arg);

        let server_fn_stmts = server_fn.block.stmts.clone();

        let wrapper_fn = quote! {
            #[cfg(not(target_arch = "wasm32"))]
            #[watch]
            pub async fn #watch_wrapper_fn_name(#(#wrapper_fn_args,)*) -> #response_type_orig {
                #( #server_fn_stmts )*;
                #self_type::#fn_name_ident(#(#query_args2.clone()),*).await
            }
        };

        server_fn.attrs.retain(|attr| !attr.path.is_ident("watch"));

        generated_fns.push(wrapper_fn);

        let client_fn_stmts = client_fn.block.stmts.clone();

        let client_wrapper_fn = quote! {
            #[cfg(target_arch = "wasm32")]
            pub async fn #watch_wrapper_fn_name(#(#orig_fn_args,)*) -> #response_type_orig {
                #( #client_fn_stmts )*
            }
        };

        let new_client_fn_body: syn::Block = syn::parse_quote! {
            {
                #self_type::#watch_wrapper_fn_name(#(#query_args3.clone()),*).await
            }
        };

        client_fn.block = new_client_fn_body;

        generated_fns.push(client_wrapper_fn);
    }

    generated_fns.extend(vec![
        quote! {
            #[cfg(not(target_arch = "wasm32"))]
            #server_fn
        },
        quote! {
            #[cfg(target_arch = "wasm32")]
            #client_fn
        },
    ]);

    Ok(generated_fns)
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
                                        &format!("arg_{}", id),
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
        .zip(methods.clone())
        .zip(query_params.clone())
        .zip(query_types.clone())
        .map(|(((model, method), params), types)| {
            let mut params = params
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
                .collect::<Vec<syn::Expr>>();

            // If watch method, inject the request_id argument
            if method.to_string().matches("_watch__").count() > 0 {
                params.push(syn::parse_quote! { request_id });
                params.push(syn::parse_quote! { client });
            }

            params
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
            #[cfg(not(target_arch = "wasm32"))]
            type Client = comet::server::client::Client;
            #[cfg(target_arch = "wasm32")]
            type Client = ();

            #[cfg(not(target_arch = "wasm32"))]
            async fn dispatch(self, request_id: u64, client: Self::Client) -> Option<Self::Response>
            where Self::Client: Send,
            {
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
            #[cfg(not(target_arch = "wasm32"))]
            type Client = Client;
            #[cfg(target_arch = "wasm32")]
            type Client = ();

            #[cfg(target_arch = "wasm32")]
            async fn dispatch(self, request_id: u64, client: Self::Client) -> Option<Self::Response>
            where Self::Client: Send,
            {
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
