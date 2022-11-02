use std::any::Any;
use std::fmt::{Display, Formatter};

use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

pub type Html = VElement;

pub trait Render {
    fn render<F, Msg>(&self, f: Box<F>) -> web_sys::Element
    where
        F: Fn(Msg) + Clone + 'static,
        Msg: Any + Sized + Clone + 'static;
}

#[derive(Debug)]
pub enum VElement {
    Tag(VTag),
    Text(String),
}

impl VElement {
    pub fn fix_events<Msg: Clone + 'static>(&mut self, i: &mut usize, events: &Vec<Msg>) {
        match self {
            VElement::Tag(tag) => {
                tag.fix_events(i, events);
            }
            VElement::Text(_) => {}
        }
    }
}

impl Render for VElement {
    fn render<F, Msg>(&self, f: Box<F>) -> web_sys::Element
    where
        F: Fn(Msg) + Clone + 'static,
        Msg: Any + Sized + Clone + 'static,
    {
        match self {
            VElement::Tag(tag) => tag.render(f),
            VElement::Text(_text) => {
                unimplemented!();
            }
        }
    }
}

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

    pub fn fix_events<Msg: Clone + 'static>(&mut self, i: &mut usize, events: &Vec<Msg>) {
        for attr in self.attrs.iter_mut() {
            attr.fix_events(i, events);
        }

        for child in self.children.iter_mut() {
            child.fix_events(i, events);
        }
    }
}

impl Render for VTag {
    fn render<F, Msg>(&self, f: Box<F>) -> web_sys::Element
    where
        F: Fn(Msg) + Clone + 'static,
        Msg: Any + Sized + Clone + 'static,
    {
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.create_element(&self.tag).unwrap();

        for attr in &self.attrs {
            match attr.value {
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
                VAttributeValue::Event(ref msg) => {
                    let real_msg = msg.downcast_ref::<Msg>().unwrap();
                    let real_msg = real_msg.clone();
                    let f = f.clone();
                    let closure =
                        Closure::wrap(Box::new(move || f(real_msg.clone())) as Box<dyn FnMut()>);

                    element
                        .add_event_listener_with_callback(
                            &attr.key,
                            closure.as_ref().unchecked_ref(),
                        )
                        .unwrap();

                    // FIXME: leak
                    closure.forget();
                }
            }
        }

        for child in &self.children {
            match child {
                VElement::Tag(tag) => {
                    element.append_child(&tag.render(f.clone())).unwrap();
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

    pub fn fix_events<Msg: Clone + 'static>(&mut self, i: &mut usize, events: &Vec<Msg>) {
        match self.value {
            VAttributeValue::Event(ref mut msg) => {
                *msg = Box::new(events[*i].clone());
                *i += 1;
            }
            _ => {}
        }
    }
}

pub enum VAttributeValue {
    String(String),
    Event(Box<dyn Any>),
    Attributes(Vec<VAttribute>),
}

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
