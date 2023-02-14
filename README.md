# Comet

[![Documentation Status](https://readthedocs.org/projects/ansicolortags/badge/?version=latest)](https://docs.rs/comet-web/0.1.3/comet)
[![GitHub license](https://img.shields.io/github/license/Champii/Comet.svg)](https://github.com/Champii/Comet/blob/master/LICENSE.md)
[![GitHub release](https://img.shields.io/github/tag/Champii/Comet.svg)](https://GitHub.com/Champii/Comet/tags/)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square)](http://makeapullrequest.com)

Reactive isomorphic rust web framework.

## Index

  1. [Introduction](#introduction)
  2. [Features](#features)
  3. [Getting started](#getting-started)
  4. [Quick tour](#quick-tour)
  5. [Todo list](#todo-list)

---

## Introduction

Work in progress, this is still an early naive prototype.
Don't expect anything to work properly, expect things to break often.

Comet is a framework for the web build with Rust + Wasm <3. It takes its inspiration from MeteorJS, Seed-rs, Yew and others.

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

---

## Features

 - Isomorphic client/server
 - Reactive view
 - Virtual dom
 - Client cache
 - Reactive database with PostgreSQL
 - Auto database generation every time your structs change (Alpha)
 - Websocket
 - Auto procol generation
 - Remote procedure calls
 - (Almost) Zero boilerplate

---

## Getting started

### Install Comet Binary and dependencies

```bash
$> cargo install comet-cli
```

You will need to install and run an instance of PostgreSQL.

If not found on your system, Comet will install these following crates using `cargo install` on the first run:
 - `wasm-pack`
 - `diesel-cli`

### Create a simple incrementing counter 

```bash
$> comet new my_counter && cd my_counter
```

There is already the dependency setup in the Cargo.toml:

```toml
comet-web = "0.1.5"
```

This newly generated project contains all you need to get started. Your journey starts with `src/main.rs`.  
Conveniently, this generated file is already the simpliest incrementing counter you can think of:


```rust
use comet::prelude::*;

pub struct Counter {
    pub value: i32,
}

component! {
    Counter {
        button click: self.value += 1 {
            self.value 
        }
    }
}

comet::run!(Counter { value: 0 });
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

---

## Quick tour

  - [Easy definition of the dom](#easy-definition-of-the-dom)
  - [Use conditional rendering and loops](#use-conditional-rendering-and-loops)
  - [Bind your variables to `input` fields that react to events](#bind-your-variables-to-input-fields-that-react-to-events)
  - [Embed your components between them](#embed-your-components-between-them)
  - [Database persistence for free](#database-persistence-for-free)
  - [Remote procedure calls](#remote-procedure-calls)
  - [Database queries](#database-queries)

### Easy definition of the dom

```rust
use comet::prelude::*;

struct MyStruct {
    my_value: String,
    my_height: u32,
}

component! {
    MyStruct {
	// Here #my_id defined the id,
	// and the dot .class1 and .class2 add some classes to the element
	// The #id must always preceed the classes, if any
	div #my_id.class1.class2 {
	    span {
		// You can access your context anywhere
		self.my_value
	    }
	    // Define style properties
	    div style: { height: self.my_height } {
		"Another child"
	    }
	}
    }
};

```

### Use conditional rendering and loops

```rust
use comet::prelude::*;

struct MyComponent {
    show: bool,
    value: HashMap<String, i32>,
}

component! {
    MyComponent {
	div {
	    div {
		// Conditional rendering with if
		if self.show {
		    "Visible !"
		}
		button click: self.show = !self.show {
		    "Toggle"
		}
	    }
	    div {
		// Use a for-like loop.
		for (key, value) in self.value {
		    div {
			key
			value
		    }
		}
		button click: self.value.push(42)  {
		    "Add a number"
		}
	    }
	}
    }
}
```

### Bind your variables to `input` fields that react to events

This is exclusive to `input` fields for now  
Each binding should be unique, as in a different variable for each one

```rust
use comet::prelude::*;

struct MyStruct {
    value: String,
}

component! {
    MyStruct {
	div {
	    input value: self.value {}
	    self.value
	}
    }
}
```

### Embed your components between them

```rust
use comet::prelude::*;

struct Child {
    value: String,
}

component! {
    Child {
	div {
	    self.value
	}
    }
}

struct Parent {
    // You need to wrap your components with a Shared<T> that is basically a Rc<RefCell<T>>
    // This is necessary for your states to persist and be available between each render
    child: Shared<Child>,
}

component! {
    Parent {
	div {
	    // To include a component, just include it like any other variable
	    self.child
	}
    }
}
```

### Database persistence for free

All the previous examples until now were client-side only. Its time to introduce some persistance.

Deriving with the `#[model]` macro gives you access to many default DB methods implemented for your types:  
```
    - async Self::fetch(i32)  -> Result<T, String>;  
    - async Self::list()      -> Result<Vec<T>, String>;  
    - async self.save()       -> Result<(), String>;
    - async Self::delete(i32) -> Result<(), String>;
```

The `String` error type is meant to change into a real error type soon.

You have a way to add your own database query methods, please read [Database queries](#database-queries) below.

```rust
use comet::prelude::*;

// You just have to add this little attribute to your type et voila !
// It will add a field `id: i32` to the struct, for database storing purpose
// Also, when adding/changing a field to this struct, the db will 
// automatically update its schema and generate new diesel bindings
#[model]
struct Todo {
    title: String,
    completed: bool,
}

impl Todo {
    pub async fn toggle(&mut self) {
        self.completed = !self.completed;

        // This will save the model in the db
        self.save().await;
    }
}

component! {
    Todo {
	div {
	    self.id
	    self.title
	    self.completed
            button click: self.toggle().await {
               "Toggle"
	    }
	}
    }
}

// This will create a new Todo in db every time this program runs
comet::run!(Todo::default().create().await.unwrap());
```

### Remote procedure calls

Note: The structs involved in the `#[rpc]` macro MUST be accessible from the root module (i.e. `src/main.rs`)

```rust
use comet::prelude::*;

// If you have other mods that use `#[rpc]`, you have to import them explicitly
// in the root (assuming this file is the root). This is a limitation that will not last, hopefully
mod other_mod;
use other_mod::OtherComponent;

#[model]
#[derive(Default)]
pub struct Counter {
    pub count: i32,
}

// This attribute indicates that all the following methods are to be treated as RPC
// These special methods are only executed server side
// The only difference with the similar method above is that the `self.count +=1` is done server side,
// and the `self` sent back to the client
#[rpc]
impl Counter {
    // The RPC methods MUST be async (at least for now)
    pub async fn remote_increment(&mut self) {
        self.count += 1;
	
        self.save().await;
    }
}

component! {
    Counter {
	button click: self.remote_increment().await {
	    self.count
	}
    }
}

comet::run!(Counter::default().create().await.unwrap());
```

### Database queries

The most simple way to define a new database query is with the macro `#[sql]`, that uses `#[rpc]` underneath.

All your models have been augmented with auto-generated diesel bindings, so you can use a familiar syntax.
There will be a way to give raw SQL in the near future.

```rust
use comet::prelude::*;

#[model]
#[derive(Default, Debug)]
pub struct Todo {
    pub title: String,
    pub completed: bool,
}

#[sql]
impl Todo {
    // Use the watch macro to get back your data whenever the result set change in DB
    // Only valid for select statement for now
    #[watch]
    pub async fn db_get_all(limit: u16) -> Vec<Todo> {
	// The diesel schema has been generated for you
        use crate::schema::todos;

        // You don't have to actually execute the query, all the machinery
	// of creating a db connection and feeding it everywhere have been 
	// abstracted away so you can concentrate on what matters
        todos::table.select(todos::all_columns).limit(limit as i64)
    }
}
```

---

## Todo List
- Function Component
- Allow for iterators inside html
- Have a ComponentId that allows to fetch the corresponding root dom element
- Find a way for global inter-component message passing
- Allow for real time value binding for input element without losing focus (might need a real virtual dom for this one)

- Separate all the reusable features in different crates:
  - [ ] Comet crate
    - [ ] The view system
      - [ ] The html macro
      - [ ] The component macro
    - [ ] The isomorphic db model through websocket
      - [ ] The #[model] proc macro that generates basic model queries
      - [ ] An abstract ws server/client
        - [ ] The auto-proto macro
        - [X] The reactive/listening part of the db [reactive-postgres-rs](https://github.com/Champii/reactive-postgres-rs)

