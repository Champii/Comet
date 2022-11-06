use std::ops::Deref;
use std::sync::Arc;
use tokio::sync::RwLock;
use vdom::VElement;

use crate::prelude::Component;

#[derive(Default, Debug)]
pub struct Shared<T>(pub Arc<RwLock<Box<T>>>);

impl<T> Clone for Shared<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> From<T> for Shared<T> {
    fn from(t: T) -> Self {
        Self(Arc::new(RwLock::new(Box::new(t))))
    }
}

impl<T> Deref for Shared<T> {
    type Target = RwLock<Box<T>>;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl<T: Into<VElement> + Clone> From<Shared<T>> for VElement {
    fn from(shared: Shared<T>) -> Self {
        let comp = shared.blocking_read().as_ref();
        comp.view(shared).into()
    }
}

impl<Msg, T: Component<Msg> + Clone> From<Shared<T>> for VElement {
    fn from(shared: Shared<T>) -> Self {
        let comp = shared.blocking_read().as_ref();
        comp.view(shared)
    }
}
