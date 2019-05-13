#![feature(proc_macro)]

extern crate proc_macro;
extern crate syn;
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn storable(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    "pub fn answer() -> u32 { 42 }".parse().unwrap()
}

#[proc_macro_attribute]
pub fn zetatest(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    "pub fn test2() -> u32 { 99 }".parse().unwrap()
}