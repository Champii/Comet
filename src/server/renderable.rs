use crate::{element::Element, prelude::Renderable};

impl<Msg> Renderable<Msg> for Element<Msg> {
    type Output = ();

    fn render(&self) -> () {
        ()
    }
}
