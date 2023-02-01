use std::ops::Deref;
use std::sync::Arc;
use tokio::sync::RwLock;
use vdom::VElement;

use crate::prelude::{Component, ToVElement};

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
