component! {
    i32,
    button @click: { *self += 1 } {
        { self }
    }
}

#[derive(Default)]
pub struct App {
    counter: Shared<i32>,
    counter2: Shared<i32>,
}

component! {
    App,
    div {
        @{self.counter}
        @{self.counter2}
        @{Shared::from(9)}
    }
}

comet::run!(App::default());
