extern crate proc_macro;

use proc_macro::TokenStream;

mod macro_procs;

mod utils;

#[proc_macro]
pub fn generate_msg(input: TokenStream) -> TokenStream {
    macro_procs::generate_msg::perform(input)
}

#[proc_macro]
pub fn generate_update(input: TokenStream) -> TokenStream {
    macro_procs::generate_update::perform(input)
}

#[proc_macro]
pub fn generate_hash(input: TokenStream) -> TokenStream {
    macro_procs::generate_hash::perform(input)
}
