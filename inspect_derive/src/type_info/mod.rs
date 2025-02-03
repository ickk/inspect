mod make_static;
mod structs;

use {
  ::proc_macro::TokenStream,
  ::syn::{parse_macro_input, Data, DeriveInput, Fields},
};

pub fn derive(tokens: TokenStream) -> TokenStream {
  let ast: DeriveInput = parse_macro_input!(tokens as DeriveInput);

  let name = ast.ident;
  let _attrs = ast.attrs;
  let generics = ast.generics;

  if generics.type_params().next().is_some() {
    let ts = match &ast.data {
      Data::Struct(data_struct) => match &data_struct.fields {
        Fields::Unnamed(fields) => {
          structs::derive_generic_tuple_struct(name, generics, fields)
        },
        Fields::Named(fields) => {
          structs::derive_generic_struct(name, generics, fields)
        },
        _ => panic!("Not supported"),
      },
      _ => panic!("Not supported"),
    };
    ts.into()
  } else {
    let ts = match &ast.data {
      Data::Struct(data_struct) => match &data_struct.fields {
        Fields::Unit => structs::derive_unit_struct(name),
        Fields::Unnamed(fields) => {
          structs::derive_tuple_struct(name, generics, fields)
        },
        Fields::Named(fields) => {
          structs::derive_struct(name, generics, fields)
        },
      },
      Data::Enum(_data_enum) => {
        todo!("enums derives are not implemented yet")
      },
      _ => panic!("Not supported"),
    };
    ts.into()
  }
}
