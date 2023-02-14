use async_trait::async_trait;
use percy_dom::prelude::*;
use wasm_bindgen_futures::spawn_local;

pub type Html = VirtualNode;

use crate::prelude::Shared;

#[async_trait(?Send)]
pub trait Component<Msg>: Send + Sync + 'static
where
    Msg: Clone + 'static,
    Self: Sized,
{
    async fn update(&mut self, msg: Msg);
    async fn update_bindings(&mut self, bindings: Shared<Vec<String>>);
    async fn view(&self, shared_self: Shared<Self>) -> Html;

    fn callback() -> Box<dyn Fn(Shared<Self>) -> Box<dyn Fn(Msg)>> {
        Box::new(move |shared| {
            Box::new(move |msg| {
                let shared = shared.clone();

                spawn_local(async move {
                    shared.write().await.update(msg).await;
                });
            })
        })
    }
}

pub trait ToVElement {
    fn to_velement(self) -> VirtualNode;
}
