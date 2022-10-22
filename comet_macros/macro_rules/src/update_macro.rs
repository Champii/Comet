#[macro_export]
macro_rules! extract_update {
    // if
    (
        $self:ident,
        $msg:ident,
        $type:ty,
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
        extract_update! {$self, $msg, $type, {
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

    // TODO: For is not handled for now, as it would produce some identical events
    //       See #1

    // tag
    (
        $self:ident,
        $msg:ident,
        $type:ty,
        {
            {
                {
                    $tag:ident
                        $([$($attr_name:ident : {$($attr_value:tt)*} ),*])?
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
                    { $($code:tt)* }
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
                comet_macro_procs::generate_update! {
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
