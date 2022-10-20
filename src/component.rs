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
    fn view(&self) -> Element<Msg>;

    #[cfg(target_arch = "wasm32")]
    fn into_shared(self) -> Rc<RefCell<Box<Self>>>
    where
        Self: Sized,
    {
        Rc::new(RefCell::new(Box::new(self)))
    }

    #[cfg(target_arch = "wasm32")]
    fn into_shared_dyn(self) -> Rc<RefCell<Box<dyn Component<Msg>>>>
    where
        Self: Sized,
    {
        Rc::new(RefCell::new(Box::new(self)))
    }

    /* #[cfg(target_arch = "wasm32")]
    fn render(comp: Rc<RefCell<Box<Self>>>, parent: &web_sys::Element)
    /* where
    Self: Sized, */
    {
        run_rec(comp, parent);
    } */
}

#[cfg(target_arch = "wasm32")]
pub fn run_rec<Msg, Comp>(component: Rc<RefCell<Box<Comp>>>, parent: &web_sys::Element)
where
    Comp: Component<Msg> + ?Sized,
    Msg: Clone + 'static,
{
    let view = component.borrow().view();
    let component = component.clone();

    let parent2 = parent.clone();
    let cb = move |msg| {
        component.borrow_mut().update(msg);
        let parent3 = parent2.clone();

        run_rec(component.clone(), &parent3);
    };

    let elem = <Element<Msg> as Renderable<Msg>>::render::<_>(&view, cb);

    parent.set_inner_html("");
    parent.append_child(&elem).unwrap();
}
