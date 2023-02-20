use crate::prelude::ToVirtualNode;
use crate::prelude::VirtualNode;
use async_trait::async_trait;
use std::ops::Deref;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Default)]
pub struct Shared<T>(pub Arc<RwLock<T>>);

impl<T> Clone for Shared<T> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl<T> From<T> for Shared<T> {
    fn from(t: T) -> Self {
        Self(Arc::new(RwLock::new(t)))
    }
}

impl<T> Deref for Shared<T> {
    type Target = RwLock<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[async_trait(?Send)]
impl<T> ToVirtualNode for Shared<T>
where
    T: ToVirtualNode + Clone,
{
    async fn to_virtual_node(self) -> VirtualNode {
        let new_self = self.0.read().await.clone();
        new_self.to_virtual_node().await
    }
}

/* #[async_trait(?Send)]
impl<T> ToVirtualNode for Shared<T>
where
    T: ToVirtualNode + Clone,
{
    async fn to_virtual_node(self) -> VirtualNode {
        let new_self = self.0.read().await.clone();
        new_self.to_virtual_node().await
    }
} */
