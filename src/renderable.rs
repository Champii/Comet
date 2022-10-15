use crate::element::Element;
use std::fmt::Debug;

pub trait Renderable<Msg> {
    fn render(&self) -> String;
}

impl<Msg> Renderable<Msg> for Element<Msg>
where
    Msg: Debug,
{
    fn render(&self) -> String {
        match self {
            Element::Text(text) => text.clone(),
            Element::Node {
                tag,
                attrs,
                events,
                children,
            } => {
                let mut result = String::new();

                result.push_str(&format!("<{}", tag));
                for (name, event) in events {
                    result.push_str(&format!(" on{}=\"{:?}\"", name, event));
                }
                for (attr_name, value) in attrs {
                    result.push_str(&format!(" {}=\"{}\"", attr_name, value));
                }
                result.push_str(">");
                for child in children {
                    result.push_str(&child.render());
                }
                result.push_str(&format!("</{}>", tag));
                result
            }
        }
    }
}
