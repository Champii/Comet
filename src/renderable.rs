use crate::element::Element;
use std::fmt::Debug;

pub trait Renderable<Msg> {
    type Output;
    fn render(&self) -> Self::Output;
}

#[cfg(not(target_arch = "wasm32"))]
impl<Msg> Renderable<Msg> for Element<Msg>
where
    Msg: Debug,
{
    type Output = ();

    fn render(&self) -> () {
        ()
    }
}
