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

    let (models, inner): (Vec<_>, Vec<_>) = crate::db_macro::MODELS
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

    let inner2 = inner.clone();
    let inner3 = inner.clone();

    let models2 = models.clone();

    let tt = quote! {
        #[derive(Serialize, Deserialize, Debug)]
        #[serde(crate = "comet::prelude::serde")] // must be below the derive attribute
        pub enum Proto {
            RPCQuery(RPCQuery),
            RPCResponse(RPCResponse),
            // #(#models(#inner)),*
        }

        impl comet::prelude::Proto for Proto {
            fn dispatch(&self) {
                match self {
                    Proto::RPCQuery(rpc_proto) => rpc_proto.dispatch(),
                    Proto::RPCResponse(rpc_proto) => rpc_proto.dispatch(),
                    // #(Proto::#models2(#inner2) => #inner3.dispatch(),)*
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

    Ok(tt.into_token_stream())
}
