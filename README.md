# Comet

Reactive isomorphic rust web framework.

Work in progress.

## Quick start

    - Install Comet binary
    - Create simple counter example
    - Run it

### Install Comet Binary

```bash
$> cargo install https://github.com/Champii/Comet/tree/comet_bin --locked
```

### Create simple counter example

```toml
[dependencies]
comet = { git = "https://github.com/Champii/Comet/tree/comet" }
```

`src/lib.rs`

```rust
use comet::prelude::*;

#[derive(Default)]
pub struct Counter {
    pub value: i32,
}

component! { Counter,
    button @click: { self.value += 1 }, {
	{{ self.value }}
    }
}

comet!(Counter::default());
```

### Run it

```bash
$> comet run
```

And go to [localhost:8080](http://localhost:8080)

## TODO List
- DB
    - Macro for models
- Websocket
- Nested components
    - Need to have a component tree
- Register for queries
- Allow for `if`, `for` and iterators inside html
