[![Crates.io](https://img.shields.io/crates/v/gensym.svg)](https://crates.io/crates/gensym)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/gensym/0.1.0/gensym/)

# gensym

Creates unique identifiers for macros using procedural macros and [UUID](https://crates.io/crates/uuid)
## Examples
```rust

macro_rules! gen_fn {
    ($a:ty, $b:ty) => {
        gensym::gensym!{ _gen_fn!{ $a, $b } }
    };
}

macro_rules! _gen_fn {
    ($gensym:ident, $a:ty, $b:ty) => {
        fn $gensym(a: $a, b: $b) {
            unimplemented!()
        }
    };
}

mod test {
    gen_fn!{ u64, u64 }
    gen_fn!{ u64, u64 }
}
```
