mod app;
pub mod component;
mod proto;
mod shared;
mod utils;

pub mod prelude;

/* mod just_to_test {
    use crate::prelude::*;

    pub struct Counter {
        value: i32,
    }

    component! {
        Counter {
            button click: self.value += 1 {
                /* for i in (0..10) {
                    { i }
                } */
            }
        }
    }
} */
