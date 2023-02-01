use comet::prelude::*;

#[derive(Default, Debug)]
pub struct Counter {
    pub value: i32,
}

component! {
    Counter {
        button click: self.value += 1 {
            self.value
        }
    }
}

#[derive(Default, Debug)]
pub struct App {
    counter: Shared<Counter>,
    counter2: Shared<Counter>,
    value: i32,
}

component! {
    App {
        div {
            self.counter
            self.counter2
            2
        }
    }
}

comet::run!(App::default());
