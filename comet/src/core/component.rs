pub trait Component<Msg>: 'static
where
    Msg: Clone + 'static,
{
    fn update(&mut self, msg: Msg);
    fn view<F>(&self, f: F, bindings: Shared<Vec<web_sys::Element>>) -> web_sys::Element
    where
        F: Fn(Option<Msg>) + Clone + 'static;
    fn update_bindings(&mut self, bindings: Shared<Vec<web_sys::Element>>);
}

use crate::prelude::*;

pub fn run_rec<Msg, Comp>(component: Shared<Comp>, parent: &web_sys::Element)
where
    Comp: Component<Msg>,
    Msg: Clone + 'static,
{
    let component2 = component.clone();

    let parent2 = parent.clone();

    let bindings: Shared<Vec<web_sys::Element>> = vec![].into();
    let bindings_clone = bindings.clone();

    let cb = move |msg| {
        component2
            .borrow_mut()
            .update_bindings(bindings_clone.clone());
        if let Some(msg) = msg {
            component2.borrow_mut().update(msg);
        }

        let parent3 = parent2.clone();

        run_rec(component2.clone(), &parent3);
    };

    let view = component.borrow().view(cb, bindings.clone());

    parent.set_inner_html("");
    parent.append_child(&view).unwrap();
}
