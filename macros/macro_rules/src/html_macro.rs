#[macro_export]
macro_rules! html_arr {
    // if
    (
        $self:ident,
        $f:ident,
        $bindings:ident,
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
        html_arr! {$self, $f, $bindings, {
            {
                {
                    $($rest)*
                }
                [$($expanded)*
                    {
                    if
                        replace_self!($self, $($predicate)*)
                        { html! { $self, $f, $bindings, $($e)* } }
                    else
                        { html! { $self, $f, $bindings, {""}} }
                    }
                ]
            }
        }}
    };
    // for
    (
        $self:ident,
        $f:ident,
        $bindings:ident,
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
        html_arr! {$self, $f, $bindings, {
            {
                {
                    $($rest)*
                }
                [$($expanded)*
                    {
                        let elem = document().create_element("span").unwrap();

                        for ($($predicate),*) in replace_self!($self, $($iter)*) {
                            elem.append_child(&html! { $self, $f, $bindings, $($e)* }.into_element()).unwrap();
                        }

                        HtmlNode::Element(elem)
                    }
                ]
            }
        }}
    };
    // tag
    (
        $self:ident,
        $f:ident,
        $bindings:ident,
        {
            {
                {
                    $tag:ident $(#$id_name:ident)? $(.$class_name:ident)*
                        $([$($attr_name:ident : {$($attr_value:tt)*} ),*])?
                        $($(@$ev:ident : {$($evcode:tt)*} ),+ )?
                        $(={ $($binding:tt)* })?
                        { $($e:tt)* }

                    $($rest:tt)*
                }
                [$($expanded:tt)*]
            }
        }
    ) => {
        html_arr! {$self, $f, $bindings, {
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

                        let mut binding: Option<String> = None;
                        $(
                            let binding = Some(replace_self!($self, $($binding)*).to_string());
                        )?

                        let elem = create_element(
                            $f.clone(),
                            stringify!($tag),
                            id_name,
                            vec![$(stringify!($class_name)),*],
                            [$($((stringify!($attr_name).to_string(), replace_self!($self, $($attr_value)*).to_string())),*)?].into(),
                            [$(($(stringify!($ev).into(),
                               gen_full_variant!($($evcode)*)
                            ),+))?].into(),
                            binding.clone(),
                        );

                        if binding.is_some() {

        #[cfg(target_arch = "wasm32")]
        crate::console_log!("BEFORE WRITE2");
                            $bindings.blocking_write().push(elem.clone());
        #[cfg(target_arch = "wasm32")]
        crate::console_log!("AFTER WRITE2");
                        }

                        let children: Vec<HtmlNode> = html_arr! {$self, $f, $bindings, $($e)*};

                        for child in children {
                            child.append_to(&elem);
                        }

                        HtmlNode::Element(elem)
                    }
                ]
            }
        }}
    };

    // Component
    (
        $self:ident,
        $f:ident,
        $bindings:ident,
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
        html_arr! {$self, $f, $bindings, {
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


                        let component_container2 = component_container.clone();
                        spawn_local(async move {
                            comet::core::component::run_rec(component, &component_container2).await;
                        });

                        HtmlNode::Element(component_container)
                    }
                ]
            }
        }}
    };

    // Text
    (
        $self:ident,
        $f:ident,
        $bindings:ident,
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
        html_arr! {$self, $f, $bindings, {
            {
                {
                    $($rest)*
                }
                [$($expanded)*
                    {
                        let res = &replace_self!(
                            $self,
                            $($code)+
                        ).to_string();

                        HtmlNode::Text(document().create_text_node(res))
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
        $bindings:ident,
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
        $bindings:ident,
        $( $e:tt )*
    ) => {
        html_arr! {$self, $f, $bindings, {
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
        $bindings:ident,
        $( $e:tt )*
    ) => {
        {
            let mut arr = html_arr! {
                $self,
                $f,
                $bindings,
                $($e)*
            };

            if arr.len() != 1 {
                panic!("The html macro must have exactly one root element");
            }

            arr.pop().unwrap()
        }
    };
}
