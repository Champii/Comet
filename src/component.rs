use std::{cell::RefCell, rc::Rc};

use crate::{
    element::{self, Element},
    renderable,
};

pub trait Component<Msg>: 'static
where
    Msg: Clone + 'static,
{
    fn update(&mut self, msg: Msg);
    fn view(&self) -> Element<Msg>;

    #[cfg(target_arch = "wasm32")]
    fn render(comp: Rc<RefCell<Self>>, parent: &web_sys::Element)
    where
        Self: Sized,
    {
        run_rec(comp, parent);
    }
}

#[cfg(target_arch = "wasm32")]
fn run_rec<Msg, Comp>(component: Rc<RefCell<Comp>>, parent: &web_sys::Element)
where
    Comp: Component<Msg>,
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

    let elem = <element::Element<Msg> as renderable::Renderable<Msg>>::render::<_>(&view, cb);

    parent.set_inner_html("");
    parent.append_child(&elem).unwrap();
}
