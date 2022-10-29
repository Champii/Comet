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
