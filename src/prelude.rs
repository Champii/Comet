pub use crate::app::*;
pub use crate::component::*;
pub use crate::element::*;
pub use crate::html;
pub use crate::html_arr;
pub use crate::renderable::*;

pub use wasm_bindgen;
pub use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
pub use crate::console_log;

#[cfg(target_arch = "wasm32")]
pub use crate::wasm::*;
