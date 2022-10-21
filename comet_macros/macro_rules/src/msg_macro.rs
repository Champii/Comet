#[macro_export]
macro_rules! extract_msg {
    // tag
    (
        {
            {
                {
                    $tag:ident
                        $([$($attr_name:ident : {$($attr_value:tt)*} ),*])?
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
                    { $($code:tt)* }
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
