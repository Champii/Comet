use crate::{element::Element, prelude::Renderable};

impl<Msg> Renderable<Msg> for Element<Msg>
where
    Msg: Clone + 'static,
{
    type Output = ();

    fn render<F>(&self, _f: F) -> Self::Output
    where
        F: Fn(Msg) + Clone + 'static,
    {
        ()
    }
}
