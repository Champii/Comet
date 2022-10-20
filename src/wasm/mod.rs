mod app;
mod component;
mod log_macro;
pub mod prelude;
mod renderable;

/* #[cfg(test)]
mod test {

    use crate::prelude::*;
    use std::{cell::RefCell, rc::Rc};
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[derive(Clone, Debug)]
    enum Msg {
        Increment,
    }

    struct TestComponent {
        pub value: i32,
    }

    impl Component<Msg> for TestComponent {
        fn update(&mut self, msg: Msg) {
            match msg {
                Msg::Increment => self.value += 1,
            }
        }

        fn view(&self) -> Element<Msg> {
            let toto = InnerComponent.into_shared_dyn();
            let elem = html! {
                div [height: 100] {
                    span {
                        button @click: Msg::Increment, {
                            {{ self.value }}
                        }
                    }
                    @toto,
                }
            };

            elem
        }
    }

    struct InnerComponent;
    impl Component<Msg> for InnerComponent {
        fn update(&mut self, _msg: Msg) {}

        fn view(&self) -> Element<Msg> {
            html! {
                span {
                    {{ "Inner" }}
                }
            }
        }
    }

    #[wasm_bindgen_test]
    fn test_html() {
        let component = TestComponent { value: 2 };

        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let elem = document.create_element("div").unwrap();

        crate::component::run_rec(Rc::new(RefCell::new(Box::new(component))), &elem);

        assert_eq!(
            elem.inner_html(),
            r#"<div height="100"><span><button>2</button></span><span><span>Inner</span></span></div>"#
        );
    }
} */
