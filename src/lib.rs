#![recursion_limit = "256"]

use std::{cell::RefCell, rc::Rc};

mod app;
mod component;
mod element;

#[macro_use]
pub mod html_macro;

pub mod prelude;
mod renderable;

#[cfg(target_arch = "wasm32")]
#[macro_use]
pub mod wasm;

#[cfg(not(target_arch = "wasm32"))]
mod server;

use prelude::*;

pub fn run<Comp, Msg>(root: Comp)
where
    Comp: Component<Msg>,
    Msg: Clone + 'static,
{
    let root = Rc::new(RefCell::new(root));
    let mut app = App::new(root);
    app.run();
}
