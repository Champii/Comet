use web_sys::HtmlElement;

pub trait Renderable<Msg> {
    fn render(&self) -> web_sys::Element;
}
