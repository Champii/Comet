#[macro_export]
macro_rules! html_arr {
    // tag
    (
        $self:ident,
        {
            {
                {
                    $tag:ident
                        $([$($attr_name:ident : $attr_value:expr),*])?
                        $($(@$ev:ident : self.$evcode:ident() ),+, )?
                        { $($e:tt)* }

                    $($rest:tt)*
                }
                [$($expanded:tt)*]
            }
        }
    ) => {
        html_arr! {$self, {
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

                                children = html_arr!($self, $($e)*);

                                #[allow(unused_mut, unused_assignments)]
                                let mut attrs = BTreeMap::new();

                                $(
                                    attrs = [$((stringify!($attr_name).to_string(), $attr_value.to_string())),*].into();
                                )?

                                #[allow(unused_mut, unused_assignments)]
                                let mut evcode = BTreeMap::new();

                                $(
                                    evcode = [($(stringify!($ev).into(), Msg::$ev),*)].into();

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
        $self:ident,
        {
            {
                {
                    {{ self.$code:ident }}
                    $($rest:tt)*
                }
                [$($expanded:tt)*]
            }
        }
    ) => {
        html_arr! {$self, {
            {
                {
                    $($rest)*
                }
                [$($expanded)*
                    {
                        Element::Text($self.$code.to_string())
                    }
                ]
            }
        }}
    };

    // Component
    (
        $self:ident,
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
        html_arr! {$self, {
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
        $self:ident,
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
        $self:ident,
        $( $e:tt )*
    ) => {
        html_arr! {$self, {
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
        $self:ident,
        $( $e:tt )*
    ) => {
        {
            let mut arr = html_arr! {
                $self,
                $($e)*
            };

            if arr.len() != 1 {
                panic!("The html macro must have exactly one root element");
            }

            arr.pop().unwrap()
        }
    };
}

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
#[macro_export]
macro_rules! extract_msg {
    // tag
    (
        {
            {
                {
                    $tag:ident
                        $([$($attr_name:ident : $attr_value:expr),*])?
                        $($(@$ev:ident : self.$evcode:ident() ),+,)?
                        { $($e:tt)* }

                    $($rest:tt)*
                }
                [$($expanded:tt)*]
            }
        }
    ) => {
        extract_msg! {{
            {
                {
                    $($rest)*
                    $($e)*
                }
                [$($expanded)*
                            $($({
                                $ev
                            })+)?
                ]
            }
        }}
    };

    // Text
    (
        {
            {
                {
                    {{ self.$code:ident }}
                    $($rest:tt)*
                }
                [$($expanded:tt)*]
            }
        }
    ) => {
        extract_msg! {{
            {
                {
                    $($rest)*
                }
                [$($expanded)*]
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
        extract_msg! {{
            {
                {
                    $($rest)*
                }
                [$($expanded)*]
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
                [$({$name:ident})*]
            }
        }
    ) => {
        #[derive(Clone)]
        pub enum Msg {
            $(
                $name
            ),*
        }

        /* fn update(&mut self, msg: Msg) {
            match msg {
                $(
                    Msg::$expanded => {
                        $code
                    }
                ),*
            }
        } */
    };

    // Entry point, base rule
    // This is defined last, else it causes an infinite recursion as it matches with itself right away
    (
        $( $e:tt )*
    ) => {
        extract_msg! {{
            {
                {
                    $( $e )*
                }
                []
            }
        }}
    };
}

#[macro_export]
macro_rules! extract_update {
    // tag
    (
        $self:ident,
        $msg:ident,
        $type:ty,
        {
            {
                {
                    $tag:ident
                        $([$($attr_name:ident : $attr_value:expr),*])?
                        $($(@$ev:ident : self.$evcode:ident() ),+,)?
                        { $($e:tt)* }

                    $($rest:tt)*
                }
                [$($expanded:tt)*]
            }
        }
    ) => {
        extract_update! {$self, $msg, $type, {
            {
                {
                    $($rest)*
                    $($e)*
                }
                [$($expanded)*
                    $($(
                        {
                            $ev, $self.$evcode()
                        }
                    )*)?
                ]
            }
        }}
    };

    // Text
    (
        $self:ident,
        $msg:ident,
        $type:ty,
        {
            {
                {
                    {{ self.$code:ident }}
                    $($rest:tt)*
                }
                [$($expanded:tt)*]
            }
        }
    ) => {
        extract_update! {$self, $msg, $type, {
            {
                {
                    $($rest)*
                }
                [$($expanded)*]
            }
        }}
    };

    // Component
    (
        $self:ident,
        $msg:ident,
        $type:ty,
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
        extract_update! {$self, $msg, $type, {
            {
                {
                    $($rest)*
                }
                [$($expanded)*]
            }
        }}
    };


    // Empty rule, to handle the case where there is no children
    () => {
        vec![]
    };

    // Final case, where we return the vec with all the elements
    (
        $self:ident,
        $msg:ident,
        $type:ty,
        {
            {
                {}
                [$({ $name:ident,  $code:expr })*]
            }
        }
    ) => {
        /* #[derive(Clone)]
        enum Msg {
            $(
                $name
            ),*
        } */

                match $msg {
                    $(
                        Msg::$name =>{
                            $code
                        }
                    ),*
            }

    };

    // Entry point, base rule
    // This is defined last, else it causes an infinite recursion as it matches with itself right away
    (
        $self:ident,
        $msg:ident,
        $type:ty,
        $( $e:tt )*
    ) => {
        extract_update! {$self, $msg, $type, {
            {
                {
                    $( $e )*
                }
                []
            }
        }}
    };
}

#[macro_export]
macro_rules! component {
    ($type:ty, $($e:tt)+) => {
        extract_msg!{$($e)+}

        impl Component<Msg> for $type {
            fn update(&mut self, msg: Msg) {
                extract_update!{self, msg, $type, $($e)+}
            }

            fn view(&self) -> Element<Msg> {
                html! {self, $($e)+ }
            }
        }
    };
}

mod lol {
    use crate::prelude::*;

    pub struct Counter {
        value: i32,
    }

    impl Counter {
        pub fn new() -> Self {
            Self { value: 0 }
        }

        pub fn increment(&mut self) {
            self.value += 1;
        }

    }

    component! { Counter,
        button @click: self.increment(), {
            {{ self.value }}
        }
    }

    // comet!(Counter);
}
