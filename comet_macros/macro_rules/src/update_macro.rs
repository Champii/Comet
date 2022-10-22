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

    // for
    (
        $self:ident,
        $msg:ident,
        $type:ty,
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
    // tag
    (
        $self:ident,
        $msg:ident,
        $type:ty,
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
        extract_update! {$self, $msg, $type, {
            {
                {
                    $($rest)*
                    $($e)*
                }
                [$($expanded)*
                    $($(
                        {
                            {$( $evcode )*} , {replace_self!($self, $($evcode)*)}
                        }
                    )*)?
                ]
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
                $({$($binding:tt)*})?
            }
        }
    ) => {
        match $msg {
            $(
                comet_macro_procs::generate_update! {
                        $($name)*
                } => {
                    $($code)*;
                }
            ),*
            $(
                    $($binding)* = value;
            )?
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
