use std::{collections::BTreeMap, fmt::Debug};

#[derive(Debug, Clone)]
pub enum Element<Msg> {
    Text(String),
    Node {
        tag: String,
        attrs: BTreeMap<String, String>,
        events: BTreeMap<String, Msg>,
        children: Vec<Self>,
    },
}
