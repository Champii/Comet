#[macro_export]
macro_rules! extract_msg {
    // if
    (
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
        extract_msg! {{
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
        extract_msg! {{
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

    // Text
    (
        {
            {
                {
                    { $($code:tt)+ }
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
        comet_macro_procs::generate_msg! {
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
