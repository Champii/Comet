mod app;
mod component;
mod element;
mod html_macro;
mod prelude;
mod renderable;

use prelude::*;

#[derive(Debug, Clone)]
pub enum Msg {
    Increment,
}

pub struct Counter {
    pub value: i32,
}

impl Counter {
    pub fn new() -> Self {
        Self { value: 0 }
    }
}

impl Component<Msg> for Counter {
    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Increment => self.value += 1,
        }
    }

    fn view(&self) -> Element<Msg> {
        html! {
            div [height: 100] {
                span {
                    {{ 2 }}
                },
                button
                    [style: "background-color: red;"]
                    @click: Msg::Increment, {
                    {{ self.value }}
                }
            }
        }
    }
}

fn main() {
    let mut app = App::new(Counter::new());

    app.run();
}

pub struct Button {
    pub is_clicked: bool,
}

impl Button {
    pub fn new() -> Self {
        Self { is_clicked: false }
    }
}

#[derive(Debug, Clone)]
pub enum ButtonMsg {
    Click,
}

impl Component<ButtonMsg> for Button {
    fn update(&mut self, msg: ButtonMsg) {
        match msg {
            ButtonMsg::Click => self.is_clicked = true,
        }
    }

    fn view(&self) -> Element<ButtonMsg> {
        html! {
            button
                [style: "background-color: red;"]
                @click: ButtonMsg::Click, {
                {{ self.is_clicked }}
            }
        }
    }
}
