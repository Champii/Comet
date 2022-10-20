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
/* 
    #[cfg(target_arch = "wasm32")]
    fn into_shared_dyn(self) -> Rc<RefCell<Box<dyn Component<Msg>>>>
    where
        Self: Sized,
    {
        Rc::new(RefCell::new(Box::new(self)))
    } */

    #[cfg(target_arch = "wasm32")]
    fn render(comp: Rc<RefCell<Box<Self>>>, parent: &web_sys::Element)
    where
    Self: Sized,
    {
        run_rec(comp, parent);
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
    // let elem = <Element<Msg> as Renderable<Msg>>::render::<_>(&view, cb);

    parent.set_inner_html("");
    parent.append_child(&view).unwrap();
}

/* #[cfg(target_arch = "wasm32")]
pub fn run_rec2<Msg, Comp>(component: &mut Comp, parent: &web_sys::Element)
where
    Comp: Component<Msg> + Clone,
    Msg: Clone + 'static,
{
    let parent2 = parent.clone();
    let cb = move |msg| {
        component.update(msg);
        let parent3 = parent2.clone();

        run_rec2(component, &parent3);
    };

    let view = component.view(cb);
    // let elem = <Element<Msg> as Renderable<Msg>>::render::<_>(&view, cb);

    parent.set_inner_html("");
    parent.append_child(&view).unwrap();
} */
