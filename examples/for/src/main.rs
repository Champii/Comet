component! {
    i32,
    div {
        for i in (0..*self) {
            button {
                { i }
            }
        }
    }
}

comet::run!(10);
