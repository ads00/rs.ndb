#![feature(proc_macro)]

extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;


#[proc_macro_derive(Storable)]
pub fn storable(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_storable(&ast)
}

fn impl_storable(ast: &syn::DeriveInput) ->TokenStream  {
let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    };
    gen.into()
}

#[proc_macro_attribute]
pub fn zetatest(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    "pub fn test2() -> u32 { 99 }".parse().unwrap()
}

