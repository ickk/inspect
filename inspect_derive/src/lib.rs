mod type_info;

extern crate proc_macro;
use ::proc_macro::TokenStream;

#[proc_macro_derive(TypeInfo)]
pub fn derive_type_info_provider(tokens: TokenStream) -> TokenStream {
  type_info::derive(tokens)
}
