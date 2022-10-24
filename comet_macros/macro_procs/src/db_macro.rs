use lazy_static::lazy_static;
use proc_macro::TokenStream;
use proc_macro2::Span;
use std::sync::{Arc, RwLock};

use quote::quote;
use syn::{parse::Result, parse_macro_input};

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
        #[derive(Serialize, Deserialize, Debug)]
        #[serde(crate = "comet::prelude::serde")] // must be below the derive attribute
        #mcall

        #[derive(Serialize, Deserialize, Debug)]
        #[serde(crate = "comet::prelude::serde")] // must be below the derive attribute
        pub struct #name_id(i32);

        use std::ops::Deref;

        impl Deref for #name_id {
            type Target = i32;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        #[derive(Serialize, Deserialize, Debug)]
        #[serde(crate = "comet::prelude::serde")] // must be below the derive attribute
        pub enum #proto_name {
            New(#name),
            Fetch(#name_id),
            List,
            Update(#name),
            Delete(#name_id),
        }

        impl #proto_name {
            pub fn dispatch(&self) {
                match self {
                    #proto_name::New(model) => #name::new(model),
                    #proto_name::Fetch(id) => #name::fetch(id),
                    #proto_name::List => #name::list(),
                    #proto_name::Update(model) => model.update(),
                    #proto_name::Delete(id) => #name::delete(id),
                }
            }
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
            pub fn new(model: &Self) {
                println!("new {:#?}", model);
            }
            pub fn fetch(id: &#name_id) {
                println!("fetch");
            }
            pub fn list() {
                println!("list");
            }
            pub fn update(&self) {
                println!("update");
            }
            pub fn delete(id: &#name_id) {
                println!("delete");
            }
        }
    };

    Ok(tt.into_token_stream())
}
