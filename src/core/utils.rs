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
    // use wasm_bindgen::JsCast;
    use percy_dom::JsCast;

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

    let mut events = events
        .into_iter()
        .map(|(event_name, event)| (event_name, Some(event)))
        .collect::<Vec<(_, _)>>();

    if let Some(binding) = &binding {
        if tag == "input" {
            let input: &web_sys::HtmlInputElement = elem.dyn_ref().unwrap();

            input.set_value(binding);

            events.push(("blur".to_string(), None));
        }
    }

    for (event_name, event) in events {
        let mut event_name = event_name;
        if event_name == "click" {
            event_name = "mousedown".to_string();
        }

        let f = f.clone();

        let closure = Closure::wrap(Box::new(move || {
            f(event.clone());
        }) as Box<dyn FnMut()>);

        elem.add_event_listener_with_callback(&event_name, closure.as_ref().unchecked_ref())
            .unwrap();

        // FIXME: leak
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

#[derive(Serialize, Deserialize, Debug)]
pub enum Event<T> {
    Insert(T),
    Update(T),
    Delete(i32),
}

impl<T> Event<T> {
    pub fn is_insert(&self) -> bool {
        match self {
            Event::Insert(_) => true,
            _ => false,
        }
    }
    pub fn is_update(&self) -> bool {
        match self {
            Event::Update(_) => true,
            _ => false,
        }
    }
    pub fn is_delete(&self) -> bool {
        match self {
            Event::Delete(_) => true,
            _ => false,
        }
    }
}

//impl from<PgEvent>
#[cfg(not(target_arch = "wasm32"))]
use reactive_pg::Event as PgEvent;

#[cfg(not(target_arch = "wasm32"))]
impl<T, M> From<PgEvent<T>> for Event<M>
where
    T: Into<M>,
{
    fn from(event: PgEvent<T>) -> Self {
        match event {
            PgEvent::Insert(t) => Event::Insert(t.into()),
            PgEvent::Update(t) => Event::Update(t.into()),
            PgEvent::Delete(id) => Event::Delete(id),
        }
    }
}

/* impl From<&str> for VirtualNode {
    fn from(text: &str) -> Self {
        VirtualNode::text(text.to_string())
    }
}

impl From<()> for VirtualNode {
    fn from(_text: ()) -> Self {
        VirtualNode::text("".to_string())
    }
}

impl From<String> for VirtualNode {
    fn from(text: String) -> Self {
        VirtualNode::text(text)
    }
}

impl From<i32> for VirtualNode {
    fn from(i: i32) -> Self {
        VirtualNode::text(i.to_string())
    }
}

impl From<&i32> for VirtualNode {
    fn from(i: &i32) -> Self {
        VirtualNode::text(i.to_string())
    }
}

impl From<u32> for VirtualNode {
    fn from(i: u32) -> Self {
        VirtualNode::text(i.to_string())
    }
}

impl<T: Into<VirtualNode>> From<Vec<T>> for VirtualNode {
    fn from(vec: Vec<T>) -> Self {
        let mut children = vec![];

        for child in vec {
            children.push(child.into());
        }
        let mut elem = VElement::new("div");
        elem.children = children;

        VirtualNode::from(elem)
    }
} */
