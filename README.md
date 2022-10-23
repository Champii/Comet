# Comet

Reactive isomorphic rust web framework.

Work in progress.

## Quick start

    - Install Comet binary
    - Create simple counter example
    - Run it

Visit the [example](https://github.com/Champii/Comet/tree/master/examples) folder, this is the only documentation for now

### Install Comet Binary

```bash
$> cargo install --git https://github.com/Champii/Comet --locked
```

You will need [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

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
	{ self }
    }
}

// We run the application with a start value
comet!(0);
```

### Run it

```bash
$> comet run
```

If you prefer to compile and run it manually:

```bash
# Build the client
$> wasm-pack build --target web

# Build and run the app
$> cargo run
```

And go to [http://localhost:8080](http://localhost:8080)

## Quick tour

### Easy definition of the dom

```rust
struct MyStruct {
    my_value: String,
    my_height: u32,
}

component! {
    MyStruct,
    // Here #my_id defined the id,
    // and the dot .class1 and .class2 add some classes to the element
    // The #id must always preceed the classes, if any
    div #my_id.class1.class2 {
	span {
	    // You can access your context anywhere
	    { self.my_value }
	}
	// Define style properties
	div [height: { self.my_height }}] {
	    { "Another child" }
	}
    }
};

```

### Use conditional rendering and loops directly from within the view

```rust
struct MyComponent {
    show: bool,
    value: HashMap<String, i32>,
}

component! {
    MyComponent,
    div {
	div {
	    // Conditional rendering with if
	    // The parenthesis are necessary
	    if (self.show) {
		{ "Visible !" }
	    }
	    button @click: { self.show = !self.show } {
		{ "Toggle" }
	    }
	}
	div {
	    // Use a for-like loop.
	    // The parenthesis are necessary around the last part
	    for key, value in (self.value) {
		div {
		    { key }
		    { value }
		}
	    }
	    button @click: { self.value.push(42) } {
		{ "Add a number" }
	    }
	}
    }
}
```

### Bind you variables to `input` fields that react to events

This is exclusive to `input` fields for now  
The whole component is re-rendered on input's blur event (unfocus).  
Each binding should be unique, as in a different variable for each one

```rust
struct MyStruct {
    value: String,
}

component! {
    MyStruct,
    div {
	input ={ self.value } {}
	{ self.value }
    }
}
```

### Embed your components between them

```rust
struct Child {
    value: String,
}

component! {
    Child,
    div {
	{ self.value }
    }
}

struct Parent {
    // You need to wrap your components with a Shared<T> that is basically a Rc<RefCell<T>>
    // This is necessary for your states to persist and be available between each render
    child: Shared<Child>,
}

component! {
    Parent,
    div {
	// To include a component, you must wrap it inside a @{ }
	@{ self.child }
    }
}
```

## TODO List
- DB
    - Macro for models
    - Register for queries
- Websocket
- Allow for iterators inside html
- Allow to mix attributes, styles and events
- Find a way for global inter-component message passing
- Allow for real time value binding for input element without losing focus (might need a real virtual dom for this one)

