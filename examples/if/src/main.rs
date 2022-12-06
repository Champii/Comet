use comet::prelude::*;

#[derive(Default)]
pub struct Toggle {
    show: bool,
}

component! {
    Toggle {
        div {
            button click: self.show = !self.show {
                if self.show { "Visible" } else { "Hidden" }
            }
            if self.show {
                div {
                    "This is visible"
                }
            }
        }
    }
}

comet::run!(Toggle::default());
