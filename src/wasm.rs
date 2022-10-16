use std::fmt::Debug;
use wasm_bindgen::prelude::*;

use crate::{element::Element, prelude::Renderable};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

impl<Msg> Renderable<Msg> for Element<Msg>
where
    Msg: Debug,
{
    fn render(&self) -> web_sys::Element {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let body = document.body().expect("document should have a body");

        match self {
            Element::Text(text) => {
                let elem = document.create_element("p").unwrap();
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

                for (name, event) in events {
                    // elem.set_attribute(name, &format!("{:?}", event)).unwrap();
                    // TODO: Closures
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
