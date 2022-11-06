use async_trait::async_trait;
use vdom::{Html, Render};
use wasm_bindgen_futures::spawn_local;

use crate::prelude::Shared;

#[async_trait]
pub trait Component<Msg>: Send + Sync + 'static
where
    Msg: Clone + 'static,
    Self: Sized,
{
    async fn update(&mut self, msg: Msg);
    async fn view(&self, shared_self: Shared<Self>) -> Html;

    fn callback() -> Box<dyn Fn(Shared<Self>) -> Box<dyn Fn(Msg)>> {
        Box::new(move |shared| {
            Box::new(move |msg| {
                let shared = shared.clone();

                spawn_local(async move {
                    shared.write().await.update(msg).await;
                });
            })
        })
    }
    /* fn callback(this: Shared<Self>) -> Box<dyn Fn(Msg)> {
        Box::new(move |msg| {
            let this = this.clone();

            spawn_local(async move {
                this.write().await.update(msg).await;
            })
        })
    } */
    // fn update_bindings(&mut self, bindings: Shared<Vec<web_sys::Element>>);
}

/* impl<Msg> Render for Component<Msg>
where
    Msg: Clone + 'static,
{
    fn render<F>(&self, f: Box<F>) -> web_sys::Element
    where
        F: Fn(Msg) + Clone + 'static,
        Msg: Any + Sized + Clone + 'static,
    {
        self.view().render(f)
    }
} */

pub async fn run_component<Msg, Comp>(comp: Shared<Comp>, parent: &web_sys::Element)
where
    Comp: Component<Msg>,
    Msg: Clone + 'static,
{
    let comp2 = comp.clone();
    let parent2 = parent.clone();

    let cb = move |msg| {
        let comp = comp2.clone();
        let parent = parent2.clone();

        spawn_local(async move {
            comp.write().await.update(msg).await;

            run_component(comp, &parent).await;
        })
    };

    let view = comp.read().await.view(comp.clone()).await;

    let dom = view.render();

    // TODO: Diff + Patch
    parent.set_inner_html("");
    parent.append_child(&dom).unwrap();
}
