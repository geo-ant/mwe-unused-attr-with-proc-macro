use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn dummy(_args: TokenStream, input: TokenStream) -> TokenStream {
    // let func = parse_macro_input!(input as ItemFn);
    // TokenStream::from(quote!(#func))
    input
}
