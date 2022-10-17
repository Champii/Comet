pub trait Renderable<Msg> {
    type Output;

    fn render(&self) -> Self::Output;
}
