use crate::prelude::*;

impl<Comp, Msg> App<Comp, Msg>
where
    Comp: Component<Msg> + 'static,
    Msg: Clone + 'static,
{
    pub async fn run(&mut self) -> PercyDom {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let body = document.body().expect("document should have a body");

        let root = self.root.read().await.view(self.root.clone()).await;

        let vdom = PercyDom::new_append_to_mount(root, &body);

        return vdom;
    }

    pub async fn update(&mut self, vdom: &mut PercyDom) {
        let root = self.root.read().await.view(self.root.clone()).await;

        vdom.update(root);
    }
}
