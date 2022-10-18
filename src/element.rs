use std::{collections::BTreeMap, fmt::Debug};

#[derive(Debug, Clone)]
pub enum Element<Msg: Clone> {
    Text(String),
    Node {
        tag: String,
        attrs: BTreeMap<String, String>,
        events: BTreeMap<String, Msg>,
        children: Vec<Self>,
    },
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
