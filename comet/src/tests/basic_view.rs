#[cfg(target_arch = "wasm32")]
#[cfg(test)]
mod html_test {
    use crate::prelude::*;

    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);

    #[derive(Clone, Debug)]
    struct Msg;

    fn assert_html<T: Into<Shared<Comp>>, Comp: Component<Msg>, Msg: Clone + 'static>(
        view: T,
        expected: &str,
    ) {
        let view = view.into();

        let elem = view.borrow().view(|_| {});

        assert_eq!(elem.outer_html(), expected);
    }

    #[wasm_bindgen_test]
    fn mono_element() {
        component! {
            i32,
            div {}
        };

        assert_html::<_, _, __component_i32::Msg>(0, "<div></div>");
    }

    #[wasm_bindgen_test]
    fn one_level_nested() {
        component! {
            i32,
            div {
                div {}
            }
        };

        assert_html::<_, _, __component_i32::Msg>(0, "<div><div></div></div>");
    }

    #[wasm_bindgen_test]
    fn two_level_nested() {
        component! {
            i32,
            div {
                div {
                    div {}
                }
            }
        };

        assert_html::<_, _, __component_i32::Msg>(0, "<div><div><div></div></div></div>");
    }

    #[wasm_bindgen_test]
    fn one_sibling() {
        component! {
            i32,
            div {
                span {}
                span {}
            }
        };

        assert_html::<_, _, __component_i32::Msg>(0, "<div><span></span><span></span></div>");
    }

    #[wasm_bindgen_test]
    fn inner_text() {
        component! {
            i32,
            div {
                button {
                    { "Increment" }
                }
            }
        };

        assert_html::<_, _, __component_i32::Msg>(
            0,
            "<div><button><span>Increment</span></button></div>",
        );
    }

    #[wasm_bindgen_test]
    fn style() {
        component! {
            i32,
            div [height: {100}] {
                button [margin: {10}] {
                    { "Increment" }
                }
            }
        };

        assert_html::<_, _, __component_i32::Msg>(
            0,
            "<div style=\"height: 100;\"><button style=\"margin: 10;\"><span>Increment</span></button></div>",
        );
    }

    // FIXME: Why the component doesnt have access to this struct when declared inside the
    // test function ?
    pub struct Test {
        pub i: i32,
    }

    #[wasm_bindgen_test]
    fn self_usage() {
        impl Test {
            fn color(&self) -> &'static str {
                match self.i {
                    0 => "red",
                    1 => "green",
                    2 => "blue",
                    _ => "black",
                }
            }
        }

        component! {
            Test,
            div [height: {self.i}, background: {self.color()}] {
                { self.i }
            }
        };

        assert_html::<_, _, __component_test::Msg>(
            Test { i: 0 },
            "<div style=\"background: red;height: 0;\"><span>0</span></div>",
        );
    }

    #[wasm_bindgen_test]
    fn class_shortcut() {
        component! {
            i32,
            div.class1.class2 {
                { "test" }
            }
        };

        assert_html::<_, _, __component_i32::Msg>(
            0,
            "<div class=\"class1 class2\"><span>test</span></div>",
        );
    }

    #[wasm_bindgen_test]
    fn id_shortcut() {
        component! {
            i32,
            div #my_id {
                { "test" }
            }
        };

        assert_html::<_, _, __component_i32::Msg>(0, "<div id=\"my_id\"><span>test</span></div>");
    }

    #[wasm_bindgen_test]
    fn class_and_id_shortcut() {
        component! {
            i32,
            div #my_id.class1.class2 {
                { "test" }
            }
        };

        assert_html::<_, _, __component_i32::Msg>(
            0,
            "<div id=\"my_id\" class=\"class1 class2\"><span>test</span></div>",
        );
    }

    #[wasm_bindgen_test]
    fn duplicated_event() {
        component! {
            i32,
            div {
                div @click: { *self += 1 } {
                    { "test" }
                }
                div @click: { *self += 1 } {
                    { "test" }
                }
            }
        };

        assert_html::<_, _, __component_i32::Msg>(
            0,
            "<div><div><span>test</span></div><div><span>test</span></div></div>",
        );
    }
}
