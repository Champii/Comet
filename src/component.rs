use crate::element::Element;

pub trait Component<Msg>: 'static
where
    Msg: Clone,
{
    fn update(&mut self, msg: Msg);
    fn view(&self) -> Element<Msg>;
}
