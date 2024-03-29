use comet::prelude::*;

#[derive(Default)]
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

#[derive(Default)]
pub struct App {
    counter: Shared<Counter>,
    counter2: Shared<Counter>,
}

component! {
    App {
        div {
            self.counter.clone()
            self.counter2.clone()
        }
    }
}

comet::run!(App::default());
