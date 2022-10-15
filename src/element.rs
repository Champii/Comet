use std::{collections::BTreeMap, fmt::Debug};

pub enum Element<Msg> {
    Text(String),
    Node {
        tag: String,
        events: BTreeMap<String, Msg>,
        children: Vec<Box<Self>>,
    },
}

impl<Msg> Debug for Element<Msg> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::Text(text) => write!(f, "Text({:?})", text),
            Element::Node {
                tag,
                events,
                children,
            } => {
                write!(f, "Node({:?}, [", tag)?;
                for (event, _) in events {
                    write!(f, "{:?}, ", event)?;
                }
                write!(f, "], [")?;
                for child in children {
                    write!(f, "{:?}, ", child)?;
                }
                write!(f, "])")
            }
        }
    }
}
