#![recursion_limit = "256"]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

mod db_macro;
mod generate_hash;
mod generate_migrations;
mod generate_msg;
mod generate_proto;
mod generate_update;
mod model_macro;

mod utils;

#[proc_macro]
pub fn generate_msg(input: TokenStream) -> TokenStream {
    generate_msg::perform(input)
}

#[proc_macro]
pub fn generate_update(input: TokenStream) -> TokenStream {
    generate_update::perform(input)
}

#[proc_macro]
pub fn generate_hash(input: TokenStream) -> TokenStream {
    generate_hash::perform(input)
}

#[proc_macro_attribute]
pub fn db(_attr: TokenStream, input: TokenStream) -> TokenStream {
    db_macro::perform(input)
}

#[proc_macro]
pub fn generate_proto(input: TokenStream) -> TokenStream {
    generate_proto::perform(input)
}

#[proc_macro_attribute]
pub fn model(attr: TokenStream, input: TokenStream) -> TokenStream {
    let table_name = attr.to_string();

    model_macro::perform(table_name, input)
}

#[proc_macro]
pub fn generate_migrations(input: TokenStream) -> TokenStream {
    generate_migrations::perform(input)
}
