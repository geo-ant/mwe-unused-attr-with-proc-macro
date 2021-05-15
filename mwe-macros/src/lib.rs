use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn dummy(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}
