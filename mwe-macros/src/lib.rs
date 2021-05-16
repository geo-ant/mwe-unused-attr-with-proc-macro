use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn};
use quote::quote;

#[proc_macro_attribute]
pub fn dummy(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut func = parse_macro_input!(input as ItemFn);
    println!("function name = {}", func.sig.ident);
    for attr in func.attrs.iter() {
        println!("attr = {:?}",attr)
    }
    if func.attrs.iter().find(|attr|attr.path.segments.first().unwrap().ident == "test").is_none() {
        func.attrs.retain(|attr|attr.path.segments.first().unwrap().ident != "should_panic");
    }
    TokenStream::from(quote!(#func))
}
