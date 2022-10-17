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
    Comp: Component<Msg> + Clone,
    Msg: Clone,
{
    App::new(root).run();
}
