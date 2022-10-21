use std::ops::Deref;
use std::{cell::RefCell, rc::Rc};

#[derive(Default)]
pub struct Shared<T>(pub Rc<RefCell<Box<T>>>);

impl<T> Clone for Shared<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> From<T> for Shared<T> {
    fn from(t: T) -> Self {
        Self(Rc::new(RefCell::new(Box::new(t))))
    }
}

impl<T> Deref for Shared<T> {
    type Target = RefCell<Box<T>>;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}
