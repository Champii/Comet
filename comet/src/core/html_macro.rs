
/* #[cfg(target_arch = "wasm32")]
#[cfg(test)]
mod html_test {
    use crate::{element, prelude::*, renderable};

    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);

    #[derive(Clone, Debug)]
    struct Msg;

    fn assert_html(view: &Element<Msg>, expected: &str) {
        let elem =
            <element::Element<Msg> as renderable::Renderable<Msg>>::render::<_>(&view, |_| {});

        assert_eq!(elem.outer_html(), expected);
    }

    #[wasm_bindgen_test]
    fn mono_element() {
        let view = html! {
            div {
            }
        };

        assert_html(&view, "<div></div>");
    }

    #[wasm_bindgen_test]
    fn one_level_nested() {
        let view = html! {
            div {
                div {}
            }
        };

        assert_html(&view, "<div><div></div></div>");
    }

    #[wasm_bindgen_test]
    fn two_level_nested() {
        let view = html! {
            div {
                div {
                    div {}
                }
            }
        };

        assert_html(&view, "<div><div><div></div></div></div>");
    }

    #[wasm_bindgen_test]
    fn one_sibling() {
        let view = html! {
            div {
                span {}
                span {}
            }
        };

        assert_html(&view, "<div><span></span><span></span></div>");
    }

    #[wasm_bindgen_test]
    fn test_html() {
        let view = html! {
            div {
                button {
                    {{ "Increment" }}
                }
            }
        };

        assert_html(&view, "<div><button>Increment</button></div>");
    }
} */


mod lol {
    use crate::prelude::*;

    pub struct Counter {
        value: i32,
    }

    component! { Counter,
        button @click: { self.value += 1 } {
            { self.value }
        }
    }
}
