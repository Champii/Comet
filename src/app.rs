use crate::prelude::*;
use std::fmt::Debug;

// #[derive(Debug)]
pub struct App<Compo, Msg>
where
    Compo: Component<Msg>,
    Msg: Debug,
{
    pub root: Compo,
    phantom: std::marker::PhantomData<Msg>,
}

impl<Compo, Msg> App<Compo, Msg>
where
    Compo: Component<Msg>,
    Msg: Debug,
{
    pub fn new(root: Compo) -> Self {
        Self {
            root,
            phantom: std::marker::PhantomData,
        }
    }

    pub fn run(&mut self) {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let body = document.body().expect("document should have a body");

        let elem = self.root.view().render();

        body.append_child(&elem).unwrap();
    }
}
