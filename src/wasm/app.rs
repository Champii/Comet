use crate::prelude::*;

impl<Comp, Msg> App<Comp, Msg>
where
    Comp: Component<Msg> + 'static,
    Msg: Clone + 'static,
{
    pub async fn run(&mut self) {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let body = document.body().expect("document should have a body");

        crate::core::component::run_rec(self.root.clone(), &body.into()).await;
    }
}
