# Comet

Reactive isomorphic rust web framework.

Work in progress.

## Quick start

    - Install Comet binary
    - Create simple counter example
    - Run it

### Install Comet Binary

```bash
$> cargo install https://github.com/Champii/Comet --locked
```

### Create simple counter example

`src/lib.rs`

```rust
#[derive(Clone)]
pub enum Msg {
    Increment,
}

#[derive(Default)]
pub struct Counter {
    pub value: i32,
}

impl Component<Msg> for Counter {
    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Increment => self.value += 1,
        }
    }

    fn view(&self) -> Element<Msg> {
        html! {
            div {
                button @click: Msg::Increment, {
                    {{ self.value }}
                }
            }
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    comet::run(Counter::default());
}
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
