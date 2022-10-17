use crate::prelude::{App, Component, Renderable};

impl<Comp, Msg> App<Comp, Msg>
where
    Comp: Component<Msg>,
{
    pub fn run(&mut self) {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let body = document.body().expect("document should have a body");

        let elem = self.root.view().render();

        body.append_child(&elem).unwrap();
    }
}
