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

And go to [http://localhost:8080](http://localhost:8080)

## Quick tour

### Easy definition of the dom

```rust
struct MyStruct {
    my_value: String,
    my_height: u32,
}

component! {
    i32,
    // Here #my_id defined the id,
    // and the dot .class1 and .class2 add some classes to the element
    // The #id must always preceeds the classes, if any
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

### Define actions directly from within the view

```rust
struct MyComponent {
    value: Vec<i32>,
}

component! {
    MyComponent,
    div {
	// Use a for-like loop
	for ((value) in self.value) {
	    div {
		{ value }
	    }
	}
	// Setup your actions
	button @click: { self.value.push(42) } {
	    { "Add a number" }
	}
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
    // This is necessary if you want your states to persist and still be available
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
- Websocket
- Register for queries
- Allow for `if`, `for` and iterators inside html
- Allow intermix of attributes, styles and events

