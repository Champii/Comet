pub use comet_macro_procs;
// pub use comet_macro_rules;
pub use derive_more::*;

pub use async_trait::async_trait;
pub use lazy_static::lazy_static;

pub use diesel;
pub use diesel::prelude::*;

pub use paste::paste;
pub use serde;
pub use serde::{Deserialize, Serialize};
pub use serde_cbor;
pub use std::sync::Arc;
pub use std::{cell::RefCell, rc::Rc};
pub use tokio::sync::RwLock;
pub use web_sys;

pub use crate::core::app::*;
pub use crate::core::component::*;
pub use crate::core::shared::*;

pub use crate::core::component::run_rec;
pub use crate::core::proto::{Message, ProtoTrait};
pub use crate::core::utils::*;

// macros
pub use crate::_gen_full_variant;
pub use crate::_gen_variant;
pub use crate::component;
pub use crate::extract_bindings;
pub use crate::extract_msg;
pub use crate::extract_update;
pub use crate::gen_full_variant;
pub use crate::gen_variant;
pub use crate::html;
pub use crate::html_arr;
pub use crate::replace_self;
pub use crate::run;
pub use comet_macro_procs::db;
pub use comet_macro_procs::generate_migrations;
pub use comet_macro_procs::generate_proto;
pub use comet_macro_procs::generate_rpc_proto;
pub use comet_macro_procs::model;
pub use comet_macro_procs::rpc;
pub use comet_macro_procs::sql;
pub use comet_macro_procs::watch;

// These are safe to be in both client and server
pub use wasm_bindgen;
pub use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
pub use crate::wasm::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
pub use crate::server::prelude::*;
