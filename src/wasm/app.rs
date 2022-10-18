use crate::{element, prelude::*, renderable};
use std::{cell::RefCell, rc::Rc};

impl<Comp, Msg> App<Comp, Msg>
where
    Comp: Component<Msg> + 'static,
    Msg: Clone + 'static,
{
    pub fn run(&mut self) {
        run_rec(self.root.clone());
    }
}

fn run_rec<Msg, Comp>(component: Rc<RefCell<Comp>>)
where
    Comp: Component<Msg>,
    Msg: Clone + 'static,
{
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let view = component.borrow().view();
    let component = component.clone();

    let cb = move |msg| {
        component.borrow_mut().update(msg);

        run_rec(component.clone());
    };

    let elem = <element::Element<Msg> as renderable::Renderable<Msg, Comp>>::render::<_>(&view, cb);

    body.set_inner_html("");
    body.append_child(&elem).unwrap();
}
