#[cfg(target_arch = "wasm32")]
#[cfg(test)]
mod html_test {
    use crate::prelude::*;

    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);

    #[derive(Clone, Debug)]
    struct Msg;

    /* async fn assert_component<
        T: Into<Shared<Comp>>,
        Comp: Component<Msg> + 'static,
        Msg: Clone + 'static,
    >(
        view: T,
        expected: &str,
    ) {
        let view = view.into();

        let elem = view.blocking_read().view(view.clone()).await;

        let html = elem.render::<_, Msg>(Box::new(|_| ()));

        assert_eq!(html.outer_html(), expected);
    } */

    macro_rules! assert_html {
        ($s:expr, $($t:tt)+) => {
            let elem = html! {
                $($t)+
            }.render().outer_html();

            assert_eq!(elem, $s);
        };
    }

    #[wasm_bindgen_test]
    fn mono_element() {
        assert_html! {
            "<div></div>",
            div {}
        };
    }

    #[wasm_bindgen_test]
    fn one_level_nested() {
        assert_html! {
            "<div><div></div></div>",
            div {
                div {}
            }
        };
    }

    #[wasm_bindgen_test]
    fn two_level_nested() {
        assert_html! {
            "<div><div><div></div></div></div>",
            div {
                div {
                    div {}
                }
            }
        };
    }

    #[wasm_bindgen_test]
    fn one_sibling() {
        assert_html! {
            "<div><span></span><span></span></div>",
            div {
                span {}
                span {}
            }
        };
    }

    #[wasm_bindgen_test]
    fn inner_text() {
        assert_html! {
            "<div><button>Increment</button></div>",
            div {
                button {
                    "Increment"
                }
            }

        };
    }

    #[wasm_bindgen_test]
    fn style() {
        assert_html! {
            "<div style=\"height: 100;\"><button style=\"margin: 10;\">Increment</button></div>",
            div style: { height: 100 } {
                button style: { margin: 10 } {
                    "Increment"
                }
            }
        };
    }

    // FIXME: Why the component doesnt have access to this struct when declared inside the
    // test function ?
    #[wasm_bindgen_test]
    fn struct_usage() {
        pub struct Test {
            pub i: i32,
        }

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
        let test = Test { i: 0 };

        assert_html! {
            "<div style=\"height: 0;background: red;\">0</div>",
            div style: {
                height: test.i
                background: test.color()
            } {
                test.i
            }
        };
    }

    #[wasm_bindgen_test]
    fn class_shortcut() {
        assert_html! {
            "<div class=\"class1 class2\">test</div>",
            div.class1.class2 {
                "test"
            }
        };
    }

    #[wasm_bindgen_test]
    fn id_shortcut() {
        assert_html! {
            "<div id=\"my_id\">test</div>",
            div #my_id {
                "test"
            }
        };
    }

    #[wasm_bindgen_test]
    fn class_and_id_shortcut() {
        assert_html! {
            "<div id=\"my_id\" class=\"class1 class2\">test</div>",
            div #my_id.class1.class2 {
                "test"
            }
        };
    }

    #[wasm_bindgen_test]
    fn test_if() {
        fn view(i: u32) -> Html {
            html! {
                div {
                    if i > 0 {
                        div {
                            "test"
                        }
                    }
                }
            }
        }

        assert_eq!(view(0).render().outer_html(), "<div></div>");
        assert_eq!(view(1).render().outer_html(), "<div><div>test</div></div>");
    }

    /* struct Test2 {
        arr: Vec<i32>,
    }

    #[wasm_bindgen_test]
    fn test_for() {
        component! {
            Test2 {
                div {
                    for (_i, item) in self.arr.iter().enumerate() {
                        div {
                            item
                        }
                    }
                }
            }
        };

        assert_html::<_, _, __component_test2::Msg>(
            Test2 { arr: vec![1, 2, 3] },
            "<div><span><div>1</div><div>2</div><div>3</div></span></div>",
        );
    } */

    #[wasm_bindgen_test]
    fn mixed_text_and_node() {
        assert_html! {
            "<div>test<div>test</div>test</div>",
            div {
                "test"
                div {
                    "test"
                }
                "test"
            }
        };
    }

    /* #[wasm_bindgen_test]
    fn bindings() {
        component! {
            String,
            div {
                input ={ *self } {}
            }
        };

        // FIXME: Need better test for that
        assert_html::<_, _, __component_string::Msg>("lol".to_string(), "<div><input></div>");
    } */
}
