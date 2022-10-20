#![feature(associated_type_defaults)]

use prelude::*;

mod app;
pub mod component;

#[macro_use]
pub mod html_macro;

pub mod prelude;

#[cfg(target_arch = "wasm32")]
#[macro_use]
pub mod wasm;

/* #[cfg(not(target_arch = "wasm32"))]
mod server; */

pub fn run<Comp, Msg>(_root: Comp)
where
    Comp: Component<Msg>,
    Msg: Clone + 'static,
{
    #[cfg(target_arch = "wasm32")]
    App::new(Rc::new(RefCell::new(Box::new(_root)))).run();
}

