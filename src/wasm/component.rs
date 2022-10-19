/* use crate::{element, prelude::*, renderable};
use std::{cell::RefCell, rc::Rc}; */

/* impl<Msg, Comp> Renderable<Msg, Comp> for Comp
where
    Comp: Component<Msg>,
    Msg: Clone + 'static,
{
    type Output = web_sys::Element;

    fn render<F>(&self, f: F) -> web_sys::Element
    where
        F: Fn(Msg) + Clone + 'static,
    {
        let view = self.borrow().view();
        let component = self.clone();

        let cb = move |msg| {
            component.borrow_mut().update(msg);

            run_rec(component.clone());
        };

        <element::Element<Msg> as renderable::Renderable<Msg, Comp>>::render::<_>(&view, cb)
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
} */
