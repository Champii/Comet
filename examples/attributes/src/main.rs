use comet::prelude::*;

pub struct Counter {
    pub i: i32,
}

component! {
    Counter {
        button
            click: self.i += 1
            style: { height: (20 + self.i * 10).to_string() } {
            self.i
        }
    }
}

comet::run!(Counter { i: 0 });
