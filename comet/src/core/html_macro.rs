#[macro_export]
macro_rules! gen_full_variant {
    ($($a:tt)*) => {
        comet_macros::gensym!{ _gen_full_variant!{ $($a)* } }
    };
}

#[macro_export]
macro_rules! _gen_full_variant {
    ($gensym:ident, $($a:tt)*) => {
        Msg::$gensym
    };
}

#[macro_export]
macro_rules! gen_variant {
    ($($a:tt)*) => {
        comet_macros::gensym!{ _gen_variant!{ $($a)* } }
    };
}

#[macro_export]
macro_rules! _gen_variant {
    ($gensym:ident, $($a:tt)*) => {
        $gensym
    };
}

#[macro_export]
macro_rules! replace_self {
    // Actually replace any self in the token stream
    (
        $self:ident,
        {
            {
                {
                    self
                    $($rest:tt)*
                }
                [$($expanded:tt)*]
            }
        }
    ) => {
        replace_self! {$self, {
            {
                {
                    $($rest)*
                }
                [$($expanded)*
                    $self
                ]
            }
        }}
    };

    // Do nothing if there is no self
    (
        $self:ident,
        {
            {
                {
                    $chunk:tt
                    $($rest:tt)*
                }
                [$($expanded:tt)*]
            }
        }
    ) => {
        replace_self! {$self, {
            {
                {
                    $($rest)*
                }
                [$($expanded)*
                    $chunk
                ]
            }
        }}
    };

    // Result
    (
        $self:ident,
        {
            {
                {}
                [$($expanded:tt)*]
            }
        }
    ) => {
        $($expanded)*
    };

    // Entry point
    (
        $self:ident,
        $($e:tt)*
    ) => {
        replace_self! {$self, {
            {{ $($e)* }[]}
        }}
    };
}

#[macro_export]
macro_rules! new_ident {
    ($before:ident,  $ident:expr) => {
        Msg::hash!($ident)
    };
}

#[macro_export]
macro_rules! html_arr {
    // tag
    (
        $self:ident,
        $f:ident,
        $id:expr,
        {
            {
                {
                    $tag:ident
                        $([$($attr_name:ident : $attr_value:expr),*])?
                        $($(@$ev:ident : {$($evcode:tt)*} ),+ )?
                        { $($e:tt)* }

                    $($rest:tt)*
                }
                [$($expanded:tt)*]
            }
        }
    ) => {
        html_arr! {$self, $f, $id + 1, {
            {
                {
                    $($rest)*
                }
                [$($expanded)*
                    {
                        use std::collections::BTreeMap;
                        use wasm_bindgen::JsCast;

                        {
                            let window = web_sys::window().expect("no global `window` exists");
                            let document = window.document().expect("should have a document on window");

                            let elem_str = stringify!($tag);

                            let elem = document.create_element(elem_str).unwrap();
                            if elem_str.starts_with("\"") {
                                elem.set_inner_html(elem_str);

                                elem
                            } else {

                                #[allow(unused_mut, unused_assignments)]

                                let children = html_arr!($self, $f, $id + 10000, $($e)*);
                                 for child in children {
                                    elem.append_child(
                                        &child
                                    )
                                    .unwrap();
                                };
                                #[allow(unused_mut, unused_assignments)]
                                let mut attrs: BTreeMap<String, String> = BTreeMap::new();

                                $(
                                    attrs = [$((stringify!($attr_name).to_string(), $attr_value.to_string())),*].into();
                                )?

                                for (attr_name, value) in attrs {
                                    elem.set_attribute(attr_name.as_str(), value.as_str()).unwrap();
                                }

                                #[allow(unused_mut, unused_assignments)]
                                let mut evcode: BTreeMap<String, Msg> = BTreeMap::new();

                                $(
                                        evcode = [($(stringify!($ev).into(),
                                           gen_full_variant!($($evcode)*)
                                        ),+)].into();
                                )?

                                if let Some(event) = evcode.get("click") {
                                    let f = $f.clone();
                                    let event = event.clone();

                                    let closure = Closure::<dyn Fn()>::wrap(Box::new(move || {
                                        f(event.clone());
                                    }));

                                    elem.dyn_ref::<web_sys::HtmlElement>()
                                        .expect("#should be an `HtmlElement`")
                                        .set_onclick(Some(closure.as_ref().unchecked_ref()));

                                    // FIXME: leak
                                    closure.forget();
                                }

                                elem
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
        $f:ident,
        $id:expr,
        {
            {
                {
                    {{ $($code:tt)* }}
                    $($rest:tt)*
                }
                [$($expanded:tt)*]
            }
        }
    ) => {
        html_arr! {$self, $f, $id, {
            {
                {
                    $($rest)*
                }
                [$($expanded)*
                    {
                        let window = web_sys::window().expect("no global `window` exists");
                        let document = window.document().expect("should have a document on window");

                        let elem = document.create_element("span").unwrap();

                        elem.set_inner_html(&replace_self!(
                            $self,
                            $($code)*
                        ).to_string());

                        elem
                    }
                ]
            }
        }}
    };

    // Component
    (
        $self:ident,
        $f:ident,
        $id:expr,
        {
            {
                {
                    @{$($comp:tt)+}
                    $($rest:tt)*
                }
                [$($expanded:tt)*]
            }
        }
    ) => {
        html_arr! {$self, $f, $id, {
            {
                {
                    $($rest)*
                }
                [$($expanded)*
                    {
                        let window = web_sys::window().expect("no global `window` exists");
                        let document = window.document().expect("should have a document on window");

                        let component_container = document.create_element("span").unwrap();

                        let component = replace_self!(
                            $self,
                            $($comp)+
                        ).clone();

                        comet::core::component::run_rec(component, &component_container);

                        component_container
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
        $f:ident,
        $id:expr,
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
        $f:ident,
        $id:expr,
        $( $e:tt )*
    ) => {
        html_arr! {$self, $f, $id, {
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
        $f:ident,
        $( $e:tt )*
    ) => {
        {
            let mut arr = html_arr! {
                $self,
                $f,
                0,
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
                        $($(@$ev:ident : {$($evcode:tt)*} ),+ )?
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
                        $($evcode)*
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
                    {{ $($code:tt)* }}
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
                    @{$($comp:tt)+}
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
                [$({$($name:tt)*})*]
            }
        }
    ) => {
        comet_macros::generate_msg! {
            [$(
                $($name)*
            ),*]
        }
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
                        $($(@$ev:ident : {$($evcode:tt)*}  ),+ )?
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
                            {$( $evcode )*} , replace_self!($self, $($evcode)*)
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
                    {{ $($code:tt)* }}
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
                    @{$($comp:tt)+}
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
                [$({ {$($name:tt)*},  $($code:tt)* })*]
            }
        }
    ) => {
        match $msg {
            $(
                comet_macros::generate_update! {
                        $($name)*
                } => {
                    $($code)*
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
        paste! {
            mod [<__component_ $type:lower>] {
                use super::*;

                extract_msg!{$($e)+}

                impl Component<Msg> for $type {
                    fn update(&mut self, msg: Msg) {
                        extract_update!{self, msg, $type, $($e)+}
                    }

                    fn view<F>(&self, f: F) -> web_sys::Element where F: Fn(Msg) + Clone + 'static {
                        html! {self, f, $($e)+ }
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! comet {
    ($($e:tt)+) => {
        #[wasm_bindgen(start)]
        pub fn main() {
            comet::run($($e)+);
        }
    }
}

mod lol {
    use crate::prelude::*;

    pub struct Counter {
        value: i32,
    }

    component! { Counter,
        button @click: {self.value += 1} {
            {{ self.value }}
        }
    }
}
