extern crate proc_macro;

use proc_macro::TokenStream;

mod db_macro;
mod generate_hash;
mod generate_msg;
mod generate_proto;
mod generate_update;

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

#[proc_macro_derive(Db)]
pub fn db_macro(input: TokenStream) -> TokenStream {
    db_macro::perform(input)
}

#[proc_macro]
pub fn generate_proto(input: TokenStream) -> TokenStream {
    generate_proto::perform(input)
}
