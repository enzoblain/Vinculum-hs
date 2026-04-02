extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn main(_args: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = input;

    let expanded = quote! {
        #(#attrs)*
        #vis #sig {
            vinculum_hs::runtime::init();
            #block
            vinculum_hs::runtime::shutdown();
        }
    };

    TokenStream::from(expanded)
}
