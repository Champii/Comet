mod app;
pub mod component;
mod shared;

pub mod prelude;

mod just_to_test {
    use crate::prelude::*;

    pub struct Counter {
        value: i32,
    }

    component! {
        Counter,
        button @click: { self.value += 1 } {
            for i in (0..10) {
                { i }
            }
        }
    }
}

pub fn create_element<F, Msg>(
    f: F,
    tag: &str,
    id_name: Option<&str>,
    class_names: Vec<&str>,
    attrs: Vec<(String, String)>,
    events: Vec<(String, Msg)>,
) -> web_sys::Element
where
    F: Fn(Msg) + Clone + 'static,
    Msg: Clone + 'static,
{
    use self::prelude::*;
    use wasm_bindgen::JsCast;

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let elem = document.create_element(tag).unwrap();

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
        let f = f.clone();
        let closure = Closure::wrap(Box::new(move || {
            f(event.clone());
        }) as Box<dyn FnMut()>);

        elem.add_event_listener_with_callback(&event_name, closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    elem
}

pub fn document() -> web_sys::Document {
    web_sys::window()
        .expect("no global `window` exists")
        .document()
        .expect("should have a document on window")
}
