use crate::prelude::*;

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
