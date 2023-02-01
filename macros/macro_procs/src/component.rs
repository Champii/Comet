use derive_syn_parse::Parse;
use proc_macro::{Span, TokenStream};

use quote::quote;
use syn::{parse::Result, parse_macro_input, Expr};

use crate::html::Element;

pub fn perform(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Component);

    proc_macro::TokenStream::from(
        component(input).unwrap_or_else(|e| syn::Error::to_compile_error(&e)),
    )
}

#[derive(Parse)]
struct Component {
    name: syn::Type,

    #[allow(dead_code)]
    #[brace]
    open_brace: syn::token::Brace,

    #[inside(open_brace)]
    html: Element,
}

impl Component {
    pub fn collect_events(&self) -> Vec<Expr> {
        self.html.collect_events()
    }
}

fn component(component: Component) -> Result<proc_macro2::TokenStream> {
    let name = &component.name;
    let html = &component.html;

    let mod_name = syn::Ident::new(
        &format!("__component_{}", quote! {#name}.to_string().to_lowercase()),
        Span::call_site().into(),
    );

    let events = component.collect_events();
    let variants = msg_variants(&events);

    let msg_enum = generate_msg_enum(&variants);
    let update_match = generate_update_match(&events, &variants);

    Ok(quote! {
        mod #mod_name {
            use super::*;
            use std::any::Any;

            #msg_enum

            #[async_trait]
            impl Component<Msg> for #name {
                async fn update(&mut self, msg: Msg) {
                    #update_match
                }

                async fn view(&self, shared_self: Shared<Self>) -> VElement {
                    // let callback = Self::callback()(shared_self);
                    let callback = Box::new(move |msg| {
                        let shared = shared_self.clone();

                        #[cfg(target_arch = "wasm32")]
                        comet::console_log!("Callback");

                        spawn_local(async move {
                            shared.write().await.update(msg).await;

                            /* #[cfg(target_arch = "wasm32")]
                            comet::console_log!("shared {:#?}", shared.read().await); */

                            // let vdom = shared.read().await.view(shared.clone()).await;


                            #[cfg(target_arch = "wasm32")]
                            crate::redraw_root().await;
                        });
                    });

                    let mut html = #html;

                    match html {
                        VElement::Tag(ref mut tag) => {
                            tag.push_attr(VAttribute::new("__component".into(), VAttributeValue::String("".into())));
                        },
                        _ => {}
                    }

                    let events: Vec<Msg> = vec![#(Msg::#variants),*];

                    html.fix_events(&mut 0, &events, callback);

                    html
                }
            }

            use comet::prelude::vdom::VElement;

            impl Into<VElement> for #name {
                fn into(self) -> VElement {
                    // unimplemented!()
                    // panic!("LOL");
                    Wrapper(Shared::from(self)).into()
                }
            }

            /* impl ToVElement for #name {
                fn to_velement(self) -> VElement {
                    unimplemented!()
                }
            } */

            /* impl From<Arc<tokio::sync::RwLock<#name>>> for VElement {
                fn from(shared: Arc<RwLock<#name>>) -> VElement { */

            impl From<crate::Wrapper<Shared<#name>>> for VElement {
                fn from(shared: crate::Wrapper<Shared<#name>>) -> VElement {

            /* impl ToVElement for Arc<tokio::sync::RwLock<#name>> {
                fn to_velement(self) -> VElement { */
                    #[cfg(target_arch = "wasm32")]
                    comet::console_log!("Into VElement");

                    /* // FIXME: the problem is here
                    let shared = Shared::from(self); */
                    let shared = shared.0;

                    comet::prelude::futures::executor::block_on(async {
                        shared.0.read().await.view(shared.clone()).await
                    })
                }
            }

            /* impl<T: ToVElement + std::fmt::Debug> From<Arc<RwLock<T>>> for VElement {
                fn from(shared: Arc<RwLock<T>>) -> VElement {
                    shared.to_velement()
                }
            } */


            /* impl<T: ToVElement + std::fmt::Debug> From<Shared<T>> for VElement {
                fn from(shared: Shared<T>) -> VElement {
                    shared.0.to_velement()
                }
            } */
            /* impl Into<VElement> for Arc<tokio::sync::RwLock<#name>> {
                fn into(self) -> VElement {
                    #[cfg(target_arch = "wasm32")]
                    comet::console_log!("Into VElement");

                    /* // FIXME: the problem is here
                    let shared = Shared::from(self); */
                    let shared = self;

                    comet::prelude::futures::executor::block_on(async {
                        shared.read().await.view(shared.clone()).await
                    })
                }
            } */
        }
    })
}

fn msg_variants(events: &Vec<Expr>) -> Vec<syn::Expr> {
    events
        .iter()
        .enumerate()
        .map(|(i, _)| {
            let ident = syn::Ident::new(&format!("Event{}", i), Span::call_site().into());
            syn::parse_quote! { #ident }
        })
        .collect()
}

fn generate_msg_enum(variants: &Vec<Expr>) -> proc_macro2::TokenStream {
    quote! {
        #[derive(Clone, Debug)]
        pub enum Msg {
            #(#variants),*
        }
    }
}

fn generate_update_match(events: &Vec<Expr>, variants: &Vec<Expr>) -> proc_macro2::TokenStream {
    quote! {
        match msg {
            #(Msg::#variants => { #events; }),*
        };
    }
}
