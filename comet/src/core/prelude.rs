pub use comet_macro_procs;
pub use comet_macro_rules;
pub use paste::paste;
pub use std::{cell::RefCell, rc::Rc};
pub use web_sys;

pub use crate::core::app::*;
pub use crate::core::component::*;
pub use crate::core::create_element;
pub use crate::core::shared::*;

// macros
pub use comet_macro_rules::_gen_full_variant;
pub use comet_macro_rules::_gen_variant;
pub use comet_macro_rules::comet;
pub use comet_macro_rules::component;
pub use comet_macro_rules::extract_msg;
pub use comet_macro_rules::extract_update;
pub use comet_macro_rules::gen_full_variant;
pub use comet_macro_rules::gen_variant;
pub use comet_macro_rules::html;
pub use comet_macro_rules::html_arr;
pub use comet_macro_rules::replace_self;

// These are safe to be in both client and server
pub use wasm_bindgen;
pub use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
pub use crate::wasm::prelude::*;

/* #[cfg(not(target_arch = "wasm32"))]
pub use crate::server::prelude::*; */
