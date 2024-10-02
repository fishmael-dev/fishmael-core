extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{self, DeriveInput};

#[proc_macro_derive(Cacheable)]
pub fn derive_cacheable(input: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(input as DeriveInput);

    let name = derive_input.ident;

    todo!()
}