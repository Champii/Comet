use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use crate::{element, prelude::*, renderable};
use wasm_bindgen::closure::Closure;

impl<Msg> Renderable<Msg> for Element<Msg>
where
    Msg: Clone + 'static,
{
    type Output = web_sys::Element;

    fn render<F>(&self, f: F) -> web_sys::Element
    where
        F: Fn(Msg) + Clone + 'static,
    {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");

        match self {
            Element::Text(text) => {
                unreachable!()
            }
            Element::Node {
                tag,
                attrs,
                events,
                children,
            } => {
                let elem = document.create_element(tag).unwrap();

                for (attr_name, value) in attrs {
                    elem.set_attribute(attr_name, value).unwrap();
                }

                if let Some(event) = events.get("click") {
                    let f = f.clone();

                    let closure = Closure::<dyn Fn()>::wrap(Box::new(move || {
                        f(event.clone());
                    }));

                    elem.dyn_ref::<HtmlElement>()
                        .expect("#loading should be an `HtmlElement`")
                        .set_onclick(Some(closure.as_ref().unchecked_ref()));

                    // FIXME: leak
                    closure.forget();
                }

                for child in children {
                    match child {
                        Element::Text(text) => {
                            elem.set_inner_html(text);
                        }
                        Element::Node { .. } => {
                            let f = f.clone();
                            elem.append_child(&<element::Element<Msg> as renderable::Renderable<
                                Msg,
                            >>::render::<F>(
                                child, f
                            ))
                            .unwrap();
                        }
                    };
                }

                elem
            }
        }
    }
}
