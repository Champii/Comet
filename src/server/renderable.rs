use crate::{
    element::Element,
    prelude::{Component, Renderable},
};

impl<Msg, Comp> Renderable<Msg, Comp> for Element<Msg>
where
    Comp: Component<Msg>,
    Msg: Clone,
{
    type Output = ();

    fn render<F>(&self, _f: F) -> Self::Output
    where
        F: Fn(Msg) + Clone + 'static,
    {
        ()
    }
}
