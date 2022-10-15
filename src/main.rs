mod component;
mod element;
mod html_macro;
mod renderable;

use component::*;
use element::*;
use renderable::*;

#[derive(Debug)]
pub enum Message {
    Increment,
    Tamere(i64),
}

pub struct Counter {
    pub value: i32,
    pub arr: Vec<i32>,
}

pub fn increment() -> Message {
    Message::Increment
}

pub fn tamere(i: i64) -> Message {
    Message::Tamere(i)
}

impl Component<Message> for Counter {
    fn update(&mut self, msg: Message) {
        match msg {
            Message::Increment => self.value += 1,
            Message::Tamere(i) => self.arr.push(i as i32),
        }
    }

    fn view(&self) -> Element<Message> {
        html! {
            button @click: increment(), @scroll: tamere(self.arr.len() as i64), {
                {{ self.value }}
            }
        }
    }
}
fn main() {
    let counter = Counter {
        value: 0,
        arr: vec![],
    };

    println!("{:#?}", counter.view());
    println!("{}", counter.view().render());
}
