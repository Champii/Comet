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
    let counter = Counter { value: 0 };

    println!("{:#?}", counter.view());
    println!("{}", counter.view().render());
}
