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
                    @$comp:tt
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
                        $comp.view()
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

    // base rule
    // This is last, else it causes an infinite recursion as it matches with itself right away
    (
        $( $e:tt )*
    ) => {
        html_arr! {{
            {{ $( $e )* }[]}
        }}
    };


}

#[macro_export]
macro_rules! html {
    ($($e:tt)*) => {
        html_arr! {
            $($e)*
        }.pop().unwrap()
    };
}

#[cfg(test)]
mod html_test {
    use crate::{element, prelude::*, renderable};

    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);

    #[derive(Clone)]
    struct Msg;

    #[wasm_bindgen_test]
    fn test_html() {
        let view = html! {
            div {
                button {
                    {{ "Increment" }}
                }
            }
        };

        let elem =
            <element::Element<Msg> as renderable::Renderable<Msg>>::render::<_>(&view, |_| {});

        assert_eq!(elem.outer_html(), "<div><button>Increment</button></div>");
    }
}
