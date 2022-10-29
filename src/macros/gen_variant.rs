#[macro_export]
macro_rules! gen_full_variant {
    ($($a:tt)*) => {
        comet_macro_procs::generate_hash!{ _gen_full_variant!{ $($a)* } }
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
