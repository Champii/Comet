#[macro_export]
macro_rules! extract_bindings {
    // if
    (
        $self:ident,
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
        extract_bindings! {$self, $bindings, {
            {
                {
                    $($rest)*
                    $($e)*
                }
                [$($expanded)*
                ]
            }
        }}
    };

    // for
    (
        $self:ident,
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
        extract_bindings! {$self, $bindings, {
            {
                {
                    $($rest)*
                    $($e)*
                }
                [$($expanded)*
                ]
            }
        }}
    };
    // tag
    (
        $self:ident,
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
        extract_bindings! {$self, $bindings, {
            {
                {
                    $($rest)*
                    $($e)*
                }
                [$($expanded)*
                    $(
                        {
                            replace_self!($self, $($binding)*)
                        }
                    )?
                ]
            }
        }}
    };
    // Component
    (
        $self:ident,
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
        extract_bindings! {$self, $bindings, {
            {
                {
                    $($rest)*
                }
                [$($expanded)*]
            }
        }}
    };


    // Text
    (
        $self:ident,
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
        extract_bindings! {$self, $bindings, {
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
        $bindings:ident,
        {
            {
                {}
                [$({ $($binding:tt)* })*]
            }
        }
    ) => {
        use wasm_bindgen::JsCast;
        use web_sys::HtmlInputElement;
        let mut bindings = $bindings.blocking_write().clone();
        bindings.reverse();
        $(
            let elem = bindings.pop().unwrap();
            let input_elem: HtmlInputElement = elem.dyn_into().unwrap();
            $($binding)* = input_elem.value();
        )*
    };

    // Entry point, base rule
    // This is defined last, else it causes an infinite recursion as it matches with itself right away
    (
        $self:ident,
        $bindings:ident,
        $( $e:tt )*
    ) => {
        extract_bindings! {$self, $bindings, {
            {
                {
                    $( $e )*
                }
                []
            }
        }}
    };
}
