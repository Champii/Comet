pub trait Renderable<Msg>
where
    Msg: Clone + 'static,
{
    type Output;

    fn render<F>(&self, f: F) -> Self::Output
    where
        F: Fn(Msg) + Clone + 'static;
}
