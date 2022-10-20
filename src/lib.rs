#![feature(associated_type_defaults)]

use std::{cell::RefCell, rc::Rc};

use prelude::*;

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

/* #[cfg(not(target_arch = "wasm32"))]
mod server; */

pub fn run<Comp, Msg>(root: Comp)
where
    Comp: Component<Msg>,
    Msg: Clone + 'static,
{
    #[cfg(target_arch = "wasm32")]
    App::new(Rc::new(RefCell::new(Box::new(root)))).run();
}
