pub use paste::paste;
pub use std::{cell::RefCell, rc::Rc};
pub use web_sys;

pub use crate::app::*;
pub use crate::component;
pub use crate::component::*;
pub use crate::element::*;
pub use crate::extract_msg;
pub use crate::extract_update;
pub use crate::html;
pub use crate::html_arr;
pub use crate::replace_self;
pub use crate::comet;
pub use crate::renderable::*;

// These are safe to be in both client and server
pub use wasm_bindgen;
pub use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
pub use crate::wasm::prelude::*;

/* #[cfg(not(target_arch = "wasm32"))]
pub use crate::server::prelude::*; */
