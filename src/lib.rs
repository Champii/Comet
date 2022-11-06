#![recursion_limit = "256"]

#[cfg(test)]
mod tests;

pub mod core;

pub mod macros;

pub mod prelude {
    pub use crate::core::prelude::Rc;
    pub use crate::core::prelude::*;
}
use prelude::*;

#[cfg(target_arch = "wasm32")]
#[macro_use]
pub mod wasm;

#[cfg(not(target_arch = "wasm32"))]
pub mod server;

#[cfg(target_arch = "wasm32")]
pub async fn _run<Comp, Msg>(_root: Comp) -> App<Comp, Msg>
where
    Comp: Component<Msg>,
    Msg: Clone + 'static,
{
    let mut app = App::new(_root.into());

    app.run().await;

    app
}
