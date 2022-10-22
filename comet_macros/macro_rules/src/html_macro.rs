#[macro_export]
macro_rules! html_arr {
    // if
    (
        $self:ident,
        $f:ident,
        {
            {
                {
                    if
                        ($($predicate:tt)*)
                        { $($e:tt)* }

                    $($rest:tt)*
                }
                [$($expanded:tt)*]
            }
        }
    ) => {
        html_arr! {$self, $f, {
            {
                {
                    $($rest)*
                }
                [$($expanded)*
                    {
                    if
                        replace_self!($self, $($predicate)*)
                        { html! { $self, $f, $($e)* } }
                    else
                        { html! { $self, $f, {""}} }
                    }
                ]
            }
        }}
    };
    // for
    (
        $self:ident,
        $f:ident,
        {
            {
                {
                    for
                        $($predicate:ident),+ in ($($iter:tt)*)
                        { $($e:tt)* }

                    $($rest:tt)*
                }
                [$($expanded:tt)*]
            }
        }
    ) => {
        html_arr! {$self, $f, {
            {
                {
                    $($rest)*
                }
                [$($expanded)*
                    {

                        let window = web_sys::window().expect("no global `window` exists");
                        let document = window.document().expect("should have a document on window");

                        let elem = document.create_element("span").unwrap();

                        for ($($predicate),*) in replace_self!($self, $($iter)*) {
                            elem.append_child(&html! { $self, $f, $($e)* }).unwrap();
                        }

                        elem
                    }
                ]
            }
        }}
    };
    // tag
    (
        $self:ident,
        $f:ident,
        {
            {
                {
                    $tag:ident $(#$id_name:ident)? $(.$class_name:ident)*
                        $([$($attr_name:ident : {$($attr_value:tt)*} ),*])?
                        $($(@$ev:ident : {$($evcode:tt)*} ),+ )?
                        { $($e:tt)* }

                    $($rest:tt)*
                }
                [$($expanded:tt)*]
            }
        }
    ) => {
        html_arr! {$self, $f, {
            {
                {
                    $($rest)*
                }
                [$($expanded)*
                    {

                        let mut id_name: Option<&str> = None;
                        $(
                            let id_name = Some(stringify!($id_name));
                        )?

                        let elem = crate::core::create_element(
                            $f.clone(),
                            stringify!($tag),
                            id_name,
                            vec![$(stringify!($class_name)),*],
                            [$($((stringify!($attr_name).to_string(), replace_self!($self, $($attr_value)*).to_string())),*)?].into(),
                            [$(($(stringify!($ev).into(),
                               gen_full_variant!($($evcode)*)
                            ),+))?].into(),
                        );

                        let children = html_arr! {$self, $f, $($e)*};

                        for child in children {
                            elem.append_child(&child).unwrap();
                        }

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
        html_arr! {$self, $f, {
            {
                {
                    $($rest)*
                }
                [$($expanded)*
                    {
                        let component_container = document().create_element("span").unwrap();

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

    // Text
    (
        $self:ident,
        $f:ident,
        {
            {
                {
                    { $($code:tt)* }
                    $($rest:tt)*
                }
                [$($expanded:tt)*]
            }
        }
    ) => {
        html_arr! {$self, $f, {
            {
                {
                    $($rest)*
                }
                [$($expanded)*
                    {
                        let window = web_sys::window().expect("no global `window` exists");
                        let document = window.document().expect("should have a document on window");

                        let res = &replace_self!(
                            $self,
                            $($code)+
                        ).to_string();

                        let text_node = document.create_text_node(res);

                        text_node
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
        $( $e:tt )*
    ) => {
        html_arr! {$self, $f, {
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
                $($e)*
            };

            if arr.len() != 1 {
                panic!("The html macro must have exactly one root element");
            }

            arr.pop().unwrap().into()
        }
    };
}
