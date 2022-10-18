use crate::prelude::Component;

pub trait Renderable<Msg, Comp>
where
    Comp: Component<Msg>,
    Msg: Clone,
{
    type Output;

    fn render<F>(&self, f: F) -> Self::Output
    where
        F: Fn(Msg) + Clone + 'static;
}
