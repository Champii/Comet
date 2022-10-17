mod app;
pub mod prelude;
mod renderable;

#[cfg(test)]
mod test {
    use crate::prelude::*;

    #[derive(Clone)]
    enum Msg {
        Increment,
    }

    #[derive(Clone)]
    struct TestComponent;

    impl Component<Msg> for TestComponent {
        fn update(&mut self, msg: Msg) {
            match msg {
                Msg::Increment => {}
            }
        }

        fn view(&self) -> Element<Msg> {
            html! {
                div {
                    button
                        @click: Msg::Increment, {
                        "Increment"
                    }
                }
            }
        }
    }

    #[test]
    fn test_html() {
        use crate::renderable::Renderable;

        let component = TestComponent;
        let view = component.view();

        assert_eq!(view.render(), (),);
    }
}

