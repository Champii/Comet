use comet::prelude::*;

pub struct Counter {
    pub i: i32,
}

component! {
    Counter {
        div {
            for i in 0..self.i {
                button {
                    i
                }
            }
        }
    }
}

comet::run!(Counter { i: 10 });
