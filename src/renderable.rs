use crate::element::Element;
use std::fmt::Debug;

pub trait Renderable<Msg> {
    type Output;
    fn render(&self) -> Self::Output;
}

#[cfg(target_arch = "wasm32")]
use crate::prelude::*;
#[cfg(target_arch = "wasm32")]
impl<Msg> Renderable<Msg> for Element<Msg>
where
    Msg: Debug,
{
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

#[cfg(not(target_arch = "wasm32"))]
impl<Msg> Renderable<Msg> for Element<Msg>
where
    Msg: Debug,
{
    type Output = ();

    fn render(&self) -> () {
        ()
    }
}
