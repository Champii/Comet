# Comet

Reactive isomorphic rust web framework.

## Quick start

    - Install Comet binary
    - Create simple counter example
    - Run it

### Install Comet Binary

```bash
$> cargo install https://github.com/Champii/Comet2 --locked
```

### Create simple counter example

`src/lib.rs`

```rust
#[derive(Clone)]
pub enum Msg {
    Increment,
}

pub struct Counter {
    pub value: i32,
}

impl Counter {
    pub fn new() -> Self {
        Self { value: 0 }
    }
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
                button
                    @click: Msg::Increment, {
                    {{ self.value }}
                },
            }
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    comet::run(Counter::new());
}
```

### Run it

```bash
$> comet run
```

And go to [localhost:8080](http://localhost:8080)

## TODO List
- Server
    - DB
	- Macro for models
	    - 
    - Websocket
- Client
    - Nested components
	- Need to have a component tree

