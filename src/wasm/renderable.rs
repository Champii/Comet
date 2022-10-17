use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use crate::prelude::*;

impl<Msg> Renderable<Msg> for Element<Msg> {
    type Output = web_sys::Element;

    fn render(&self) -> Self::Output {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");

        match self {
            Element::Text(text) => {
                let elem = document.create_element("span").unwrap();
                elem.set_inner_html(text);
                elem.into()
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

                for (_name, _event) in events {
                    let closure = Closure::wrap(Box::new(move || {
                        console_log!("clicked");
                        // inner_closure();
                    }) as Box<dyn FnMut()>);

                    elem.dyn_ref::<HtmlElement>()
                        .expect("#loading should be an `HtmlElement`")
                        .set_onclick(Some(closure.as_ref().unchecked_ref()));

                    closure.forget();
                }

                for child in children {
                    elem.append_child(&child.render()).unwrap();
                }

                console_log!("elem: {:?}", elem);

                elem
            }
        }
    }
}
