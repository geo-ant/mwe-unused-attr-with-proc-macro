use proc_macro::TokenStream;
use syn::{ ItemFn, parse_macro_input};
use quote::quote;

#[proc_macro_attribute]
pub fn dummy(_args: TokenStream, input: TokenStream) -> TokenStream {
    let func = parse_macro_input!(input as ItemFn);
    TokenStream::from(quote!(#func))
}
