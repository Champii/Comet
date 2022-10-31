use proc_macro::TokenStream;

use proc_macro2::Span;
use quote::quote;
use syn::parse::Result;

pub fn perform(input: TokenStream) -> TokenStream {
    proc_macro::TokenStream::from(
        exprs_to_idents(input).unwrap_or_else(|e| syn::Error::to_compile_error(&e)),
    )
}

pub fn exprs_to_idents(_mcall: TokenStream) -> Result<proc_macro2::TokenStream> {
    use quote::ToTokens;

    let (models, _inner): (Vec<_>, Vec<_>) = crate::db_macro::MODELS
        .read()
        .unwrap()
        .iter()
        .map(|name| {
            (
                syn::Ident::new(name, Span::call_site()),
                syn::Ident::new(&format!("{}Proto", name).to_string(), Span::call_site()),
            )
        })
        .unzip();

    // FIXME: This is insane
    let models2 = models.clone();
    let models3 = models.clone();
    let models4 = models.clone();
    let models5 = models.clone();
    let models6 = models.clone();
    let models7 = models.clone();
    let _models8 = models.clone();
    let models9 = models.clone();
    let models10 = models.clone();

    let models_ids = 0..models.len() as u64;

    let tt = quote! {
        pub type ModelId = u64;

        #[derive(Serialize, Deserialize, Debug)]
        #[serde(crate = "comet::prelude::serde")] // must be below the derive attribute
        pub enum Model {
            #(#models(#models2)),*
        }

        #(
            impl From<#models3> for Model {
                fn from(m: #models4) -> Self {
                    Model::#models5(m)
                }
            }

            impl From<Model> for #models6 {
                fn from(m: Model) -> Self {
                    match m {
                        Model::#models7(m) => m,
                        _ => panic!("Invalid model type"),
                    }
                }
            }
        )*

        impl Model {
            pub fn id(&self) -> i32 {
                match self {
                    #(
                        Model::#models9(m) => m.id,
                    )*
                }
            }

            pub fn model_id(&self) -> ModelId {
                match self {
                    #(
                        Model::#models10(m) => #models_ids,
                    )*
                }
            }
        }

        #[derive(Serialize, Deserialize, Debug)]
        #[serde(crate = "comet::prelude::serde")] // must be below the derive attribute
        pub enum Proto {
            Event(u64, Vec<crate::Event<Model>>), // original request_id
            RPCQuery(RPCQuery),
            RPCResponse(RPCResponse),
        }

        impl Proto {

        }

        #[async_trait]
        impl comet::prelude::ProtoTrait for Proto {
            type Response = Proto;
            #[cfg(not(target_arch = "wasm32"))]
            type Client = Client;
            #[cfg(target_arch = "wasm32")]
            type Client = ();

            async fn dispatch(self, request_id: u64, client: Self::Client) -> Option<Self::Response> {
                match self {
                    Proto::Event(_request_id, _event) => {
                        // update cache
                        // redraw
                        None
                    },
                    Proto::RPCQuery(rpc_proto) => rpc_proto.dispatch(request_id, client).await,
                    Proto::RPCResponse(rpc_proto) => rpc_proto.dispatch(request_id, client).await,
                    // #(Proto::#models2(#inner2) => #inner3.dispatch(),)*
                    _ => todo!(),
                }
            }
        }
    };

    Ok(tt.into_token_stream())
}
