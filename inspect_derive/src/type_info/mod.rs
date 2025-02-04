mod enums;
mod make_static;
mod structs;

use {
  ::proc_macro::TokenStream,
  ::syn::{parse_macro_input, Data, DeriveInput},
};

pub fn derive(tokens: TokenStream) -> TokenStream {
  let ast: DeriveInput = parse_macro_input!(tokens as DeriveInput);

  let name = ast.ident;
  let _attrs = ast.attrs;
  let generics = ast.generics;

  let ts = match &ast.data {
    Data::Struct(data_struct) => {
      structs::derive_struct(name, generics, data_struct)
    },
    Data::Enum(data_enum) => enums::derive_enum(name, generics, data_enum),
    Data::Union(_) => panic!("Unions are not supported (yet)"),
  };

  ts.into()
}
