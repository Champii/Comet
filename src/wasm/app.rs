use crate::prelude::*;

impl<Comp, Msg> App<Comp, Msg>
where
    Comp: Component<Msg> + 'static,
    Msg: Clone + 'static,
{
    pub async fn run(&mut self) {
        Self::run_component(self.root.clone()).await;
    }

    async fn run_component(comp: Shared<Comp>) {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let body = document.body().expect("document should have a body");

        let comp2 = comp.clone();

        let cb = move |msg| {
            let comp = comp2.clone();

            spawn_local(async move {
                comp.write().await.update(msg).await;

                spawn_local(Self::run_component(comp));
            })
        };

        let view = comp.read().await.view();

        let dom = view.render(Box::new(cb));

        // TODO: Diff + Patch
        body.set_inner_html("");
        body.append_child(&dom).unwrap();
    }
}
