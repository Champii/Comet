pub use comet_macro_procs;
pub use derive_more::*;

pub use async_trait::async_trait;
pub use futures;
pub use lazy_static::lazy_static;

pub use crate::core::vdom;
pub use crate::core::vdom::*;

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

pub use crate::core::proto::{Message, ProtoTrait};
pub use crate::core::utils::*;

pub use crate::run;
pub use comet_macro_procs::component;
pub use comet_macro_procs::db;
pub use comet_macro_procs::generate_cache;
pub use comet_macro_procs::generate_migrations;
pub use comet_macro_procs::generate_proto;
pub use comet_macro_procs::generate_rpc_proto;
pub use comet_macro_procs::html;
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
