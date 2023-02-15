use async_trait::async_trait;
use percy_dom::prelude::*;

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
}

#[async_trait(?Send)]
pub trait ToVirtualNode {
    async fn to_virtual_node(self) -> VirtualNode;
}
