#[macro_export]
macro_rules! html_arr {
    // tag
    (
        {
            {
                {
                    $tag:ident
                        $([$($attr_name:ident : $attr_value:expr),*])?
                        $($(@$ev:ident : $evcode:expr ),+, )?
                        { $($e:tt)* }

                    $($rest:tt)*
                }
                [$($expanded:tt)*]
            }
        }
    ) => {
        html_arr! {{
            {
                {
                    $($rest)*
                }
                [$($expanded)*
                    {
                        use std::collections::BTreeMap;

                        {
                            let elem_str = stringify!($tag);

                            if elem_str.starts_with("\"") {
                                Element::Text(elem_str.to_string())
                            } else {
                                #[allow(unused_mut, unused_assignments)]
                                let mut children = vec![];

                                children = html_arr!($($e)*);

                                #[allow(unused_mut, unused_assignments)]
                                let mut attrs = BTreeMap::new();

                                $(
                                    attrs = [$((stringify!($attr_name).to_string(), $attr_value.to_string())),*].into();
                                )?

                                #[allow(unused_mut, unused_assignments)]
                                let mut evcode = BTreeMap::new();

                                $(
                                    evcode = [$((stringify!($ev).into(), $evcode)),*].into();
                                )?

                                Element::Node {
                                    tag: elem_str.to_string(),
                                    attrs,
                                    events: evcode,
                                    children: children,
                                }
                            }
                        }
                    }
                ]
            }
        }}
    };

    // Text
    (
        {
            {
                {
                    {{ $code:expr }}
                    $($rest:tt)*
                }
                [$($expanded:tt)*]
            }
        }
    ) => {
        html_arr! {{
            {
                {
                    $($rest)*
                }
                [$($expanded)*
                    {
                        Element::Text($code.to_string())
                    }
                ]
            }
        }}
    };

    // Component
    (
        {
            {
                {
                    @$comp:tt,
                    $($rest:tt)*
                }
                [$($expanded:tt)*]
            }
        }
    ) => {
        html_arr! {{
            {
                {
                    $($rest)*
                }
                [$($expanded)*
                    {
                        // use std::{cell::RefCell, rc::Rc};
                        Element::Component($comp)
                    }
                ]
            }
        }}
    };


    // Empty rule, to handle the case where there is no children
    () => {
        vec![]
    };

    // Final case, where we return the vec with all the elements
    (
        {
            {
                {}
                [$($expanded:tt)*]
            }
        }
    ) => {
        vec![$($expanded),*]
    };

    // Entry point, base rule
    // This is defined last, else it causes an infinite recursion as it matches with itself right away
    (
        $( $e:tt )*
    ) => {
        html_arr! {{
            {
                {
                    $( $e )*
                }
                []
            }
        }}
    };
}

// Conveinience macro to get the root element of the defined dom
#[macro_export]
macro_rules! html {
    (
        $( $e:tt )*
    ) => {
        {
            let mut arr = html_arr! {
                $($e)*
            };

            if arr.len() != 1 {
                panic!("The html macro must have exactly one root element");
            }

            arr.pop().unwrap()
        }
    };
}

#[cfg(target_arch = "wasm32")]
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
}
