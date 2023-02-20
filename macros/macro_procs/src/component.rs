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

    pub fn collect_bindings(&self) -> Vec<Expr> {
        self.html.collect_bindings()
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
    let binds_map = component.collect_bindings();

    let variants = msg_variants(&events);

    let msg_enum = generate_msg_enum(&variants);
    let update_match = generate_update_match(&events, &variants);
    let update_bindings = generate_update_bindings(&binds_map);

    Ok(quote! {
        mod #mod_name {
            use super::*;
            use std::any::Any;

            #msg_enum

            #[async_trait(?Send)]
            impl Component<Msg> for #name {
                async fn update(&mut self, msg: Msg) {
                    #update_match
                }

                async fn update_bindings(&mut self, bindings: Shared<Vec<String>>) {
                    #update_bindings
                }

                async fn view(&self, shared_self: Shared<Self>) -> Html {
                    let mut bindings = Shared::from(vec![]);
                    let bindings2 = bindings.clone();

                    let callback = Box::new(move |msg_opt| {
                        let shared = shared_self.clone();
                        let bindings = bindings2.clone();

                        spawn_local(async move {
                            shared.write().await.update_bindings(bindings).await;
                            if let Some(msg) = msg_opt {
                                shared.write().await.update(msg).await;
                            }

                            #[cfg(target_arch = "wasm32")]
                            crate::redraw_root().await;
                        });
                    });

                    let mut events: Vec<Msg> = vec![#(Msg::#variants),*];

                    let mut root = #html;

                    let mut root = root.pop().unwrap();

                    match root {
                        VirtualNode::Element(ref mut tag) => {
                            tag.attrs.insert("__component".into(), "".into());
                        },
                        _ => {}
                    }

                    root
                }
            }

            use comet::prelude::VirtualNode;

            #[async_trait(?Send)]
            impl ToVirtualNode for #name {
                async fn to_virtual_node(self) -> VirtualNode {
                    // FIXME
                    Wrapper(Shared::from(self)).to_virtual_node().await
                }
            }

            #[async_trait(?Send)]
            impl ToVirtualNode for Wrapper<#name> {
                async fn to_virtual_node(self) -> VirtualNode {
                    // FIXME
                    Wrapper(Shared::from(self.0)).to_virtual_node().await
                }
            }

            #[async_trait(?Send)]
            impl ToVirtualNode for Wrapper<Shared<#name>> {
                async fn to_virtual_node(self) -> VirtualNode {
                    let shared = self.0.clone();
                    let shared2 = shared.clone();
                    let cmp = shared.0.read().await;
                    cmp.view(shared2.clone()).await
                }
            }
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

fn generate_update_bindings(binds: &Vec<Expr>) -> proc_macro2::TokenStream {
    quote! {
        use comet::prelude::percy_dom::JsCast;
        let document = web_sys::window().unwrap().document().unwrap();
        let mut i = 0;
        #(
            let name = bindings.read().await.get(i).unwrap().clone();
            let elements = document.get_elements_by_class_name(name.as_str());

            let element = elements.item(0 as u32).unwrap();

            i += 1;

            if let Ok(input_elem) = element.clone().dyn_into::<web_sys::HtmlInputElement>() {
                #binds = input_elem.value().parse().unwrap_or_default();
            } else if let Ok(input_elem) = element.dyn_into::<web_sys::HtmlSelectElement>() {
                #binds = input_elem.value().parse().unwrap_or_default();
            }
        )*
    }
}
