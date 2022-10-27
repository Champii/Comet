#[async_trait]
pub trait Component<Msg>: Send + Sync + 'static
where
    Msg: Clone + 'static,
{
    async fn update(&mut self, msg: Msg);
    fn view<F>(&self, f: F, bindings: Shared<Vec<web_sys::Element>>) -> web_sys::Element
    where
        F: Fn(Option<Msg>) + Clone + 'static;
    fn update_bindings(&mut self, bindings: Shared<Vec<web_sys::Element>>);
}

use crate::prelude::*;
use wasm_bindgen_futures::spawn_local;

pub async fn run_rec<Msg, Comp>(component: Shared<Comp>, parent: &web_sys::Element)
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
            component2
                .borrow_mut()
                .update_bindings(bindings_clone.clone());

            if let Some(msg) = msg {
                let component3 = component2.clone();
                component3.borrow_mut().update(msg).await;
            }

            let parent3 = parent2.clone();

            let component4 = component2.clone();
            run_rec(component4.clone(), &parent3).await;
        })
    };

    #[cfg(target_arch = "wasm32")]
    crate::console_log!("run_rec");
    let view = component.borrow().view(cb, bindings.clone());
    #[cfg(target_arch = "wasm32")]
    crate::console_log!("run_rec_lol");

    parent.set_inner_html("");
    parent.append_child(&view).unwrap();
}
