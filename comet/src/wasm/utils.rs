use crate::prelude::*;

pub fn create_element<F, Msg>(
    f: F,
    tag: &str,
    id_name: Option<&str>,
    class_names: Vec<&str>,
    attrs: Vec<(String, String)>,
    events: Vec<(String, Msg)>,
    binding: Option<String>,
) -> web_sys::Element
where
    F: Fn(Option<Msg>) + Clone + 'static,
    Msg: Clone + 'static,
{
    use wasm_bindgen::JsCast;

    let elem = document().create_element(tag).unwrap();

    if let Some(id_name) = id_name {
        elem.set_id(id_name);
    }

    for class_name in class_names {
        elem.class_list().add_1(class_name).unwrap();
    }

    if !attrs.is_empty() {
        elem.set_attribute(
            "style",
            &attrs
                .iter()
                .map(|(k, v)| format!("{}: {};", k, v))
                .collect::<Vec<_>>()
                .join(""),
        )
        .unwrap();
    }

    for (event_name, event) in events {
        let mut event_name = event_name;
        if event_name == "click" {
            event_name = "mousedown".to_string();
        }

        let f = f.clone();

        let closure = Closure::wrap(Box::new(move || {
            f(Some(event.clone()));
        }) as Box<dyn FnMut()>);

        elem.add_event_listener_with_callback(&event_name, closure.as_ref().unchecked_ref())
            .unwrap();

        // FIXME: leak
        closure.forget();
    }

    if let Some(binding) = binding {
        if tag == "input" {
            let input: &web_sys::HtmlInputElement = elem.dyn_ref().unwrap();

            input.set_value(&binding);

            let f = f.clone();

            let closure = Closure::wrap(Box::new(move || {
                f(None);
            }) as Box<dyn FnMut()>);

            elem.add_event_listener_with_callback("blur", closure.as_ref().unchecked_ref())
                .unwrap();

            // FIXME: leak
            closure.forget();
            // TODO: add event listener for change
        }
    }

    elem
}

pub fn document() -> web_sys::Document {
    web_sys::window()
        .expect("no global `window` exists")
        .document()
        .expect("should have a document on window")
}

#[derive(Clone)]
pub enum HtmlNode {
    Text(web_sys::Text),
    Element(web_sys::Element),
}

impl HtmlNode {
    pub fn into_element(self) -> web_sys::Element {
        match self {
            HtmlNode::Text(text) => panic!("Expected element, got text: {:?}", text),
            HtmlNode::Element(elem) => elem,
        }
    }

    pub fn append_to(self, parent: &web_sys::Element) {
        match self {
            HtmlNode::Text(text) => parent.append_child(&text).unwrap(),
            HtmlNode::Element(elem) => parent.append_child(&elem).unwrap(),
        };
    }
}
