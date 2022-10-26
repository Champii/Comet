# Comet

Reactive isomorphic rust web framework.

## Index

  1. [Introduction](#introduction)
  2. [Getting started](#getting-started)
  3. [Quick tour](#quick-tour)
  4. [Todo list](#todo-list)

## Introduction

Work in progress.

Comet is a framework for the web build with Rust+Wasm<3. It takes its inspiration from MeteorJS, Seed-rs, Yew and others.

This crate aims to be an all-in-one all-inclusive battery-included isomorphic reactive framework.

      - You keep saying 'Isomorphic', but why ?

In this context, Isomorphic means that you only write one program for both client and server.  
One crate. One. For both. Yes.  
This means that we rely a lot on macros and code generation, with all the good and the bad this could bring,
but it allows for a great deal of features, close to no boilerplate, and a little quality of life improvement on different aspects.

      - Ok, and how is it reactive then ?

It is reactive in many sense, first by its `component` system, that encapsulate little bits of logic into an HTML templating system,
and which can bind your struct's methods directly to JS events, triggering a render of only the components that changed. 
There is also a reactive layer on top of a `PostgreSQL` database, that permits to watch for some queries to change over time and 
to send push notifications over websocket to every client watching for thoses change, triggering a render when needed.

Visit the [examples](https://github.com/Champii/Comet/tree/master/examples) folder.

## Getting started

### Install Comet Binary and dependencies

```bash
$> cargo install --git https://github.com/Champii/Comet --locked
```

You will need to install and run an instance of PostgreSQL.

If not found on your system, Comet will install these following crates using `cargo install` on the first run:
 - `wasm-pack`
 - `diesel-cli`

### Create a simple incrementing counter 

```bash
$> comet new my_counter && cd my_counter
```

This newly generated project contains all you need to get started. The only file you have to care about for now is `src/lib.rs`, this is your entry point.  
Conveignantly, the generated file is already the simpliest incrementing counter you can think of.

The default generated file `src/lib.rs` :

```rust
// The mandatory imports
use comet::prelude::*;

// This macro takes two arguments:
// - A type for which we will implement `Component`
// - And a root HTML element
// We implement `Component` for a simple integer.
component! {
    // We use an i32 here, but you can use any stucts/enums/custom type
    i32,

    // The root of this HTML element is a simple button
    // It has a 'click' event registered that will increment our i32 by 1
    button @click: { *self += 1 } {
        // We display our value inside the button
        { self }
    }
}

// This is where all the magic happens
// We run the application with an instance of our i32 component that starts with the value 0
comet!(0);
```

### Run it

Setup your database address as an env variable

/!\ Warning: This database will be COMPLETELY WIPED at startup and everytime your models change  
This is not ideal but, hey ! This is still a work in progress :p

```bash
$> export DATABASE_URL="postgres://your_user:your_password@localhost/your_db"
```

Actually run your project

```bash
$> comet run
```

This will download and install the tools it needs to build and run your crate.

```bash
[✓] Installing wasm-pack
[✓] Installing diesel-cli
[✓] Diesel setup
[✓] Migrating database
[✓] Patching schema
[✓] Building client
[✓] Building server
[✓] Running
 -> Listening on 0.0.0.0:8080
```

Then go go to [http://localhost:8080](http://localhost:8080)

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

### Database persistance for free

Note: This one is still a proof of concept, this needs work.

```rust
// You just have to add this little attribute to your type et voila !
#[model]
struct Todo {
    title: String,
    completed: bool,
}

impl Todo {
    fn toggle(&mut self) {
        self.completed = !self.completed;

        // This will save the model in the db
        self.save();
    }
}

component! {
    Todo,
    div {
        p {
            { self.title }
            { self.completed }
            button @click: { self.toggle() } {
                { "Toggle" }
            }
        }
    }
}

// This will create a new Todo in db every time this program runs
comet!(Todo::create());
```

## Todo List
- DB
    - Register for queries
- Websocket
- Allow for iterators inside html
- Allow to mix attributes, styles and events
- Find a way for global inter-component message passing
- Allow for real time value binding for input element without losing focus (might need a real virtual dom for this one)

- Separate all the reusable features in different crates:
  - [ ] Comet crate
    - [ ] The view system
      - [ ] The html macro
      - [ ] The component macro
    - [ ] The isomorphic db model through websocket
      - [ ] The #[db] proc macro that generates basic model queries
      - [ ] An abstract ws server/client
          - [ ] The auto-proto macro
          - [X] The reactive/listening part of the db [reactive-postgres-rs](https://github.com/Champii/reactive-postgres-rs)

