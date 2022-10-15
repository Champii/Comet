use crate::element::Element;

pub trait Component<Msg> {
    fn update(&mut self, msg: Msg);
    fn view(&self) -> Element<Msg>;
}
