pub use comet_macros;
pub use paste::paste;
pub use std::{cell::RefCell, rc::Rc};
pub use web_sys;

pub use crate::core::app::*;
pub use crate::core::component::*;
pub use crate::core::shared::*;

// macros
pub use crate::_gen_full_variant;
pub use crate::_gen_variant;
pub use crate::comet;
pub use crate::component;
pub use crate::extract_msg;
pub use crate::extract_update;
pub use crate::gen_full_variant;
pub use crate::gen_variant;
pub use crate::html;
pub use crate::html_arr;
pub use crate::replace_self;

// These are safe to be in both client and server
pub use wasm_bindgen;
pub use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
pub use crate::wasm::prelude::*;

/* #[cfg(not(target_arch = "wasm32"))]
pub use crate::server::prelude::*; */
