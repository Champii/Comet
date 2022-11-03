use std::any::Any;

use async_trait::async_trait;
use vdom::{Html, Render, VElement};
use wasm_bindgen_futures::spawn_local;

use crate::prelude::Shared;

#[async_trait]
pub trait Component<Msg>: Send + Sync + 'static
where
    Msg: Clone + 'static,
    Self: Sized,
{
    async fn update(&mut self, msg: Msg);
    async fn view(&self) -> Html;

    fn callback(this: Shared<Self>) -> Box<dyn Fn(Msg)> {
        Box::new(move |msg| {
            let this = this.clone();

            spawn_local(async move {
                this.write().await.update(msg).await;
            })
        })
    }
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

    let view = comp.read().await.view().await;

    let dom = view.render(Box::new(cb));

    // TODO: Diff + Patch
    parent.set_inner_html("");
    parent.append_child(&dom).unwrap();
}

/* pub async fn run_rec<Msg, Comp>(component: Shared<Comp>, parent: &web_sys::Element)
where
    Comp: Component<Msg>,
    Msg: Clone + 'static,
{
    let component2 = component.clone();

    let parent2 = parent.clone();

    let bindings: Shared<Vec<web_sys::Element>> = vec![].into();
    let bindings_clone = bindings.clone();

    let cb = move |msg| {
        let component2 = component2.clone();
        let parent2 = parent2.clone();
        let bindings_clone = bindings_clone.clone();

        spawn_local(async move {
            /* component2
            .write()
            .await
            .update_bindings(bindings_clone.clone()); */

            if let Some(msg) = msg {
                let component3 = component2.clone();
                component3.write().await.update(msg).await;
            }

            let parent3 = parent2.clone();

            let component4 = component2.clone();

            run_rec(component4.clone(), &parent3).await;
        })
    };

    let view = component.read().await.view(cb, bindings.clone());

    parent.set_inner_html("");
    parent.append_child(&view).unwrap();
} */
