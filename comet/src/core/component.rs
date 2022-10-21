#[cfg(target_arch = "wasm32")]
use std::{cell::RefCell, rc::Rc};

use crate::prelude::*;

#[cfg(target_arch = "wasm32")]
pub type Shared<T> = Rc<RefCell<Box<T>>>;

pub trait Component<Msg>: 'static
where
    Msg: Clone + 'static,
{
    fn update(&mut self, msg: Msg);
    fn view<F>(&self, f: F) -> web_sys::Element
    where
        F: Fn(Msg) + Clone + 'static;

    #[cfg(target_arch = "wasm32")]
    fn into_shared(self) -> Rc<RefCell<Box<Self>>>
    where
        Self: Sized,
    {
        Rc::new(RefCell::new(Box::new(self)))
    }
}

#[cfg(target_arch = "wasm32")]
pub fn run_rec<Msg, Comp>(component: Rc<RefCell<Box<Comp>>>, parent: &web_sys::Element)
where
    Comp: Component<Msg> + ?Sized,
    Msg: Clone + 'static,
{
    let component2 = component.clone();

    let parent2 = parent.clone();
    let cb = move |msg| {
        component2.borrow_mut().update(msg);
        let parent3 = parent2.clone();

        run_rec(component2.clone(), &parent3);
    };

    let view = component.borrow().view(cb);

    parent.set_inner_html("");
    parent.append_child(&view).unwrap();
}
