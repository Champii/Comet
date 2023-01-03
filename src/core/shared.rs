use std::ops::Deref;
use std::sync::Arc;
use tokio::sync::RwLock;
use vdom::VElement;

use crate::prelude::{Component, ToVElement};

#[derive(Default, Debug)]
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

impl<T: ToVElement + std::fmt::Debug> From<Shared<T>> for VElement {
    fn from(shared: Shared<T>) -> VElement {
        shared.0.to_velement()
    }
}

/* impl<T, Msg> From<Shared<T>> for VElement
where
    T: Component<Msg> + 'static,
    Msg: Clone + 'static,
{
    fn from(shared: Shared<T>) -> VElement {
        futures::executor::block_on(async { shared.read().await.view(shared.clone()).await })
    }
} */

impl<T> Deref for Shared<T> {
    type Target = RwLock<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
