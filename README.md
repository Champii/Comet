# Comet

Reactive isomorphic rust web framework.

Work in progress.

## Quick start

    - Install Comet binary
    - Create simple counter example
    - Run it

### Install Comet Binary

```bash
$> cargo install --git https://github.com/Champii/Comet --locked
```

### Create simple counter example

```bash
$> comet new my_counter && cd my_counter
```

The default generated file `src/lib.rs` :

```rust
// The mandatory imports
use comet::prelude::*;

// We create a component that is an `i32` and which we can increment with a button
component! {
    i32,
    button @click: { *self += 1 } {
	{{ self }}
    }
}

// We run the application with a start value
comet!(0);
```

### Run it

```bash
$> comet run
```

And go to [http://localhost:8080](http://localhost:8080)

## TODO List
- DB
    - Macro for models
- Websocket
- Register for queries
- Allow for `if`, `for` and iterators inside html

