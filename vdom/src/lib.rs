use std::any::Any;
use std::fmt::{Display, Formatter};

use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

pub type Html = VElement;

pub trait Render {
    fn render(&mut self) -> web_sys::Element;
}

#[derive(Debug)]
pub enum VElement {
    Tag(VTag),
    Text(String),
    // Custom(Box<dyn Render>),
}

impl VElement {
    pub fn fix_events<Msg: Clone + 'static, F: Fn(Msg) + Clone + 'static>(
        &mut self,
        i: &mut usize,
        events: &Vec<Msg>,
        callback: Box<F>,
    ) {
        match self {
            VElement::Tag(tag) => {
                tag.fix_events(i, events, callback);
            }
            VElement::Text(_) => {}
        }
    }
}

impl Render for VElement {
    fn render(&mut self) -> web_sys::Element {
        match self {
            VElement::Tag(tag) => tag.render(),
            VElement::Text(_text) => {
                unimplemented!();
            }
        }
    }
}

impl From<&str> for VElement {
    fn from(text: &str) -> Self {
        VElement::Text(text.to_string())
    }
}

impl From<String> for VElement {
    fn from(text: String) -> Self {
        VElement::Text(text)
    }
}

impl<T: Into<VElement>> From<Vec<T>> for VElement {
    fn from(vec: Vec<T>) -> Self {
        let mut children = vec![];

        for child in vec {
            children.push(child.into());
        }

        VElement::Tag(VTag {
            tag: "div".to_string(),
            attrs: vec![],
            children,
        })
    }
}

/* pub trait EventCallback: Fn(Box<dyn Any>) + Sized + Clone + 'static {
    fn call(self) {
        self()
    }
} */

#[derive(Debug)]
pub struct VTag {
    tag: String,
    attrs: Vec<VAttribute>,
    children: Vec<VElement>,
}

impl VTag {
    pub fn new(tag: String, attrs: Vec<VAttribute>, children: Vec<VElement>) -> Self {
        Self {
            tag,
            attrs,
            children,
        }
    }

    pub fn fix_events<Msg: Clone + 'static, F: Fn(Msg) + Clone + 'static>(
        &mut self,
        i: &mut usize,
        events: &Vec<Msg>,
        callback: Box<F>,
    ) {
        for attr in self.attrs.iter_mut() {
            attr.fix_events(i, events, callback.clone());
        }

        for child in self.children.iter_mut() {
            // Short circuit child component, they have their own fix_events
            match child {
                VElement::Tag(tag) => {
                    if tag
                        .attrs
                        .iter()
                        .find(|attr| attr.key == "__component")
                        .is_some()
                    {
                        continue;
                    }

                    child.fix_events(i, events, callback.clone());
                }
                _ => {}
            }
        }
    }

    pub fn push_attr(&mut self, attr: VAttribute) {
        self.attrs.push(attr);
    }
}

impl Render for VTag {
    fn render(&mut self) -> web_sys::Element {
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.create_element(&self.tag).unwrap();

        for attr in &mut self.attrs {
            if attr.key == "__component" {
                continue;
            }

            match &mut attr.value {
                VAttributeValue::String(ref value) => {
                    element.set_attribute(&attr.key, &value).unwrap();
                }
                VAttributeValue::Attributes(ref attrs) => {
                    let value_str = &attrs
                        .iter()
                        .map(|attr| {
                            format!("{}: {};", attr.key, attr.value.to_string()).to_string()
                        })
                        .collect::<Vec<_>>()
                        .join("");

                    element.set_attribute(&attr.key, value_str).unwrap();
                }
                VAttributeValue::Event(ref mut cb) => {
                    // let real_msg = msg.downcast_ref::<Msg>().unwrap();
                    // let real_msg = real_msg.clone();
                    /* let f = f.clone();
                    let closure = Closure::wrap(Box::new(move || f(msg)) as Box<dyn FnMut()>); */
                    // let cb = cb.clone();
                    let orig = cb.take().unwrap();

                    // let placeholder = Closure::wrap(Box::new(move || {}) as Box<dyn Fn()>);
                    *cb = None;

                    element
                        .add_event_listener_with_callback(&attr.key, orig.as_ref().unchecked_ref())
                        .unwrap();

                    orig.forget();
                    // cb.forget();

                    // cb.forget();

                    // FIXME: leak
                    // closure.forget();
                }
            }
        }

        for child in &mut self.children {
            match child {
                VElement::Tag(tag) => {
                    /* if tag
                        .attrs
                        .iter()
                        .find(|attr| attr.key == "__component")
                        .is_some()
                    {
                        continue;
                    } */

                    element.append_child(&tag.render()).unwrap();
                }
                VElement::Text(text) => {
                    let document = web_sys::window().unwrap().document().unwrap();
                    let text_node = document.create_text_node(&text);

                    element.append_child(&text_node).unwrap();
                }
            }
        }

        element
    }
}

#[derive(Debug)]
pub struct VAttribute {
    key: String,
    value: VAttributeValue,
}

impl VAttribute {
    pub fn new(key: String, value: VAttributeValue) -> Self {
        Self { key, value }
    }

    pub fn fix_events<Msg: Clone + 'static, F: Fn(Msg) + Clone + 'static>(
        &mut self,
        i: &mut usize,
        events: &Vec<Msg>,
        callback: Box<F>,
    ) {
        match self.value {
            VAttributeValue::Event(ref mut f) => {
                let msg = Box::new(events[*i].clone());
                let closure = Closure::wrap(Box::new(move || {
                    let msg = msg.clone();
                    callback(*msg)
                }) as Box<dyn Fn()>);
                *f = Some(closure);

                *i += 1;
            }
            _ => {}
        }
    }
}

pub enum VAttributeValue {
    String(String),
    // Event(Box<dyn Any>),
    Event(Option<Closure<dyn Fn()>>),
    Attributes(Vec<VAttribute>),
}

// FIXME: is this safe ? The Box<dyn Any> is always a Box<Msg>
unsafe impl Send for VAttributeValue {}

impl Display for VAttributeValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            VAttributeValue::String(value) => write!(f, "{}", value),
            VAttributeValue::Event(_) => write!(f, "event"),
            VAttributeValue::Attributes(attrs) => {
                let value_str = &attrs
                    .iter()
                    .map(|attr| format!("{}:{};", attr.key, attr.value.to_string()).to_string())
                    .collect::<Vec<_>>()
                    .join("");

                write!(f, "{}", value_str)
            }
        }
    }
}

impl std::fmt::Debug for VAttributeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VAttributeValue::String(s) => write!(f, "String({})", s),
            VAttributeValue::Event(_) => write!(f, "Event"),
            VAttributeValue::Attributes(attrs) => {
                write!(f, "Attributes({:?})", attrs)
            }
        }
    }
}
