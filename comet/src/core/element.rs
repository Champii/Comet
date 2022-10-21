use std::fmt::{Debug, Formatter};
use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use crate::prelude::Component;

pub enum Element<Msg: Clone> {
    Text(String),
    Node {
        tag: String,
        attrs: BTreeMap<String, String>,
        events: BTreeMap<String, Msg>,
        children: Vec<Self>,
    },
    // Component(Rc<RefCell<Box<dyn Component<Msg>>>>),
}

impl<Msg: Clone> Debug for Element<Msg> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::Text(text) => write!(f, "Text({})", text),
            Element::Node {
                tag,
                attrs,
                events: _events,
                children,
            } => write!(f, "Node({} {:?} {:?})", tag, attrs, children),
            // Element::Component(_) => write!(f, "Component"),
        }
    }
}

impl<Msg, T> From<T> for Element<Msg>
where
    T: Into<String>,
    Msg: Clone,
{
    fn from(text: T) -> Self {
        Element::Text(text.into())
    }
}
